-include Makefile.user

DEVICE=EFM32GG990F1024
TARGET=thumbv7m-none-eabi

ARM_GCC_TOOLCHAIN=$(SIMPLICITY_STUDIO_HOME)/developer/toolchains/gnu_arm/4.8_2013q4/
LIB_PATH=$(SIMPLICITY_STUDIO_HOME)/developer/sdks/efm32/v2

RUSTC=/usr/local/bin/rustc
FLASH=eACommander

LIB_DIR=lib
OUT_DIR=out

# Binaries will be generated with this name (.elf, .bin, .hex, etc)
PROJ_NAME=buttons_int

OUT = $(OUT_DIR)/$(PROJ_NAME)

.PHONY: proj
all: proj

proj: $(OUT).hex $(OUT).bin

include Makefile.emlib

LDFLAGS = $(AFLAGS) -T$(LIB_PATH)/Device/SiliconLabs/EFM32GG/Source/GCC/efm32gg.ld -Wl,--start-group -lgcc -lc -lnosys -Wl,--end-group
RUSTFLAGS = --target $(TARGET) -g -C link-args="$(LDFLAGS)" -L . -L $(LIB_DIR) --verbose
FLASHFLAGS = --verify --reset

$(OUT).elf: $(PROJ_NAME).rs $(LIB_DIR)/libcompiler-rt.a #$(OUT_DIR)/libcore.rlib 
	$(RUSTC) $(RUSTFLAGS) $(PROJ_NAME).rs -o $@

$(OUT).hex: $(OUT).elf
	$(OBJCOPY) -O ihex $(OUT).elf $(OUT).hex

$(OUT).bin: $(OUT).elf
	$(OBJCOPY) -O binary $(OUT).elf $(OUT).bin

.PHONY: flash
flash: all
	$(FLASH) --flash $(OUT).bin $(FLASHFLAGS)

.PHONY:clean
clean:
	rm out/*

