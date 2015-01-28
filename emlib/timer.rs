use core::intrinsics::transmute;
use core::default::Default;

#[repr(C)]
pub struct CC {
    CTRL: u32,
    CCV: u32,
    CCVP: u32,
    CCVN: u32,
}

#[repr(C)]
pub struct Timer {
    CTRL: u32,
    CMD: u32,
    STATUS: u32,
    IEN: u32,
    IF: u32,
    IFS: u32,
    IFC: u32,
    TOP: u32,
    TOPB: u32,
    CNT: u32,
    ROUTE: u32,
    RESERVED0: u32,
    CC: [CC; 3],
    RESERVED1: [u32; 4],
    DTCTRL: u32,
    DTTIME: u32,
    DTFC: u32,
    DTOGEN: u32,
    DTFAULT: u32,
    DTFAULTC: u32,
    DTLOCK: u32,
}

pub enum Idx {
    Timer0,
    Timer1,
    Timer2,
    Timer3
}

impl Timer {
    pub fn new(idx: Idx) -> &'static Timer {
        unsafe {
            match idx {
                Idx::Timer0 => transmute(0x40010000),
                Idx::Timer1 => transmute(0x40010400),
                Idx::Timer2 => transmute(0x40010800),
                Idx::Timer3 => transmute(0x40010C00)
            }
        }
    }

}

#[repr(u8)]
#[derive(Copy)]
pub enum CCMode {
    Off = 0x0,
    Capture = 0x1,
    Compare = 0x2,
    PWM = 0x3,
}

#[repr(u8)]
#[derive(Copy)]
pub enum ClkSel {
    HFPerClk = 0x0,
    CC1 = 0x1,
    Cascade = 0x2,
}

#[repr(u8)]
#[derive(Copy)]
pub enum Edge {
    Rising = 0x0,
    Faling = 0x1,
    Both = 0x2,
    None = 0x3,
}

#[repr(u8)]
#[derive(Copy)]
pub enum Event {
    EveryEdge = 0x0,
    Every2ndEdge = 0x1,
    Rising = 0x2,
    Falling = 0x3,
}

#[repr(u8)]
#[derive(Copy)]
pub enum InputAction {
    None = 0x0,
    Start = 0x1,
    Stop = 0x2,
    ReloadStart = 0x3,
}

#[repr(u8)]
#[derive(Copy)]
pub enum Mode {
    Up = 0x0,
    Down = 0x1,
    UpDown = 0x2,
    QDec = 0x3,
}

#[repr(u8)]
#[derive(Copy)]
pub enum OutputAction {
    None = 0x0,
    Toggle = 0x1,
    Clear = 0x2,
    Set = 0x3,
}

#[repr(u8)]
#[derive(Copy)]
pub enum Prescale {
    Prescale1    = 0x0,
    Prescale2    = 0x1,
    Prescale4    = 0x2,
    Prescale8    = 0x3,
    Prescale16   = 0x4,
    Prescale32   = 0x5,
    Prescale64   = 0x6,
    Prescale128  = 0x7,
    Prescale256  = 0x8,
    Prescale512  = 0x9,
    Prescale1024 = 0xA,
}

#[repr(u8)]
#[derive(Copy)]
pub enum PRSSEL {
    Ch0 = 0x0,
    Ch1 = 0x1,
    Ch2 = 0x2,
    Ch3 = 0x3,
    Ch4 = 0x4,
    Ch5 = 0x5,
    Ch6 = 0x6,
    Ch7 = 0x7,
    Ch8  = 0x8,
    Ch9  = 0x9,
    Ch10 = 0xA,
    Ch11 = 0xB,
}

#[repr(u8)]
#[derive(Copy)]
pub enum DtiFaultAction {
    None = 0x0,
    Inactive = 0x1,
    Clear = 0x2,
    Tristate = 0x3,
}

#[repr(C)]
#[derive(Copy)]
pub struct Init {
    pub enable: bool,
    pub debug_run: bool,
    pub prescale: Prescale,
    pub clk_sel: ClkSel,
    pub count_2x: bool,
    pub ati: bool,
    pub fall_action: InputAction,
    pub rise_action: InputAction,
    pub mode: Mode,
    pub dma_clr_act: bool,
    pub quad_mode_x4: bool,
    pub one_shot: bool,
    pub sync: bool,
}

impl Default for Init {
    fn default() -> Init {
        Init {
            enable:       true,
            debug_run:    false,
            prescale:     Prescale::Prescale1,
            clk_sel:      ClkSel::HFPerClk,
            count_2x:     false,
            ati:          false,
            fall_action:  InputAction::None,
            rise_action:  InputAction::None,
            mode:         Mode::Up,
            dma_clr_act:  false,
            quad_mode_x4: false,
            one_shot:     false,
            sync:         false
        }
    }
}

