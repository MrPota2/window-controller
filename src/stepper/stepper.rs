#![deny(unsafe_code)]

use embedded_hal::{delay::DelayNs, digital::OutputPin};

#[derive(Clone, Copy)]
pub enum Direction {
    Cw,
    Ccw,
}

pub struct Stepper<STEP, DIR, DELAY> {
    /// Direction of spin: -1 or 1
    pub direction: Direction,
    /// Speed in ???
    pub speed: u32,
    /// Step resolution, how small each step is
    pub resolution: f32,

    delay: DELAY,
    steps_per_revolution: u32,
    step_pin: STEP,
    direction_pin: DIR,
}

impl<STEP, DIR, DELAY, E> Stepper<STEP, DIR, DELAY>
where
    STEP: OutputPin<Error = E>,
    DIR: OutputPin<Error = E>,
    DELAY: DelayNs,
{
    pub fn new(step_pin: STEP, direction_pin: DIR, delay: DELAY) -> Self {
        let direction = Direction::Cw;
        let speed = 1;
        let resolution = 1.0;
        let steps_per_revolution = 2000;
        Self {
            direction,
            speed,
            resolution,
            step_pin,
            direction_pin,
            steps_per_revolution,
            delay,
        }
    }

    pub fn rotate(&mut self, direction: Option<Direction>) {
        self.direction = direction.unwrap_or(self.direction);
        for _ in 0..self.steps_per_revolution {
            self.step();
        }
    }

    pub fn step(&mut self) -> Result<(), STEP::Error> {
        match self.direction {
            Direction::Cw => self.direction_pin.set_low(),
            Direction::Ccw => self.direction_pin.set_high(),
        }?;

        self.step_pin.set_high()?;
        self.delay.delay_ms(self.speed.into());
        self.step_pin.set_low()?;
        self.delay.delay_ms(self.speed.into());
        Ok(())
    }

    pub fn set_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }
    pub fn set_speed(&mut self, speed: u32) {
        self.speed = speed;
    }
    pub fn set_resolution(&mut self, resolution: f32) {
        self.resolution = resolution;
    }
}
