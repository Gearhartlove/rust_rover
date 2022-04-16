/*
TODO: Learn how motor control connects to Arduino
TODO: Study Arduino Pins : Digital + Analogue
TODO: Learn where servo wires go on arduino / moto controller

NOTE: hall sensors aren't changing with the digital output
NOTE: sensors are turning to true at the same time . . .
    > NVM, this is not the case, just happens really fast is all
 */

#![no_std]
#![no_main]

use panic_halt as _;
use ufmt::uWrite;
use core::{convert::Infallible, fmt, ptr};
use core::fmt::Write;
use ufmt::derive::uDebug;
use arduino_hal::prelude::*;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    /*
     * For examples (and inspiration), head to
     *
     *     https://github.com/Rahix/avr-hal/tree/main/examples
     *
     * NOTE: Not all examples were ported to all boards!  There is a good chance though, that code
     * for a different board can be adapted for yours.  The Arduino Uno currently has the most
     * examples available.
     */

    // let mut w = Writer {};
    // Important because this sets the bit in the DDR register!
    //pins.d9.into_output();
    // let timer = dp.TC1;
    let mut led = pins.d13.into_output();

    let mut sensor1 = pins.d11;//.into_output();
    let mut sensor2 = pins.d12;//.into_output();


    // uWrite::write_str(&mut w, "hello");
    // ufmt::uwriteln!(&mut w, "test").unwrap();
    // assert_eq!("tdest", "test");

    ufmt::uwriteln!(&mut serial, "hello world from arduino uno! :)").void_unwrap();
    let mut counter = 1;
    loop {
        // Blink the LED to indicate that everything is working properly.
        // led.toggle();
        // arduino_hal::delay_ms(500);
        // led.toggle();
        // arduino_hal::delay_ms(500);

        let sensor1_status = sensor1.is_high();
        let sensor2_status = sensor1.is_high();

        if sensor1_status != sensor2_status {
            ufmt::uwriteln!(&mut serial, "{} different sensors", counter).void_unwrap();
            counter += 1;
        }
        // ufmt::uwriteln!(&mut serial, "hello world from arduino uno! :)").void_unwrap();
        // print the output of the hall sensors to the console
        // ufmt::uwriteln!(&mut serial, "{} sensor1:{} sensor2:{}", counter, sensor1_status, sensor2_status).void_unwrap();
    }
}