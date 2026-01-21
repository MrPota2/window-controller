#![no_std]
#![no_main]

use panic_halt as _;

use crate::{hal::arduino::get_arduino_stepper, stepper::stepper::Direction};

pub mod hal;
pub mod stepper;

#[arduino_hal::entry]
fn main() -> ! {
    let steps_per_revolution = 200;

    let mut stepper = get_arduino_stepper();

    loop {
        stepper.set_direction(Direction::Cw);
        for _ in 0..steps_per_revolution {
            stepper.step().unwrap();
        }

        arduino_hal::delay_ms(1000);

        stepper.set_direction(Direction::Ccw);

        for _ in 0..steps_per_revolution {
            stepper.step().unwrap()
        }

        arduino_hal::delay_ms(1000);
    }
}
