use arduino_hal::{
    hal::port::{self, PD2, PD3, PD4, PD5, PD6},
    port::mode::Output,
    Adc,
};

use crate::{
    hal::arduino::speed_input::AnalogSpeed,
    stepper::{
        resolution::{EnableStepModeControl, MicroStepPins, WithStepResolutionControl},
        stepper::Stepper,
    },
};

pub struct ArduinoDelay;

impl embedded_hal::delay::DelayNs for ArduinoDelay {
    fn delay_ns(&mut self, ns: u32) {
        arduino_hal::delay_ns(ns);
    }
    fn delay_us(&mut self, us: u32) {
        arduino_hal::delay_us(us);
    }
    fn delay_ms(&mut self, ms: u32) {
        arduino_hal::delay_ms(ms);
    }
}

pub fn get_arduino_stepper() -> Stepper<
    port::Pin<Output, PD3>,
    port::Pin<Output, PD2>,
    ArduinoDelay,
    AnalogSpeed,
    WithStepResolutionControl<
        port::Pin<Output, PD4>,
        port::Pin<Output, PD5>,
        port::Pin<Output, PD6>,
    >,
> {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let dir_pin = pins.d2.into_output();
    let step_pin = pins.d3.into_output();
    let ms1_pin = pins.d4.into_output();
    let ms2_pin = pins.d5.into_output();
    let ms3_pin = pins.d6.into_output();
    let mut pot_value = Adc::new(dp.ADC, Default::default());
    let pot_pin = pins.a0.into_analog_input(&mut pot_value);
    let speed_control = AnalogSpeed::new(pot_value, pot_pin, 1, 60);
    let delay = ArduinoDelay;
    let stepper = Stepper::new(step_pin, dir_pin, delay, speed_control);
    Stepper::enable_step_mode_control(
        stepper,
        MicroStepPins {
            ms1_pin,
            ms2_pin,
            ms3_pin,
        },
    )
}
