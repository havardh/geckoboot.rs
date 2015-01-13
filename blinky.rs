#![no_std]
#![feature(lang_items)]

extern crate core;

use emlib::{
    CMU_ClockEnable,
    CMU_Clock_TypeDef,
    CMU_ClockFreqGet,

    SysTick_Config_,

    GPIO_PinModeSet,
    GPIO_Port_TypeDef,
    GPIO_Mode_TypeDef,
    GPIO_PinOutSet_,
    GPIO_PinOutClear_,
    GPIO_PinOutToggle_,

};

use core::intrinsics::{volatile_load, volatile_store};

mod zero { pub mod zero; }
pub mod emlib;

static mut msTicks: u32 = 0;

#[no_mangle]
pub unsafe extern fn _start() {
    main()
}

#[no_mangle]
pub unsafe extern fn SysTick_Handler() {
    let ticks = volatile_load(&msTicks as *const u32);
    volatile_store(&mut msTicks as *mut u32, ticks + 1);
}

unsafe fn Delay(dlyTicks: u32) {
    let curTicks = volatile_load(&msTicks as *const u32);
    while volatile_load(&msTicks as *const u32) - curTicks < dlyTicks {}
}

#[no_mangle] 
pub unsafe extern fn main()
{
    let freq = CMU_ClockFreqGet(CMU_Clock_TypeDef::cmuClock_CORE);
    
    if SysTick_Config_(freq) != 0 {
        loop {}
    }
    
    // Setup Clocks
    CMU_ClockEnable(CMU_Clock_TypeDef::cmuClock_HFPER, true);
    CMU_ClockEnable(CMU_Clock_TypeDef::cmuClock_GPIO, true);
    
    GPIO_PinModeSet(
        GPIO_Port_TypeDef::gpioPortE, 2,
        GPIO_Mode_TypeDef::gpioModePushPull, 0
    );
    
    GPIO_PinModeSet(
        GPIO_Port_TypeDef::gpioPortE, 3,
        GPIO_Mode_TypeDef::gpioModePushPull, 0
    );

    GPIO_PinOutSet_(GPIO_Port_TypeDef::gpioPortE, 2);
    GPIO_PinOutClear_(GPIO_Port_TypeDef::gpioPortE, 3);

    // Blink loop
    loop {
        
        GPIO_PinOutToggle_(GPIO_Port_TypeDef::gpioPortE, 2);
        GPIO_PinOutToggle_(GPIO_Port_TypeDef::gpioPortE, 3);
        
        Delay(1);
    }

}
