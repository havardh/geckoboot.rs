
pub enum IRQn {
/******  Cortex-M3 Processor Exceptions Numbers *******************************************/
  NonMaskableInt   = -14,              /*!< 2 Non Maskable Interrupt                 */
  HardFault        = -13,              /*!< 3 Cortex-M3 Hard Fault Interrupt         */
  MemoryManagement = -12,              /*!< 4 Cortex-M3 Memory Management Interrupt  */
  BusFault         = -11,              /*!< 5 Cortex-M3 Bus Fault Interrupt          */
  UsageFault       = -10,              /*!< 6 Cortex-M3 Usage Fault Interrupt        */
  SVCall           = -5,               /*!< 11 Cortex-M3 SV Call Interrupt           */
  DebugMonitor     = -4,               /*!< 12 Cortex-M3 Debug Monitor Interrupt     */
  PendSV           = -2,               /*!< 14 Cortex-M3 Pend SV Interrupt           */
  SysTick          = -1,               /*!< 15 Cortex-M3 System Tick Interrupt       */

/******  EFM32G Peripheral Interrupt Numbers **********************************************/
  DMA              = 0,  /*!< 16+0 EFM32 DMA Interrupt */
  GPIO_EVEN        = 1,  /*!< 16+1 EFM32 GPIO_EVEN Interrupt */
  TIMER0           = 2,  /*!< 16+2 EFM32 TIMER0 Interrupt */
  USART0_RX        = 3,  /*!< 16+3 EFM32 USART0_RX Interrupt */
  USART0_TX        = 4,  /*!< 16+4 EFM32 USART0_TX Interrupt */
  USB              = 5,  /*!< 16+5 EFM32 USB Interrupt */
  ACMP0            = 6,  /*!< 16+6 EFM32 ACMP0 Interrupt */
  ADC0             = 7,  /*!< 16+7 EFM32 ADC0 Interrupt */
  DAC0             = 8,  /*!< 16+8 EFM32 DAC0 Interrupt */
  I2C0             = 9,  /*!< 16+9 EFM32 I2C0 Interrupt */
  I2C1             = 10, /*!< 16+10 EFM32 I2C1 Interrupt */
  GPIO_ODD         = 11, /*!< 16+11 EFM32 GPIO_ODD Interrupt */
  TIMER1           = 12, /*!< 16+12 EFM32 TIMER1 Interrupt */
  TIMER2           = 13, /*!< 16+13 EFM32 TIMER2 Interrupt */
  TIMER3           = 14, /*!< 16+14 EFM32 TIMER3 Interrupt */
  USART1_RX        = 15, /*!< 16+15 EFM32 USART1_RX Interrupt */
  USART1_TX        = 16, /*!< 16+16 EFM32 USART1_TX Interrupt */
  LESENSE          = 17, /*!< 16+17 EFM32 LESENSE Interrupt */
  USART2_RX        = 18, /*!< 16+18 EFM32 USART2_RX Interrupt */
  USART2_TX        = 19, /*!< 16+19 EFM32 USART2_TX Interrupt */
  UART0_RX         = 20, /*!< 16+20 EFM32 UART0_RX Interrupt */
  UART0_TX         = 21, /*!< 16+21 EFM32 UART0_TX Interrupt */
  UART1_RX         = 22, /*!< 16+22 EFM32 UART1_RX Interrupt */
  UART1_TX         = 23, /*!< 16+23 EFM32 UART1_TX Interrupt */
  LEUART0          = 24, /*!< 16+24 EFM32 LEUART0 Interrupt */
  LEUART1          = 25, /*!< 16+25 EFM32 LEUART1 Interrupt */
  LETIMER0         = 26, /*!< 16+26 EFM32 LETIMER0 Interrupt */
  PCNT0            = 27, /*!< 16+27 EFM32 PCNT0 Interrupt */
  PCNT1            = 28, /*!< 16+28 EFM32 PCNT1 Interrupt */
  PCNT2            = 29, /*!< 16+29 EFM32 PCNT2 Interrupt */
  RTC              = 30, /*!< 16+30 EFM32 RTC Interrupt */
  BURTC            = 31, /*!< 16+31 EFM32 BURTC Interrupt */
  CMU              = 32, /*!< 16+32 EFM32 CMU Interrupt */
  VCMP             = 33, /*!< 16+33 EFM32 VCMP Interrupt */
  LCD              = 34, /*!< 16+34 EFM32 LCD Interrupt */
  MSC              = 35, /*!< 16+35 EFM32 MSC Interrupt */
  AES              = 36, /*!< 16+36 EFM32 AES Interrupt */
  EBI              = 37, /*!< 16+37 EFM32 EBI Interrupt */
  EMU              = 38, /*!< 16+38 EFM32 EMU Interrupt */
}

