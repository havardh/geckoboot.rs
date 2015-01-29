#![no_std]
#![allow(unstable)]

extern crate core;

use cmsis::nvic;

use emlib::cmu;
use emlib::chip;
use emlib::gpio;
use emlib::rtc;

mod emlib;
mod cmsis;

pub mod std {
    pub use core::cmp;
    pub use core::option;
    pub use core::num;
    pub use core::marker;
    pub use core::default;
}

const LFXO_FREQ: u32 = 32768;
const RTC_TIMEOUT_S: u32 = 2;

fn rtc_setup() {
    // let rtc_init = rtc::Init { enable: false, .. rtc::Init::default() };
    let rtc_init = rtc::Init { enable: false, debug_run: false, comp0_top: true };

    /* Enable LE domain registers */
    cmu::clock_enable(cmu::Clock::CORELE, true);

    /* Enable LFXO as LFACLK in CMU. This will also start LFXO */
    cmu::clock_select_set(cmu::Clock::LFA, cmu::Select::LFXO);

    /* Enable RTC clock */
    cmu::clock_enable(cmu::Clock::RTC, true);

    rtc::init(&rtc_init);

    /* Interrupt every second */
    rtc::compare_set(0, LFXO_FREQ * RTC_TIMEOUT_S);

    /* Enable interrupt */
    nvic::enable_IRQ(nvic::IRQn::RTC);
    rtc::int_enable(rtc::RTC_IEN_COMP0);

    /* Start Counter */
    rtc::enable(true);
}

fn gpio_setup() {
    cmu::clock_enable(cmu::Clock::GPIO, true);

    gpio::pin_mode_set(gpio::Port::E, 2, gpio::Mode::PushPullDrive, 0);
    gpio::pin_out_clear(gpio::Port::E, 2);
}

#[no_mangle]
pub extern fn main() {

    chip::init();

    rtc_setup();

    gpio_setup();

    loop { }
}

#[no_mangle]
pub extern fn RTC_IRQHandler() {
    /* Clear interrupt source */
    rtc::int_clear(rtc::RTC_IEN_COMP0);

    gpio::pin_out_toggle(gpio::Port::E, 2);
}
