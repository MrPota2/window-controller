use arduino_hal::{
    hal::port::{self, PD2, PD3},
    port::mode::Output,
};

use crate::stepper::stepper::Stepper;

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

pub fn get_arduino_stepper() -> Stepper<port::Pin<Output, PD3>, port::Pin<Output, PD2>, ArduinoDelay>
{
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let dir_pin = pins.d2.into_output();
    let step_pin = pins.d3.into_output();
    let delay = ArduinoDelay;
    Stepper::new(step_pin, dir_pin, delay)
}
