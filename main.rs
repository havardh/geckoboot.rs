#![no_std]
#![no_main]
#![allow(unstable)]
#![feature(lang_items)]

extern crate core;

#[no_mangle]
pub extern fn main() {
    loop {}
}

#[lang = "stack_exhausted"] extern fn stack_exhausted() {}
#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] fn panic_fmt() -> ! { loop {} }
