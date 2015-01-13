#include "em_gpio.h"

void GPIO_PinOutClear_(GPIO_Port_TypeDef port, unsigned int pin) {
    GPIO_PinOutClear(port, pin);
}

void GPIO_PinOutSet_(GPIO_Port_TypeDef port, unsigned int pins) {
    GPIO_PinOutSet(port, pins);
}

void GPIO_PinOutToggle_(GPIO_Port_TypeDef port, unsigned int pin) {
    GPIO_PinOutToggle(port, pin);
}
