#![no_std]
#![no_main]

use arduino_hal::port::mode::Analog;
use arduino_hal::prelude::_embedded_hal_adc_OneShot;
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let steps_per_revolution = 200;


    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut led_pin = pins.d13.into_output();
    let mut dir_pin = pins.d2.into_output();
    let mut step_pin = pins.d3.into_output();
    let speed_pin = pins.a0.into_floating_input();

    loop {
        dir_pin.set_high();

        for _ in 0..steps_per_revolution {
        led_pin.set_high();
            step_pin.set_high();
            arduino_hal::delay_ms(100);
            step_pin.set_low();
        led_pin.set_low();
            arduino_hal::delay_ms(100);
        }

        arduino_hal::delay_ms(1000);

        dir_pin.set_low();

        for _ in 0..steps_per_revolution {
            step_pin.set_high();
            arduino_hal::delay_ms(100);
            step_pin.set_low();
            arduino_hal::delay_ms(100);
        }

        arduino_hal::delay_ms(1000);
    }
}