#[repr(C)]
#[derive(Copy)]
pub struct InitCC {
    event_ctrl: Event,
    edge: Edge,
    prs_sel: PRSSEL,
    cufoa: OutputAction,
    cofoa: OutputAction,
    cmoa: OutputAction,
    mode: CCMode,
    filter: bool,
    prs_input: bool,
    coist: bool,
    out_invert: bool,
}

impl Default for InitCC {
    fn default() -> InitCC {
        InitCC {
            event_ctrl: Event::EveryEdge,
            edge:       Edge::Rising,
            prs_sel:    PRSSEL::Ch0,
            cufoa:      OutputAction::None,
            cofoa:      OutputAction::None,
            cmoa:       OutputAction::None,
            mode:       CCMode::Off,
            filter:     false,
            prs_input:  false,
            coist:      false,
            out_invert: false,
        }
    }
}

#[repr(C)]
#[derive(Copy)]
pub struct InitDTI {
    enable: bool,
    active_low_out: bool,
    invert_complementary_out: bool,
    auto_restart: bool,
    enable_prs_source: bool,
    prs_sel: PRSSEL,
    prescale: Prescale,
    rise_time: u32,
    fall_time: u32,
    outputs_enable_mask: u32,
    enable_fault_source_core_lockup: bool,
    enable_fault_source_debugger: bool,
    enable_fault_source_prs_sel_0: bool,
    fault_source_prs_sel_0: PRSSEL,
    enable_fault_source_prs_sel_1: bool,
    fault_source_prs_sel_1: PRSSEL,
    fault_action: DtiFaultAction,
}

impl Default for InitDTI {
    fn default() -> InitDTI {
        InitDTI {
            enable:                          true,
            active_low_out:                  false,
            invert_complementary_out:        false,
            auto_restart:                    false,
            enable_prs_source:               false,
            prs_sel:                         PRSSEL::Ch0,
            prescale:                        Prescale::Prescale1,
            rise_time:                       0,
            fall_time:                       0,
            outputs_enable_mask:             (0x1 << 0 | 0x1<<3),
            enable_fault_source_core_lockup: true,
            enable_fault_source_debugger:    true,
            enable_fault_source_prs_sel_0:   false,
            fault_source_prs_sel_0:          PRSSEL::Ch0,
            enable_fault_source_prs_sel_1:   false,
            fault_source_prs_sel_1:          PRSSEL::Ch0,
            fault_action:                    DtiFaultAction::Inactive,
        }
    }
}

extern {

    pub fn STATIC_INLINE_TIMER_CaptureGet(timer: *mut Timer, ch: u32) -> u32;
    pub fn STATIC_INLINE_TIMER_CompareBufSet(timer: *mut Timer, ch: u32, val: u32);
    pub fn STATIC_INLINE_TIMER_CompareSet(timer: *mut Timer, ch: u32, val: u32);
    pub fn STATIC_INLINE_TIMER_CounterGet(timer: *mut Timer) -> u32;
    pub fn STATIC_INLINE_TIMER_CounterSet(timer: *mut Timer, val: u32);
    pub fn STATIC_INLINE_TIMER_Enable(timer: *mut Timer, enable: bool);
    pub fn TIMER_Init(timer: *mut Timer, init: *const Init);
    pub fn TIMER_InitCC(timer: *mut Timer, ch: u32, init: *const InitCC);
    pub fn TIMER_InitDTI(timer: *mut Timer, init: *const InitDTI);
    pub fn STATIC_INLINE_TIMER_EnableDTI(timer: *mut Timer, enable: bool);
    pub fn STATIC_INLINE_TIMER_GetDTIFault(timer: *mut Timer) -> u32;
    pub fn STATIC_INLINE_TIMER_ClearDTIFault(timer: *mut Timer, flags: u32);
    pub fn STATIC_INLINE_TIMER_IntClear(timer: *mut Timer, flags: u32);
    pub fn STATIC_INLINE_TIMER_IntDisable(timer: *mut Timer, flags: u32);
    pub fn STATIC_INLINE_TIMER_IntEnable(timer: *mut Timer, flags: u32);
    pub fn STATIC_INLINE_TIMER_IntGet(timer: *mut Timer) -> u32;
    pub fn STATIC_INLINE_TIMER_IntGetEnabled(timer: *mut Timer) -> u32;
    pub fn STATIC_INLINE_TIMER_IntSet(timer: *mut Timer, flags: u32);
    pub fn STATIC_INLINE_TIMER_Lock(timer: *mut Timer);
    pub fn TIMER_Reset(timer: *mut Timer);
    pub fn STATIC_INLINE_TIMER_TopBufSet(timer: *mut Timer, val: u32);
    pub fn STATIC_INLINE_TIMER_TopGet(timer: *mut Timer) -> u32;
    pub fn STATIC_INLINE_TIMER_TopSet(timer: *mut Timer, val: u32);
    pub fn STATIC_INLINE_TIMER_Unlock(timer: *mut Timer);
    
}

