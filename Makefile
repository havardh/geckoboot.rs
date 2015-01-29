DEVICE=EFM32GG990F1024
TARGET=thumbv7m-none-eabi

ARM_GCC_TOOLCHAIN=$(SIMPLICITY_STUDIO_HOME)/developer/toolchains/gnu_arm/4.8_2013q4/
LIB_PATH=$(SIMPLICITY_STUDIO_HOME)/developer/sdks/efm32/v2/

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
	emlib/emu.c \
	emlib/chip.c \
	cmsis/cmsis.c \
	emlib/swo.c \
	emdrv/gpiointerrupt.c \

# Binaries will be generated with this name (.elf, .bin, .hex, etc)
PROJ_NAME=blinky

# Normally you shouldn't need to change anything below this line!
#######################################################################################

RUSTC=/usr/local/bin/rustc
CC=$(ARM_GCC_TOOLCHAIN)/bin/arm-none-eabi-gcc
AR=$(ARM_GCC_TOOLCHAIN)/bin/arm-none-eabi-ar
GDB=$(ARM_GCC_TOOLCHAIN)/bin/arm-none-eabi-gdb
OBJCOPY=$(ARM_GCC_TOOLCHAIN)/bin/arm-none-eabi-objcopy
FLASH=eACommander

AFLAGS = -mthumb -mcpu=cortex-m3 

CFLAGS	= -O0 -Wall -T$(LIB_PATH)/Device/SiliconLabs/EFM32GG/Source/GCC/efm32gg.ld
CFLAGS += -mthumb -mcpu=cortex-m3
CFLAGS += $(INCLUDEPATHS)
CFLAGS += -D$(DEVICE)
CFLAGS += -std=c99
CFLAGS += -Wl,--start-group -lgcc -lc -lnosys -Wl,--end-group

RUSTFLAGS = --target $(TARGET) \
	-g \
	-L$(LIB_DIR) \
    -C link-args="$(LDFLAGS)" \
	--out-dir $(OUT_DIR) \
	-A non_camel_case_types \
	-A dead_code \
	-A non_snake_case --verbose \
	-Z print-link-args

RUSTLIBFLAGS = -O -g --target $(TARGET) -L $(LIB_DIR) --cfg stage0 --out-dir $(LIB_DIR)

LDFLAGS = -T$(LIB_PATH)/Device/SiliconLabs/EFM32GG/Source/GCC/efm32gg.ld $(AFLAGS) -v

# Note: A bug in the OSX eACommander requires at least two flags to run in cli mode
FLASHFLAGS = --verify --reset

# add startup file to build
SRCS += $(LIB_PATH)/Device/SiliconLabs/EFM32GG/Source/GCC/startup_efm32gg.S
OBJS = $(SRCS:.c=.o)
OBJS := $(OBJS:.s=.o)

.c.o:
	$(CC) $(CFLAGS) -c $< -o $@

.s.o:
	$(CC) $(CFLAGS) -c $< -o $@

.PHONY: proj

all: clean proj

proj: $(OUT_DIR)/$(PROJ_NAME).hex $(OUT_DIR)/$(PROJ_NAME).bin

$(LIB_DIR)/libcompiler-rt.a: $(OBJS)
	$(AR) rcs $@ $(OBJS)

$(OUT_DIR)/$(PROJ_NAME).elf: $(PROJ_NAME).rs $(LIB_DIR)/libcompiler-rt.a #$(OUT_DIR)/libcore.rlib 
	$(RUSTC) $(RUSTFLAGS) $(PROJ_NAME).rs -o $@

$(OUT_DIR)/$(PROJ_NAME).hex: $(OUT_DIR)/$(PROJ_NAME).elf
	$(OBJCOPY) -O ihex $(OUT_DIR)/$(PROJ_NAME).elf $(OUT_DIR)/$(PROJ_NAME).hex

$(OUT_DIR)/$(PROJ_NAME).bin: $(OUT_DIR)/$(PROJ_NAME).elf
	$(OBJCOPY) -O binary $(OUT_DIR)/$(PROJ_NAME).elf $(OUT_DIR)/$(PROJ_NAME).bin

# Building Rust library

.PHONY: librust
librust: $(OUT_DIR)/libcore.rlib

$(OUT_DIR)/libcore.rlib: $(RUST_SRC_HOME)/src/libcore/lib.rs
	$(RUSTC) $(RUSTLIBFLAGS) $(RUST_SRC_HOME)/src/libcore/lib.rs

# Rust Library

.PHONY: flash
flash: all
	$(FLASH) --flash $(OUT_DIR)/$(PROJ_NAME).bin $(FLASHFLAGS)

.PHONY: debug
debug:
	$(GDB) -x efm32gdbinit

clean:
	rm -f out/*
