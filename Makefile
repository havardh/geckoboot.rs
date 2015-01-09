LLVM_36_HOME=../llvm
ARM_GCC_TOOLCHAIN=/Users/havard/arm-cs-tools

RUSTC=rustc
LLC=$(LLVM_36_HOME)/Debug+Asserts/bin/llc

# Put your source files here (or *.c, etc)
SRCS=sys/system_efm32gg.c
INCS=sys/include

# Binaries will be generated with this name (.elf, .bin, .hex, etc)
PROJ_NAME=blinky

# Normally you shouldn't need to change anything below this line!
#######################################################################################

CC=$(ARM_GCC_TOOLCHAIN)/bin/arm-none-eabi-gcc
OBJCOPY=$(ARM_GCC_TOOLCHAIN)/bin/arm-none-eabi-objcopy

CFLAGS  = -g -O0 -Wall -Tsys/efm32gg.ld 
CFLAGS += -mlittle-endian -mthumb -mcpu=cortex-m3
CFLAGS += -mfloat-abi=soft -mfpu=fpv4-sp-d16
CFLAGS += -Isys/inc -Isys/inc/core

# add startup file to build
SRCS += sys/startup_efm32gg.s
OBJS = $(SRCS:.c=.o)

.PHONY: proj

all: clean proj

proj: $(PROJ_NAME).elf

blinky.s: blinky.rs
	$(RUSTC) --target thumbv7m-none-eabi --crate-type lib --emit llvm-ir -A non_camel_case_types -A dead_code -A non_snake_case blinky.rs
	$(LLC) -mtriple arm-none-eabi -march=thumb -mattr=+thumb2 -mcpu=cortex-m3 --float-abi=soft --asm-verbose=false blinky.ll -o=blinky.s -O0

$(PROJ_NAME).elf: $(SRCS) blinky.s
	$(CC) -O0 $(CFLAGS) $^ -o $@ 
	$(OBJCOPY) -O ihex $(PROJ_NAME).elf $(PROJ_NAME).hex
	$(OBJCOPY) -O binary $(PROJ_NAME).elf $(PROJ_NAME).bin

clean:
	rm -f *.o $(PROJ_NAME).elf $(PROJ_NAME).hex $(PROJ_NAME).bin $(PROJ_NAME).s $(PROJ_NAME).ll
