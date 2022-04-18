/*
TODO: setup bluetooth between arduino and <phone>
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

    // N W S E functionality
    loop {
        let direction = GetBluetoothInput();
        match direction.unwrap() {
            Direction::North => {moveNorth()}
            Direction::West => {moveWest()}
            Direction::South => {moveSouth()}
            Direction::East => {moveEast()}
        }
    }
}

fn moveNorth() {}
fn moveWest() {}
fn moveSouth() {}
fn moveEast() {}

fn GetBluetoothInput() -> Option<Direction> {
    // listen for bluetooth input
    // handle inputs based on Direction enum and return that value
    None
}

enum Direction {
    North,
    West,
    South,
    East,
}