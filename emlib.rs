#[repr(C)]
pub enum CMU_Clock_TypeDef {
    cmuClock_CORE = 262176,
    cmuClock_HFPER = 164112,
    cmuClock_GPIO = 184832
}

extern {
    pub fn CMU_ClockEnable(clock: CMU_Clock_TypeDef, enable: bool);
    pub fn CMU_ClockFreqGet(clock: CMU_Clock_TypeDef) -> u32;
}

extern {
    pub fn SysTick_Config_(ticks: u32) -> u32;
}

#[repr(C)]
pub enum GPIO_Port_TypeDef {
  gpioPortA = 0,
  gpioPortB = 1,
  gpioPortC = 2,
  gpioPortD = 3,
  gpioPortE = 4,
  gpioPortF = 5,
}

pub enum GPIO_Mode_TypeDef {
    gpioModePushPull = 0x00000004,
}

extern {
    pub fn GPIO_PinModeSet(
        port: GPIO_Port_TypeDef,
        pins: u32,
        mode: GPIO_Mode_TypeDef,
        out: u32
    );
    pub fn GPIO_PinOutSet_(port: GPIO_Port_TypeDef, pins: u32);
    pub fn GPIO_PinOutClear_(port: GPIO_Port_TypeDef, pins: u32);
    pub fn GPIO_PinOutToggle_(port: GPIO_Port_TypeDef, pins: u32);
}