pub fn capture_get(timer: &Timer, ch: u32) -> u32 {
    unsafe { STATIC_INLINE_TIMER_CaptureGet(transmute(timer), ch) }
}

pub fn compare_buf_set(timer: &Timer, ch: u32, val: u32) {
    unsafe { STATIC_INLINE_TIMER_CompareBufSet(transmute(timer), ch, val) }
}

pub fn compare_set(timer: &Timer, ch: u32, val: u32) {
    unsafe { STATIC_INLINE_TIMER_CompareSet(transmute(timer), ch, val) }
}

pub fn counter_get(timer: &Timer) -> u32 {
    unsafe { STATIC_INLINE_TIMER_CounterGet(transmute(timer)) }
}

pub fn counter_set(timer: &Timer, val: u32) {
    unsafe { STATIC_INLINE_TIMER_CounterSet(transmute(timer), val) }
}

pub fn enable(timer: &Timer, enable: bool) {
    unsafe { STATIC_INLINE_TIMER_Enable(transmute(timer), enable) }
}

pub fn init(timer: &Timer, init: &Init) {
    unsafe { TIMER_Init(transmute(timer), transmute(init)) }
}

pub fn init_cc(timer: &Timer, ch: u32, init: InitCC) {
    unsafe { TIMER_InitCC(transmute(timer), ch, transmute(&init)) }
}

pub fn init_dti(timer: &Timer, init: InitDTI) {
    unsafe { TIMER_InitDTI(transmute(timer), transmute(&init)) }
}

pub fn enable_dti(timer: &Timer, enable: bool) {
    unsafe { STATIC_INLINE_TIMER_EnableDTI(transmute(timer), enable) }
}

pub fn get_dti_fault(timer: &Timer) -> u32 {
    unsafe { STATIC_INLINE_TIMER_GetDTIFault(transmute(timer)) }
}

pub fn clear_dti_fault(timer: &Timer, flags: u32) {
    unsafe { STATIC_INLINE_TIMER_ClearDTIFault(transmute(timer), flags) }
}

pub fn int_clear(timer: &Timer, flags: u32) {
    unsafe { STATIC_INLINE_TIMER_IntClear(transmute(timer), flags) }
}

pub fn int_disable(timer: &Timer, flags: u32) {
    unsafe { STATIC_INLINE_TIMER_IntDisable(transmute(timer), flags) }
}

pub fn int_enable(timer: &Timer, flags: u32) {
    unsafe { STATIC_INLINE_TIMER_IntEnable(transmute(timer), flags)}
}

pub fn int_get(timer: &Timer) -> u32 {
    unsafe { STATIC_INLINE_TIMER_IntGet(transmute(timer)) }
}

pub fn int_get_enabled(timer: &Timer) -> u32 {
    unsafe { STATIC_INLINE_TIMER_IntGetEnabled(transmute(timer)) }
}

pub fn int_set(timer: &Timer, flags: u32) {
    unsafe { STATIC_INLINE_TIMER_IntSet(transmute(timer), flags) }
}

pub fn lock(timer: &Timer) {
    unsafe { STATIC_INLINE_TIMER_Lock(transmute(timer)) }
}

pub fn reset(timer: &Timer) {
    unsafe { TIMER_Reset(transmute(timer)) }
}

pub fn top_buf_set(timer: &Timer, val: u32) {
    unsafe { STATIC_INLINE_TIMER_TopBufSet(transmute(timer), val) }
}

pub fn top_get(timer: &Timer) -> u32 {
    unsafe { STATIC_INLINE_TIMER_TopGet(transmute(timer)) }
}

pub fn top_set(timer: &Timer, val: u32) {
    unsafe { STATIC_INLINE_TIMER_TopSet(transmute(timer), val) }
}

pub fn unlock(timer: &Timer) {
    unsafe { STATIC_INLINE_TIMER_Unlock(transmute(timer)) }
}


pub static TIMER_IF_OF: u32     = (0x1 << 0);
pub static TIMER_IF_UF: u32     = (0x1 << 1);
pub static TIMER_IF_CC0: u32    = (0x1 << 4);
pub static TIMER_IF_CC1: u32    = (0x1 << 5);
pub static TIMER_IF_CC2: u32    = (0x1 << 6);
pub static TIMER_IF_ICBOF0: u32 = (0x1 << 8);
pub static TIMER_IF_ICBOF1: u32 = (0x1 << 9);
pub static TIMER_IF_ICBOF2: u32 = (0x1 << 10);
