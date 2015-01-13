#include "em_device.h"
#include "core_cm3.h"

uint32_t SysTick_Config_(uint32_t ticks) {
  return SysTick_Config(ticks);
}
