#[repr(C)]
pub enum Clock {
    CORE = 262176,
    HFPER = 164112,
    GPIO = 184832,
}

/** Oscillator types. */
#[repr(C)]
pub enum Osc {
  LFXO,     /**< Low frequency crystal oscillator. */
  LFRCO,    /**< Low frequency RC oscillator. */
  HFXO,     /**< High frequency crystal oscillator. */
  HFRCO,    /**< High frequency RC oscillator. */
  AUXHFRCO, /**< Auxiliary high frequency RC oscillator. */
  ULFRCO    /**< Ultra low frequency RC oscillator. */
}

/** Selectable clock sources. */
#[repr(C)]
pub enum Select {
    Error,      /**< Usage error. */
    Disabled,   /**< Clock selector disabled. */
    LFXO,       /**< Low frequency crystal oscillator. */
    LFRCO,      /**< Low frequency RC oscillator. */
    HFXO,       /**< High frequency crystal oscillator. */
    HFRCO,      /**< High frequency RC oscillator. */
    CORELEDIV2, /**< Core low energy clock divided by 2. */
    AUXHFRCO,   /**< Auxilliary clock source can be used for debug clock */
    HFCLK,      /**< Divided HFCLK on Giant for debug clock, undivided on Tiny Gecko and for USBC (not used on Gecko) */
    ULFRCO,     /**< Ultra low frequency RC oscillator. */
}

extern {
    pub fn CMU_ClockEnable(clock: Clock, enable: bool);
    pub fn CMU_ClockFreqGet(clock: Clock) -> u32;
    pub fn CMU_ClockSelectGet(clock: Clock) -> Select;
    pub fn CMU_ClockSelectSet(clock: Clock, reference: Select);
    pub fn CMU_OscillatorEnable(osc: Osc, enable: bool, wait: bool);
}

pub fn clock_enable(clock: Clock, enable: bool) {
    unsafe { CMU_ClockEnable(clock, enable) }
}

pub fn clock_freq_get(clock: Clock) -> u32 {
    unsafe { CMU_ClockFreqGet(clock) }
}

pub fn clock_select_get(clock: Clock) -> Select {
    unsafe { CMU_ClockSelectGet(clock) }
}

pub fn clock_select_get(clock: Clock, reference: Select) {
    unsafe { CMU_ClockSelectSet(clock, reference) }
}

pub fn oscillator_enable(osc: Osc, enable: bool, wait: bool) {
    unsafe { CMU_OscillatorEnable(osc, enable, wait) }
}
