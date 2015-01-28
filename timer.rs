#![no_std]

extern crate core;

use emlib::chip;
use emlib::cmu;
use emlib::gpio;
use emlib::timer;
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
pub extern fn TIMER0_IRQHandler() {
    timer::int_clear(timer::TIMER0, timer::TIMER_IF_OF);

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
        enable:       true,
        debug_run:    false,
        prescale:     timer::Prescale::Prescale1024,
        clk_sel:      timer::ClkSel::HFPerClk,
        count_2x:     false,
        ati:          false,
        fall_action:  timer::InputAction::None,
        rise_action:  timer::InputAction::None,
        mode:         timer::Mode::Up,
        dma_clr_act:  false,
        quad_mode_x4: false,
        one_shot:     false,
        sync:         false
    };

    timer::int_enable(timer::TIMER0, timer::TIMER_IF_OF);
    nvic::enable_IRQ(nvic::IRQn::TIMER0);
    timer::top_set(timer::TIMER0, TOP);
    timer::init(timer::TIMER0, &timer_init);

    loop {}
}
