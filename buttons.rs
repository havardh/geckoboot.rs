#![no_std]

extern crate core;

use emlib::cmu;
use emlib::gpio;

mod emlib;

#[no_mangle] pub extern fn _start() { main() }

const LED0: u32 = 2;
const LED1: u32 = 3;

const PB0: u32 = 9;
const PB1: u32 = 10;


fn main() {

    cmu::clock_enable(cmu::Clock::HFPER, true);
    cmu::clock_enable(cmu::Clock::GPIO, true);

    gpio::pin_mode_set(gpio::Port::E, LED0, gpio::Mode::PushPull, 0);
    gpio::pin_mode_set(gpio::Port::E, LED1, gpio::Mode::PushPull, 0);

    gpio::pin_mode_set(gpio::Port::B, PB0, gpio::Mode::Input, 0);
    gpio::pin_mode_set(gpio::Port::B, PB1, gpio::Mode::Input, 0);

    gpio::pin_out_clear(gpio::Port::E, LED0);
    gpio::pin_out_clear(gpio::Port::E, LED1);

    loop {

        let pb0 = gpio::pin_in_get(gpio::Port::B, PB0);
        let pb1 = gpio::pin_in_get(gpio::Port::B, PB1);

        if pb0 == 0 {
            gpio::pin_out_set(gpio::Port::E, LED0);
        } else {
            gpio::pin_out_clear(gpio::Port::E, LED0);
        }

        if pb1 == 0 {
            gpio::pin_out_set(gpio::Port::E, LED1);
        } else {
            gpio::pin_out_clear(gpio::Port::E, LED1);
        }
    }
}
