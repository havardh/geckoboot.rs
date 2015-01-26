#![no_std]

extern crate core;

use core::str::StrExt;
use core::intrinsics::transmute;

mod cmsis;

pub mod std {
  pub use core::cmp;  // used for #[derive(Eq)] until fixed in rust.
  pub use core::option;
  pub use core::num;
  pub use core::marker;
}

extern {
    pub fn STATIC_INLINE_CHIP_Init();
    pub fn SegmentLCD_Init(useBoost: bool);
    pub fn SegmentLCD_AllOn();
    pub fn SegmentLCD_AllOff();
    pub fn SegmentLCD_Write(string: *const u8);
}

#[no_mangle]
pub extern fn main() {

    unsafe {
        STATIC_INLINE_CHIP_Init();

        SegmentLCD_Init(false);
        SegmentLCD_AllOff();

        let string = &[b'R' ,b'u',b's',b't',b'y', b' ', b'G', b'e', b'c', b'k', b'o', b'\0'];
        SegmentLCD_Write(transmute(string));
    }

    loop {}
}
