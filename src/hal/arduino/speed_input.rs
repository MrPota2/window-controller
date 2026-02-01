#![deny(unsafe_code)]

use core::convert::Infallible;

use arduino_hal::{
    port::{self, mode::Analog, A0},
    Adc,
};

use crate::stepper::speed::SpeedSource;

pub trait GetSpeed {
    fn get_speed(&mut self) -> u16;
}

pub struct WithStepResolutionControl;

pub struct AnalogSpeed {
    adc: Adc,
    pin: port::Pin<Analog, A0>,
    min_rpm: u32,
    max_rpm: u32,
}

impl AnalogSpeed {
    pub fn new(adc: Adc, pin: port::Pin<Analog, A0>, min_rpm: u32, max_rpm: u32) -> Self {
        Self {
            adc,
            pin,
            min_rpm,
            max_rpm,
        }
    }

    pub fn read_raw(&mut self) -> u16 {
        self.pin.analog_read(&mut self.adc)
    }
    fn map_to_rpm(&self, raw: u16) -> u32 {
        let raw = raw as u32;
        let max_raw = 1023u32; // Uno is 10-bit
        self.min_rpm + (raw * (self.max_rpm - self.min_rpm)) / max_raw
    }
}

impl SpeedSource for AnalogSpeed {
    type Error = Infallible;

    fn speed_rpm(&mut self) -> Result<u32, Self::Error> {
        let raw = self.read_raw();
        Ok(self.map_to_rpm(raw))
    }
}
