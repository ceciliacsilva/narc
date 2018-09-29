// #![deny(unsafe_code)]
#![feature(panic_implementation)]
#![no_std]
#![no_main]


#[macro_use]
extern crate cortex_m;
#[macro_use]
extern crate cortex_m_rt;
extern crate stm32l0;

mod aux;
mod peripherals;

entry!(main);

fn main() -> ! {
    let _y;
    let x = 42;
    _y = x;

    peripherals::gpio_test();

    // infinite loop; just so we don't leave this stack frame
    loop {
                
    }
}
