//! HAL for microcontroller STM32l052

#![no_std]
#![feature(unsize)]
#![feature(never_type)]

pub extern crate stm32l052;
extern crate embedded_hal;
extern crate cortex_m;
extern crate cast;
extern crate nb;

pub mod flash;
pub mod rcc;
pub mod gpio;
pub mod time;
pub mod pwm;
pub mod delay;
pub mod adc;
pub mod serial;
pub mod dma;