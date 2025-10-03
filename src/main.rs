#![no_std]
#![no_main]

use embedded_hal::digital;
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let steps_per_revolution = 200;

    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut dir_pin = pins.d2.into_output();
    let mut step_pin = pins.d3.into_output();

    loop {
        dir_pin.set_high();

        for _ in 0..steps_per_revolution {
            step_pin.set_high();
            arduino_hal::delay_ms(2000);
            step_pin.set_low();
            arduino_hal::delay_ms(2000);
        }

        arduino_hal::delay_ms(1000);

        dir_pin.set_low();

        for _ in 0..steps_per_revolution {
            step_pin.set_high();
            arduino_hal::delay_ms(2000);
            step_pin.set_low();
            arduino_hal::delay_ms(2000);
        }

        arduino_hal::delay_ms(1000);
    }
}
