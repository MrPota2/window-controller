#![no_std]
#![no_main]

use core::str;

use arduino_hal::clock::MHz16;
use arduino_hal::hal::port::{PC0, PD0, PD1};
use arduino_hal::hal::usart::Usart0;
use arduino_hal::port::mode::{Analog, Input, Output};
use arduino_hal::port::Pin;
use arduino_hal::{delay_ms, Adc};
use arduino_hal::{hal::Usart, pac::USART0};
use embedded_hal::digital::{OutputPin, PinState};
use panic_halt as _;
use ufmt::uwriteln;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut adc = Adc::new(dp.ADC, Default::default());

    let led_pin = pins.d13.into_output().downgrade();
    let dir_pin = pins.d2.into_output().downgrade();
    let step_pin = pins.d3.into_output().downgrade();
    let speed_pin = pins.a0.into_analog_input(&mut adc);

    let serial = arduino_hal::default_serial!(dp, pins, 9600);

    let mut stepper = Stepper::new(adc, led_pin, dir_pin, step_pin, speed_pin, serial);

    let mut dir: bool = true;
    loop {
        let stepper_command = StepperCommand {
            rpm: 6,
            steps: 60,
            dir: dir,
        };
        stepper.rotate(stepper_command);
        dir = !dir;
    }
}

struct Stepper {
    adc: Adc,
    led_pin: Pin<Output>,
    dir_pin: Pin<Output>,
    step_pin: Pin<Output>,
    steps_per_rotation: u32,
    speed_pin: Pin<Analog, PC0>,
    serial: Usart<USART0, Pin<Input, PD0>, Pin<Output, PD1>, MHz16>,
}

impl Stepper {
    fn new(
        adc: Adc,
        led_pin: Pin<Output>,
        dir_pin: Pin<Output>,
        step_pin: Pin<Output>,
        speed_pin: Pin<Analog, PC0>,
        mut serial: Usart0<arduino_hal::DefaultClock>,
    ) -> Self {
        uwriteln!(&mut serial, "Starting up...").unwrap();

        Self {
            adc,
            led_pin,
            dir_pin,
            step_pin,
            steps_per_rotation: 60,
            speed_pin,
            serial,
        }
    }

    fn rotate(&mut self, args: StepperCommand) {
        uwriteln!(&mut self.serial, "loop: ").unwrap();
        match self.dir_pin.set_state(PinState::from(args.dir)) {
            Ok(_) => (),
            Err(err) => panic!("Problem setting stepper direction: {err:?}"),
        }

        for _ in 0..args.steps {
            let reading: u16 = self.adc.read_blocking(&mut self.speed_pin);
            let speed = (reading as u32 * 100) / 1023;
            let delay = speed; // self.delay(args.rpm);
            uwriteln!(&mut self.serial, "delay is {}", delay).unwrap();
            self.led_pin.set_high();
            self.step_pin.set_high();

            arduino_hal::delay_ms(delay);

            self.led_pin.set_low();
            self.step_pin.set_low();

            delay_ms(delay);
        }
    }

    fn delay(&self, rpm: u32) -> u32 {
        let steps_per_minute = rpm * self.steps_per_rotation;
        let milliseconds_per_step = 60 * 1000 / steps_per_minute;
        milliseconds_per_step / 2
    }

    fn log(&mut self, message: &str) {
        uwriteln!(&mut self.serial, "{}", message).unwrap();
    }
}

struct StepperCommand {
    steps: i16,
    rpm: u32,
    dir: bool,
}
