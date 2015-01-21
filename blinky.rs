#![no_std]

extern crate core;

use emlib::cmu;
use emlib::gpio;

use cmsis::sys_tick;

use core::intrinsics::{volatile_load, volatile_store};

mod emlib;
mod cmsis;

static mut msTicks: u32 = 0;

#[no_mangle] pub extern fn _start() { main() }

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

fn main()
{
    let freq = cmu::clock_freq_get(cmu::Clock::CORE);

    if sys_tick::config(freq) != 0 {
        loop {}
    }

    cmu::clock_enable(cmu::Clock::HFPER, true);
    cmu::clock_enable(cmu::Clock::GPIO, true);
    
    gpio::pin_mode_set(gpio::Port::E, LED0, gpio::Mode::PushPull, 0);
    gpio::pin_mode_set(gpio::Port::E, LED1, gpio::Mode::PushPull, 0);

    gpio::pin_out_set(gpio::Port::E, LED0);
    gpio::pin_out_clear(gpio::Port::E, LED1);

    loop {
        gpio::pin_out_toggle(gpio::Port::E, LED0);
        gpio::pin_out_toggle(gpio::Port::E, LED1);

        delay(1);
    }
}
