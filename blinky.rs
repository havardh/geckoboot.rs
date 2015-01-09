#![no_std]
#![feature(lang_items)]

mod zero { pub mod zero; }

#[no_mangle]
pub extern "C" fn _start() {
    main()
}

#[no_mangle] 
pub extern "C" fn main()
{

    unsafe {
        
        let BITBAND_PER_BASE = 0x42000000 as u32;
        let PER_MEM_BASE = 0x40000000 as u32;
        
        // Setup Clocks
        let hpfer_reg = 0x400c8008;
        let hpfer_bit = 8;
        let cmu_HPFER: *mut u32 = (BITBAND_PER_BASE + ((hpfer_reg - PER_MEM_BASE) * 32) + (hpfer_bit * 4)) as *mut u32;
        *cmu_HPFER = 1 as u32;

        let gpio_reg = 0x400c8044;
        let gpio_bit = 13;
        let cmu_GPIO: *mut u32 = (BITBAND_PER_BASE + ((gpio_reg - PER_MEM_BASE) * 32) + (gpio_bit * 4)) as *mut u32;
        *cmu_GPIO = 1 as u32;
        
        // Setup LED
        let gpio_base = 0x40006000 as u64;
        let gpio_typedef_size = 9*4; // bytes
        let gpioE = gpio_base + 4 * gpio_typedef_size;
        let wordSize = 4;

        let MODEL = 1;
        let DOUTSET = 4;
        let DOUTCLR = 5;
        let DOUTTGL = 6;

        let gpioEDOUTCLR: *mut u32  = (gpioE + DOUTCLR*wordSize) as *mut u32;
        let setoutput = 0b1100 as u32;
        *gpioEDOUTCLR = setoutput;

        let pin = 3; // LED1 on stk3700 at gpio port E
        let gpioModePushPull = 0x4 as u32;

        let gpioEMODEL: *mut u32 = (gpioE + MODEL*wordSize) as *mut u32;
        let setmodel = (0xF << (pin * 4)) | (gpioModePushPull << (pin * 4)) as u32;
        *gpioEMODEL = setmodel;

        let gpioEDOUTSET: *mut u32 = (gpioE + DOUTSET*wordSize) as *mut u32;
        let setout = (1u32 << pin) as u32;
        *gpioEDOUTSET = setout;

        let gpioEDOUTTGL: *mut u32 = (gpioE + DOUTTGL*wordSize) as *mut u32;
        let settgl = (1u32 << pin) as u32;

        // Blink loop
        loop {
            let mut j = 0u;
            *gpioEDOUTTGL = settgl;
            while j < 1000000 {
                j += 1;
            }
        }
    }
}
