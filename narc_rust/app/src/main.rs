// #![deny(unsafe_code)]
#![feature(panic_implementation)]
#![no_std]
#![no_main]


// #[macro_use]
extern crate cortex_m;
// #[macro_use]
extern crate cortex_m_rt;
extern crate cortex_m_semihosting;
extern crate hal;
extern crate embedded_hal;

use hal::stm32l052;

mod aux;
// mod peripherals;
mod peripherals_hal;

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    let _y;
    let x = 42;
    _y = x;

    // peripherals::gpio_test();

    peripherals_hal::gpio_test();
    
    // infinite loop; just so we don't leave this stack frame
    loop {
                
    }
}
