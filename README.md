geckoboot.rs
============

A blinky demo for using the Rust programming language bare metal on EFM32GG_STK3700 Giant Gecko starter kit from Silicon Labs.

This repository is based on https://github.com/neykov/armboot/ and adapted for the Giant Gecko kit.

Requirements:
-------------
  * arm-none-eabi toolchain
  * llvm-3.6 (nightly) toolchain
  * rustc (tested on "rustc 1.0.0-nightly (44a287e6 2015-01-08 17:03:40 -0800)")
  * Rust libcore compiled for arm (tested at 44a287e6)

Setup:
------

The `rustc` command must be available on your path.
Add the environment variables `SIMPLICITY_STUDIO_HOME` and `RUST_SRC_HOME` pointing to Silicon Labs Simplicity Studio and the Rust Source respectivly. And the arm-gcc-toolchain must be on your path.
For the `make flash` command to work the utility eACommander must be on your path.

Compiling:
----------

`make flash PROJ_NAME=<filename>` excluding .rs

Structure
---------
  * blinky.rs - sample program (blinks LED1 and LED0 on the EFM32GG_STK3700 board)
  * buttons.rs - sample program (hooks up PB0 and PB1 with LED0 and LED1 on the EFM32GG_STK3700 board)
  * emlib/ - rust bindings for emlib
  * emdrv/ - rust bindings for emdrv

Credits
-------
  * The project structure and build scripts: https://github.com/neykov/armboot/, https://github.com/rowol/stm32_discovery_arm_gcc
  * Rust zero.rs: https://github.com/pcwalton/zero.rs
  * C source for inspiration: https://github.com/EnergyMicro/EFM32GG_STK3700
  * emlib for EFM32GG: https://github.com/EnergyMicro/emlib
  * CMSIS: https://github.com/EnergyMicro/CMSIS
  * target triple: https://github.com/hackndev/zinc
