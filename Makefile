LLVM_36_HOME=../llvm
ARM_GCC_TOOLCHAIN=/Applications/SimplicityStudio_v2/developer/toolchains/gnu_arm/4.8_2013q4/

RUST_SRC=../rust
RUSTC=rustc
LLC=$(LLVM_36_HOME)/Debug+Asserts/bin/llc
eACommander=/Applications/eACommander.app/Contents/MacOS/eACommander

DEVICE=EFM32GG990F1024
TARGET=thumbv7m-none-eabi

SIMPLICITY_STUDIO=/Applications/SimplicityStudio_v2
LIB_PATH=$(SIMPLICITY_STUDIO)/developer/sdks/efm32/v2/

LIB_DIR=lib
OUT_DIR=out

INCLUDEPATHS += \
-I$(LIB_PATH)/CMSIS/Include \
-I$(LIB_PATH)/Device/SiliconLabs/EFM32GG/Include \
-I$(LIB_PATH)/emlib/inc \
-I$(LIB_PATH)/kits/common/drivers \
-I$(LIB_PATH)/kits/common/bsp \
-I$(LIB_PATH)/kits/EFM32GG_STK3700/config \
-I$(LIB_PATH)/emdrv/gpiointerrupt/inc

SRCS = \
  $(LIB_PATH)/Device/SiliconLabs/EFM32GG/Source/system_efm32gg.c \
  $(LIB_PATH)/emlib/src/em_cmu.c \
  $(LIB_PATH)/emlib/src/em_emu.c \
  $(LIB_PATH)/emlib/src/em_gpio.c \
  $(LIB_PATH)/emlib/src/em_system.c \
  $(LIB_PATH)/emlib/src/em_int.c \
  $(LIB_PATH)/kits/common/drivers/retargetio.c \
  $(LIB_PATH)/emdrv/gpiointerrupt/src/gpiointerrupt.c 

#  $(LIB_PATH)/emlib/src/em_emu.c \
#  $(LIB_PATH)/kits/common/drivers/segmentlcd.c \
#  $(LIB_PATH)/kits/common/bsp/bsp_bcc.c \
#  $(LIB_PATH)/kits/common/bsp/bsp_stk.c \
#  $(LIB_PATH)/kits/common/bsp/bsp_stk_leds.c \
#  $(LIB_PATH)/kits/common/bsp/bsp_trace.c \
#  $(LIB_PATH)/emlib/src/em_assert.c \
#  $(LIB_PATH)/emlib/src/em_ebi.c \
#  $(LIB_PATH)/emlib/src/em_lcd.c \

#  $(LIB_PATH)/emlib/src/em_usart.c \



SRCS += emlib/gpio.c \
	emlib/chip.c \
	cmsis/cmsis.c \
	emlib/swo.c \
	emlib/timer.c \
	emdrv/gpiointerrupt.c \

# Binaries will be generated with this name (.elf, .bin, .hex, etc)
PROJ_NAME=blinky

# Normally you shouldn't need to change anything below this line!
#######################################################################################

CC=$(ARM_GCC_TOOLCHAIN)/bin/arm-none-eabi-gcc
GDB=$(ARM_GCC_TOOLCHAIN)/bin/arm-none-eabi-gdb
OBJCOPY=$(ARM_GCC_TOOLCHAIN)/bin/arm-none-eabi-objcopy
FLASH=$(eACommander)

CFLAGS  = -g -O0 -Wall -T$(LIB_PATH)/Device/SiliconLabs/EFM32GG/Source/GCC/efm32gg.ld
CFLAGS += -mthumb -mcpu=cortex-m3
CFLAGS += $(INCLUDEPATHS)
CFLAGS += -D$(DEVICE)
CFLAGS += -std=c99 --specs=nano.specs
CFLAGS += -Wl,--start-group -lgcc -lc -lnosys -Wl,--end-group

RUSTFLAGS = --target $(TARGET) \
	--crate-type lib -g \
	-L $(LIB_DIR) --emit asm \
	--out-dir $(OUT_DIR) \
	-A non_camel_case_types \
	-A dead_code \
	-A non_snake_case 

RUSTLIBFLAGS = -O -g --target $(TARGET) -L $(LIB_DIR) --cfg stage0 --out-dir $(LIB_DIR)

# Note: A bug in the OSX eACommander requires at least two flags to run in cli mode
FLASHFLAGS = --verify --reset

# add startup file to build
SRCS += $(LIB_PATH)/Device/SiliconLabs/EFM32GG/Source/GCC/startup_efm32gg.S
OBJS = $(SRCS:.c=.o)

.PHONY: proj

all: clean proj

proj: $(OUT_DIR)/$(PROJ_NAME).elf

$(OUT_DIR)/$(PROJ_NAME).s: $(PROJ_NAME).rs #$(OUT_DIR)/libcore.rlib $(OUT_DIR)/libcollections.rlib
	$(RUSTC) $(RUSTFLAGS) $(PROJ_NAME).rs

$(OUT_DIR)/$(PROJ_NAME).elf: $(SRCS) $(OUT_DIR)/$(PROJ_NAME).s
	$(CC) -O0 $(CFLAGS) $^ -o $@
	$(OBJCOPY) -O ihex $(OUT_DIR)/$(PROJ_NAME).elf $(OUT_DIR)/$(PROJ_NAME).hex
	$(OBJCOPY) -O binary $(OUT_DIR)/$(PROJ_NAME).elf $(OUT_DIR)/$(PROJ_NAME).bin

# Building Rust library

.PHONY: librust
librust: $(OUT_DIR)/libcore.rlib $(OUT_DIR)/libcollections.rlib

$(OUT_DIR)/libcore.rlib: $(RUST_SRC)/src/libcore/lib.rs
	$(RUSTC) $(RUSTLIBFLAGS) $(RUST_SRC)/src/libcore/lib.rs

$(OUT_DIR)/libcollections.rlib: $(RUST_SRC)/src/libcollections/lib.rs $(OUT_DIR)/libunicode.rlib $(OUT_DIR)/liballoc.rlib
	$(RUSTC) $(RUSTLIBFLAGS) $(RUST_SRC)/src/libcollections/lib.rs

$(OUT_DIR)/libunicode.rlib: $(RUST_SRC)/src/libunicode/lib.rs
	$(RUSTC) $(RUSTLIBFLAGS) $(RUST_SRC)/src/libunicode/lib.rs

$(OUT_DIR)/liballoc.rlib: $(RUST_SRC)/src/liballoc/lib.rs $(OUT_DIR)/liblibc.rlib
	$(RUSTC) $(RUSTLIBFLAGS) $(RUST_SRC)/src/liballoc/lib.rs

$(OUT_DIR)/liblibc.rlib: $(RUST_SRC)/src/liblibc/lib.rs
	$(RUSTC) $(RUSTLIBFLAGS) $(RUST_SRC)/src/liblibc/lib.rs
# Rust Library

.PHONY: flash
flash: all
	$(FLASH) --flash $(OUT_DIR)/$(PROJ_NAME).bin $(FLASHFLAGS)

.PHONY: debug
debug: 
	$(GDB) -x efm32gdbinit

clean:
	rm -f out/*
