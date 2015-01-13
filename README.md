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

Compiling:
----------

Edit "Makefile" with the path to the rust source, rustc compiler, gcc-toolchain and llvm, eACommander and type "make flash".

Upload the resulting binary, blinky.bin, on the target with eACommander.

Structure
---------
  * blinky.rs - sample program (blinks the LED1 of the EFM32GG_STK3700 board)
  * sys/ - bootstrap code (boot loader and system initialization)
  * zero/ - zero.rs

Credits
-------
  * The project structure and build scripts: https://github.com/neykov/armboot/, https://github.com/rowol/stm32_discovery_arm_gcc
  * Rust zero.rs: https://github.com/pcwalton/zero.rs
  * C source for inspiration: https://github.com/EnergyMicro/EFM32GG_STK3700
  * emlib for EFM32GG: https://github.com/EnergyMicro/emlib
  * CMSIS: https://github.com/EnergyMicro/CMSIS
  * target triple: https://github.com/hackndev/zinc
