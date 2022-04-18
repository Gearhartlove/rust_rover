/*
TODO: Study Arduino Pins : Digital + Analogue
TODO: look at analogue pin api
TODO: read https://maker.pro/arduino/tutorial/how-to-use-a-hall-effect-sensor-with-arduino

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
    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());

    // used to print messages to the console
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    let mut led = pins.d13.into_output();

    // as digital pins
    // let mut sensor1 = pins.d11;//.into_output();
    // let mut sensor2 = pins.d12;//.into_output();

    // as analogue pins
    let mut a0 = pins.a0.into_analog_input(&mut adc);
    let mut a1 = pins.a1.into_analog_input(&mut adc);
    // let mut a0 = pins.a0.into_floating_input();
    // let mut a1 = pins.a1;

    // a0.into_analog_input(&mut adc);
    // let voltage =  adc.read_nonblocking(&a0).unwrap();
    // sensor1.


    ufmt::uwriteln!(&mut serial, "hello world from arduino uno! :)").void_unwrap();
    let mut counter = 1;
    loop {

        let voltage_a0 =  adc.read_blocking(&a0) as i32;
        let voltage_a1 =  adc.read_blocking(&a1) as i32;

        ufmt::uwriteln!(&mut serial, "difference = {}", voltage_a0-voltage_a1).void_unwrap();

        // Blink the LED to indicate that everything is working properly.
        // led.toggle();
        // arduino_hal::delay_ms(500);
        // led.toggle();
        // arduino_hal::delay_ms(500);

        // let sensor1_status = sensor1.is_high();
        // let sensor2_status = sensor1.is_high();
        //
        // if sensor1_status != sensor2_status {
        //     ufmt::uwriteln!(&mut serial, "{} different sensors", counter).void_unwrap();
        //     counter += 1;
        // }

        // ufmt::uwriteln!(&mut serial, "a0: {} a1: {}", voltage_a0, voltage_a1).void_unwrap();
    }
}