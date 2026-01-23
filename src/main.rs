#![no_std]
#![no_main]

use panic_halt as _;

use crate::stepper::resolution::Resolution;
use crate::{
    hal::arduino::get_arduino_stepper,
    stepper::{resolution::SetStepResolution, stepper::Direction},
};

pub mod hal;
pub mod stepper;

#[arduino_hal::entry]
fn main() -> ! {
    let mut stepperino = get_arduino_stepper();

    #[cfg(feature = "full")]
    stepperino.set_step_resolution(stepper::resolution::Resolution::FULL);

    #[cfg(feature = "half")]
    stepperino.set_step_resolution(stepper::resolution::Resolution::HALF);

    #[cfg(feature = "fourth")]
    stepperino.set_step_resolution(stepper::resolution::Resolution::FOURTH);

    #[cfg(feature = "sixteenth")]
    stepperino.set_step_resolution(stepper::resolution::Resolution::SIXTEENTH);

    loop {
        stepperino.set_direction(Direction::Cw);
        //        for _ in 0..steps_per_revolution {
        //          stepperino.step().unwrap();
        //    }

        arduino_hal::delay_ms(1000);

        stepperino.set_direction(Direction::Ccw);

        stepperino.rotate(2);

        arduino_hal::delay_ms(1000);
    }
}
