#![no_std]
#![no_main]
#![allow(unstable)]
#![feature(lang_items)]

extern crate core;

use emlib::chip;
use emlib::cmu;
use emlib::gpio;
use emlib::timer;
use emlib::timer::Timer;
use cmsis::nvic;

use core::default::Default;

mod cmsis;
mod emlib;

pub mod std {
    pub use core::cmp;
    pub use core::option;
    pub use core::num;
    pub use core::marker;
    pub use core::default;
}

static TOP: u32 = 27342;

#[no_mangle]
#[allow(non_snake_case)]
pub extern fn TIMER0_IRQHandler() {
    let timer0 = Timer::timer0();
    timer0.int_clear(timer::TIMER_IF_OF);

    gpio::pin_out_toggle(gpio::Port::E, 2);
}

#[no_mangle]
pub extern fn main() {

    chip::init();

    cmu::clock_enable(cmu::Clock::HFPER, true);
    cmu::clock_enable(cmu::Clock::GPIO, true);
    cmu::clock_enable(cmu::Clock::TIMER0, true);

    gpio::pin_mode_set(gpio::Port::E, 2, gpio::Mode::PushPullDrive, 0);
    gpio::pin_out_clear(gpio::Port::E, 2);

    let timer_init = timer::Init {
        debug_run:    true,
        prescale:     timer::Prescale::Prescale1024,
        ..Default::default()
    };


    let timer0 = Timer::timer0();

    timer0.int_enable(timer::TIMER_IF_OF);
    nvic::enable_IRQ(nvic::IRQn::TIMER0);
    timer0.top_set(TOP);
    timer0.init(&timer_init);

    loop {}
}

#[lang = "stack_exhausted"] extern fn stack_exhausted() {}
#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] fn panic_fmt() -> ! { loop {} }
