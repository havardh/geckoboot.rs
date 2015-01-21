#include "em_device.h"
#include "core_cm3.h"

uint32_t STATIC_INLINE_SysTick_Config(uint32_t ticks) {
  return SysTick_Config(ticks);
}
