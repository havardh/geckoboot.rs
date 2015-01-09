geckoboot.rs
============

A blinky demo for using the Rust programming language bare metal on EFM32GG_STK3700 Giant Gecko starter kit from Silicon Labs.

This repository is based on https://github.com/neykov/armboot/ and adapted for the Giant Gecko kit.

Requirements:
-------------
  * arm-none-eabi toolchain
  * llvm-3.6 (nightly) toolchain
  * rustc (requires Rust 0.12)

Compiling:
----------

Edit "Makefile" with the path to the rustc compiler, gcc-toolchain and llvm, and type "make".

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