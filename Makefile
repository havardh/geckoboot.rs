LLVM_36_HOME=../llvm
ARM_GCC_TOOLCHAIN=/Users/havard/arm-cs-tools

RUST_SRC=../rust
RUSTC=rustc
LLC=$(LLVM_36_HOME)/Debug+Asserts/bin/llc
eACommander=/Applications/eACommander.app/Contents/MacOS/eACommander

DEVICE=EFM32GG990F1024
TARGET=thumbv7m-none-eabi

# Put your source files here (or *.c, etc)
SRCS=sys/system_efm32gg.c

SRCS += sys/src/em_acmp.c \
  sys/src/em_adc.c \
  sys/src/em_aes.c \
  sys/src/em_assert.c \
  sys/src/em_burtc.c \
  sys/src/em_cmu.c \
  sys/src/em_dac.c \
  sys/src/em_dbg.c \
  sys/src/em_dma.c \
  sys/src/em_ebi.c \
  sys/src/em_emu.c \
  sys/src/em_gpio.c \
  sys/src/em_i2c.c \
  sys/src/em_idac.c \
  sys/src/em_int.c \
  sys/src/em_lcd.c \
  sys/src/em_lesense.c \
  sys/src/em_letimer.c \
  sys/src/em_leuart.c \
  sys/src/em_mpu.c \
  sys/src/em_msc.c \
  sys/src/em_opamp.c \
  sys/src/em_pcnt.c \
  sys/src/em_prs.c \
  sys/src/em_rmu.c \
  sys/src/em_rtc.c \
  sys/src/em_system.c \
  sys/src/em_timer.c \
  sys/src/em_usart.c \
  sys/src/em_vcmp.c \
  sys/src/em_wdog.c \

SRCS += gpio.c \
	core.c

# Binaries will be generated with this name (.elf, .bin, .hex, etc)
PROJ_NAME=blinky

# Normally you shouldn't need to change anything below this line!
#######################################################################################

CC=$(ARM_GCC_TOOLCHAIN)/bin/arm-none-eabi-gcc
GDB=$(ARM_GCC_TOOLCHAIN)/bin/arm-none-eabi-gdb
OBJCOPY=$(ARM_GCC_TOOLCHAIN)/bin/arm-none-eabi-objcopy
FLASH=$(eACommander)

CFLAGS  = -g -O0 -Wall -Tsys/efm32gg.ld 
CFLAGS += -mlittle-endian -mthumb -mcpu=cortex-m3
CFLAGS += -mfloat-abi=soft -mfpu=fpv4-sp-d16
CFLAGS += -Isys/inc -Isys/inc/core
CFLAGS += -D$(DEVICE)
CFLAGS += -std=c99

RUSTFLAGS = --target $(TARGET) \
	--crate-type lib \
	-L . --emit llvm-ir \
	-A non_camel_case_types \
	-A dead_code \
	-A non_snake_case 

LLCFLAGS = -mtriple arm-none-eabi \
  -march=thumb \
  -mattr=+thumb2 \
  -mcpu=cortex-m3 \
  --float-abi=soft \
  --asm-verbose=false \
  -O0

# Note: A bug in the OSX eACommander requires at least two flags to run in cli mode
FLASHFLAGS = --verify --reset

# add startup file to build
SRCS += sys/startup_efm32gg.s
OBJS = $(SRCS:.c=.o)

.PHONY: proj

all: clean proj

proj: $(PROJ_NAME).elf

blinky.s: blinky.rs libcore.rlib
	$(RUSTC) $(RUSTFLAGS) $(PROJ_NAME).rs
	$(LLC) $(LLCFLAGS) $(PROJ_NAME).ll -o=$(PROJ_NAME).s 

libcore.rlib:
	$(RUSTC) -O --target $(TARGET) $(RUST_SRC)/src/libcore/lib.rs

$(PROJ_NAME).elf: $(SRCS) blinky.s
	$(CC) -O0 $(CFLAGS) $^ -o $@ 
	$(OBJCOPY) -O ihex $(PROJ_NAME).elf $(PROJ_NAME).hex
	$(OBJCOPY) -O binary $(PROJ_NAME).elf $(PROJ_NAME).bin

.PHONY: flash
flash: $(PROJ_NAME).elf
	$(FLASH) $(FLASHFLAGS) --flash $(PROJ_NAME).bin

.PHONY: debug
debug: flash
	$(GDB) -x efm32gdbinit

clean:
	rm -f *.o $(PROJ_NAME).elf $(PROJ_NAME).hex $(PROJ_NAME).bin $(PROJ_NAME).s $(PROJ_NAME).ll
