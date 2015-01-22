#![no_std]

extern crate core;

use core::str::StrExt;
use core::intrinsics::transmute;

mod cmsis;

extern {
    pub fn STATIC_INLINE_CHIP_Init();
    pub fn SegmentLCD_Init(useBoost: bool);
    pub fn SegmentLCD_AllOn();
    pub fn SegmentLCD_AllOff();
    pub fn SegmentLCD_Write(string: *const u8);
}

#[no_mangle]
pub extern fn _start() {
    main()
}


fn main() {

    unsafe {
        STATIC_INLINE_CHIP_Init();

        SegmentLCD_Init(false);
        SegmentLCD_AllOff();


        let string = &[b'P' ,b'u',b's',b' ',b'<',b'3', b'\0'];
        SegmentLCD_Write(transmute(string));
    }

    loop {}
}
