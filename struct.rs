#![no_std]
#![no_main]
#![allow(unstable)]
#![feature(lang_items)]

extern crate core;

use emlib::cmu;
use emlib::gpio;

use cmsis::sys_tick;

use core::intrinsics::{volatile_load, volatile_store};

mod emlib;
mod cmsis;

pub mod std {
  pub use core::cmp;  // used for #[derive(Eq)] until fixed in rust.
  pub use core::option;
  pub use core::num;
  pub use core::marker;
}


static mut msTicks: u32 = 0;

#[no_mangle]
pub extern fn SysTick_Handler() {
    unsafe { 
        let ticks = volatile_load(&msTicks as *const u32); 
        volatile_store(&mut msTicks as *mut u32, ticks + 1);
    }
}

fn delay(dlyTicks: u32) {
    unsafe { 
        let curTicks = volatile_load(&msTicks as *const u32);
        while volatile_load(&msTicks as *const u32) - curTicks < dlyTicks {}
    }
}

const LED0: u32 = 2;
const LED1: u32 = 3;

struct Led {
    pin: u32
}

impl Led {
    pub fn init(&self) {
        gpio::pin_mode_set(gpio::Port::E, self.pin, gpio::Mode::PushPull, 0);
    }

    pub fn set(&self) {
        gpio::pin_out_set(gpio::Port::E, self.pin);
    }

    pub fn clear(&self) {
        gpio::pin_out_clear(gpio::Port::E, self.pin);
    }

    pub fn toggle(&self) {
        gpio::pin_out_toggle(gpio::Port::E, self.pin);
    }
}

fn init() {
    
    let freq = cmu::clock_freq_get(cmu::Clock::CORE);

    if sys_tick::config(freq) != 0 {
        loop {}
    }

    
    cmu::clock_enable(cmu::Clock::HFPER, true);
    cmu::clock_enable(cmu::Clock::GPIO, true);
}

#[no_mangle]
pub extern fn main() {

    init();

    let led0 = Led { pin:LED0 };
    let led1 = Led { pin:LED1 };

    led0.init();
    led1.init();

    led0.set();
    led1.clear();
    
    loop {
        led0.toggle();
        led1.toggle();

        delay(1);
    }
}


#[lang = "stack_exhausted"] extern fn stack_exhausted() {}
#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] fn panic_fmt() -> ! { loop {} }
