#[repr(C)]
pub enum Clock {
    CORE = 262176,
    HFPER = 164112,
    TIMER0 = 152064,
    GPIO = 184832,
}

extern {
    pub fn CMU_ClockEnable(clock: Clock, enable: bool);
    pub fn CMU_ClockFreqGet(clock: Clock) -> u32;
}

pub fn clock_enable(clock: Clock, enable: bool) {
    unsafe { CMU_ClockEnable(clock, enable) }
}

pub fn clock_freq_get(clock: Clock) -> u32 {
    unsafe { CMU_ClockFreqGet(clock) }
}