extern {
    pub fn STATIC_INLINE_NVIC_SetPriorityGrouping(PriorityGroup: u32);
    pub fn STATIC_INLINE_NVIC_GetPriorityGrouping();
    pub fn STATIC_INLINE_NVIC_EnableIRQ(IRQn: IRQn);
    pub fn STATIC_INLINE_NVIC_DisableIRQ(IRQn: IRQn);
    pub fn STATIC_INLINE_NVIC_GetPendingIRQ(IRQn: IRQn);
    pub fn STATIC_INLINE_NVIC_SetPendingIRQ(IRQn: IRQn);
    pub fn STATIC_INLINE_NVIC_ClearPendingIRQ(IRQn: IRQn);
    pub fn STATIC_INLINE_NVIC_GetActive(IRQn: IRQn);
    pub fn STATIC_INLINE_NVIC_SetPriority(IRQn: IRQn, priority: u32);
    pub fn STATIC_INLINE_NVIC_GetPriority(IRQn: IRQn);
    pub fn STATIC_INLINE_NVIC_EncodePriority (PriorityGroup: u32, PreemptPriority: u32, SubPriority: u32);
    pub fn STATIC_INLINE_NVIC_DecodePriority (Priority: u32, PriorityGroup: u32, pPreemptPriority: *mut u32, pSubPriority: *mut u32);
    pub fn STATIC_INLINE_NVIC_SystemReset();
}

pub fn NVIC_SetPriorityGrouping(PriorityGroup: u32) {
    unsafe { NVIC_SetPriorityGrouping(PriorityGroup) }
}

pub fn NVIC_GetPriorityGrouping() {
    unsafe { NVIC_GetPriorityGrouping() }
}

pub fn NVIC_EnableIRQ(IRQn: IRQn) {
    unsafe { NVIC_EnableIRQ(IRQn) }
}

pub fn NVIC_DisableIRQ(IRQn: IRQn) {
    unsafe { NVIC_DisableIRQ(IRQn) }
}

pub fn NVIC_GetPendingIRQ(IRQn: IRQn) {
    unsafe { NVIC_GetPendingIRQ(IRQn) }
}

pub fn NVIC_SetPendingIRQ(IRQn: IRQn) {
    unsafe { NVIC_SetPendingIRQ(IRQn) }
}

pub fn NVIC_ClearPendingIRQ(IRQn: IRQn) {
    unsafe { NVIC_ClearPendingIRQ(IRQn) }
}

pub fn NVIC_GetActive(IRQn: IRQn) {
    unsafe { NVIC_GetActive(IRQn) }
}

pub fn NVIC_SetPriority(IRQn: IRQn, priority: u32) {
    unsafe { NVIC_SetPriority(IRQn, priority) }
}

pub fn NVIC_GetPriority(IRQn: IRQn) {
    unsafe { NVIC_GetPriority(IRQn) }
}

pub fn NVIC_EncodePriority(PriorityGroup: u32, PreemptPriority: u32, SubPriority: u32) {
    unsafe { NVIC_EncodePriority(PriorityGroup, PreemptPriority, SubPriority) }
}

pub fn NVIC_DecodePriority(Priority: u32, PriorityGroup: u32, pPreemptPriority: *mut u32, pSubPriority: *mut u32) {
    unsafe { NVIC_DecodePriority(Priority, PriorityGroup, pPreemptPriority, pSubPriority) }
}

pub fn NVIC_SystemReset() {
    unsafe { NVIC_SystemReset() }
}

