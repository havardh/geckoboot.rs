
#[repr(C)]
pub struct Init {
    enable: bool,
    debug_run = bool,
    comp0_top = bool
}

impl Init {
    pub fn default() -> Init {
        Init {
            enable: true,
            debug_run: false,
            comp0_top: true
        }
    }
}

extern {
    pub fn RTC_CompareGet(comp: u32) -> u32;
    pub fn RTC_CompareSet(comp: u32, value: u32);
    pub fn STATIC_INLINE_RTC_CounterGet();
    pub fn STATIC_INLINE_RTC_IntClear(flags: u32);
    pub fn STATIC_INLINE_RTC_IntDisable(flags: u32);
    pub fn STATIC_INLINE_RTC_IntEnable(flags: u32);
    pub fn STATIC_INLINE_RTC_IntGet() -> u32;
    pub fn STATIC_INLINE_RTC_IntSet(flags: u32);
    pub fn RTC_Reset();

    pub fn RTC_Enable(enable: bool);
    pub fn RTC_FreezeEnable(enable: bool);
    pub fn RTC_Init(init: &Init); // pub fn RTC_Init(const RTC_Init_TypeDef *init)
    pub fn RTC_CounterReset();
}

pub fn compare_get(comp: u32) -> u32 {
    unsafe { RTC_CompareGet(comp) }
}

pub fn compare_set(comp: u32, value: u32) {
    unsafe { RTC_CompareSet(comp, value) }
}

pub fn counter_get() {
    unsafe { STATIC_INLINE_RTC_CounterGet() }
}

pub fn int_clear(flags: u32) {
    unsafe { STATIC_INLINE_RTC_IntClear(flags) }
}

pub fn int_disable(flags: u32) {
    unsafe { STATIC_INLINE_RTC_IntDisable(flags) }
}

pub fn int_enable(flags: u32) {
    unsafe { STATIC_INLINE_RTC_IntEnable(flags) }
}

pub fn int_get() -> u32 {
    unsafe { STATIC_INLINE_RTC_IntGet() }
}

pub fn int_set(flags: u32) {
    unsafe { STATIC_INLINE_RTC_IntSet(flags) }
}

pub fn reset() {
    unsafe { RTC_Reset() }
}

pub fn compare_get(comp: u32) -> u32 {
    unsafe { RTC_CompareGet(comp: u32) }
}

pub fn compare_set(comp: u32, value: u32) {
    unsafe { RTC_CompareSet(comp, value) }
}

pub fn enable(enable: bool) {
    unsafe { RTC_Enable(enable) }
}

pub fn freeze_enable(enable: bool) {
    unsafe { RTC_FreezeEnable(enable) }
}

pub fn init(init: &Init) {
    unsafe { RTC_Init(init) }
}

pub fn counter_reset() {
    unsafe { RTC_CounterReset() }
}
