#![deny(unsafe_code)]

use embedded_hal::{delay::DelayNs, digital::OutputPin};

use crate::stepper::resolution::{
    get_pin_settings_from, EnableStepModeControl, MicroStepPins, NoStepModeControl, Resolution,
    ResolutionMatrix, SetStepResolution, WithStepResolutionControl,
};

#[derive(Clone, Copy)]
pub enum Direction {
    Cw,
    Ccw,
}

pub struct Stepper<STEP, DIR, DELAY, MODE = NoStepModeControl> {
    /// Direction of spin: -1 or 1
    pub direction: Direction,
    /// Speed in rms
    pub speed_rpm: u32,
    /// Step resolution, how small each step is
    pub resolution: Resolution,

    delay: DELAY,
    min_delay_us: u32,
    steps_per_revolution: u32,
    step_pin: STEP,
    direction_pin: DIR,

    mode: MODE,
}

impl<STEP, DIR, DELAY, E> Stepper<STEP, DIR, DELAY, NoStepModeControl>
where
    STEP: OutputPin<Error = E>,
    DIR: OutputPin<Error = E>,
    DELAY: DelayNs,
{
    pub fn new(step_pin: STEP, direction_pin: DIR, delay: DELAY) -> Self {
        let direction = Direction::Cw;
        let speed = 120;
        let resolution = Resolution::FULL;
        let steps_per_revolution = 200;
        let min_delay_us = 50;
        Self {
            direction,
            speed_rpm: speed,
            resolution,
            step_pin,
            direction_pin,
            steps_per_revolution,
            delay,
            min_delay_us,
            mode: NoStepModeControl,
        }
    }
}

impl<STEP, DIR, DELAY, E, MS1, MS2, MS3> EnableStepModeControl<MicroStepPins<MS1, MS2, MS3>, E>
    for Stepper<STEP, DIR, DELAY, NoStepModeControl>
where
    STEP: OutputPin<Error = E>,
    DIR: OutputPin<Error = E>,
    DELAY: DelayNs,
    MS1: OutputPin<Error = E>,
    MS2: OutputPin<Error = E>,
    MS3: OutputPin<Error = E>,
{
    type WithStepModeControl = Stepper<STEP, DIR, DELAY, WithStepResolutionControl<MS1, MS2, MS3>>;

    fn enable_step_mode_control(
        self,
        res: MicroStepPins<MS1, MS2, MS3>,
    ) -> Self::WithStepModeControl {
        Stepper {
            direction: self.direction,
            speed_rpm: self.speed_rpm,
            resolution: self.resolution,
            delay: self.delay,
            steps_per_revolution: self.steps_per_revolution,
            step_pin: self.step_pin,
            direction_pin: self.direction_pin,
            min_delay_us: self.min_delay_us,
            mode: WithStepResolutionControl { pins: res },
        }
    }
}

impl<STEP, DIR, DELAY, E, MS1, MS2, MS3> SetStepResolution<E>
    for Stepper<STEP, DIR, DELAY, WithStepResolutionControl<MS1, MS2, MS3>>
where
    MS1: OutputPin<Error = E>,
    MS2: OutputPin<Error = E>,
    MS3: OutputPin<Error = E>,
{
    fn set_step_resolution(&mut self, resolution: Resolution) -> Result<(), E> {
        self.resolution = resolution;
        let ResolutionMatrix { ms1, ms2, ms3 } = get_pin_settings_from(&self.resolution);
        match ms1 {
            super::resolution::SIGNAL::HIGH => self.mode.pins.ms1_pin.set_high()?,
            super::resolution::SIGNAL::LOW => self.mode.pins.ms1_pin.set_low()?,
        }
        match ms2 {
            super::resolution::SIGNAL::HIGH => self.mode.pins.ms2_pin.set_high()?,
            super::resolution::SIGNAL::LOW => self.mode.pins.ms2_pin.set_low()?,
        }
        match ms3 {
            super::resolution::SIGNAL::HIGH => self.mode.pins.ms3_pin.set_high()?,
            super::resolution::SIGNAL::LOW => self.mode.pins.ms3_pin.set_low()?,
        }

        Ok(())
    }
}

impl<STEP, DIR, DELAY, MODE, E> Stepper<STEP, DIR, DELAY, MODE>
where
    STEP: OutputPin<Error = E>,
    DIR: OutputPin<Error = E>,
    DELAY: DelayNs,
{
    pub fn rotate(&mut self, rotations: u32) -> Result<(), STEP::Error> {
        let steps = self.steps_for_rotations(rotations);
        for _ in 0..steps {
            self.step(self.delay(steps))?;
        }

        Ok(())
    }

    pub fn step(&mut self, delay: u32) -> Result<(), STEP::Error> {
        match self.direction {
            Direction::Cw => self.direction_pin.set_low(),
            Direction::Ccw => self.direction_pin.set_high(),
        }?;

        self.step_pin.set_high()?;
        self.delay.delay_us(delay);
        self.step_pin.set_low()?;
        self.delay.delay_us(delay);
        Ok(())
    }

    pub fn set_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }
    pub fn set_speed(&mut self, speed: u32) {
        self.speed_rpm = speed;
    }
    pub fn set_resolution(&mut self, resolution: Resolution) {
        self.resolution = resolution;
    }
    fn steps_for_rotations(&mut self, rotations: u32) -> u32 {
        let micro = match self.resolution {
            Resolution::FULL => 1,
            Resolution::HALF => 2,
            Resolution::FOURTH => 4,
            Resolution::EIGTH => 8,
            Resolution::SIXTEENTH => 16,
        };
        rotations * self.steps_per_revolution * micro
    }
    fn delay(&self, steps: u32) -> u32 {
        let min_delay = 50;
        let requested_delay_microseconds = 60000000 / steps / 2 / self.speed_rpm;
        if requested_delay_microseconds < min_delay {
            min_delay
        } else {
            requested_delay_microseconds
        }
    }
}
