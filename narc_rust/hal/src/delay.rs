//! Delays

//!
//! # Example
//! ```
//! #![feature(panic_implementation)]
//! #![no_std]
//! #![no_main]
//!
//! extern crate cortex_m;
//! extern crate cortex_m_rt;
//! extern crate hal;
//! extern crate embedded_hal;
//! 
//! use hal::stm32l052;
//! use hal::gpio::GpioExt;
//! use hal::rcc::RccExt; 
//! use core::panic::PanicInfo;
//! use core::sync::atomic::{self, Ordering};
//! use cortex_m::asm::bkpt;
//! use embedded_hal::digital::OutputPin; 
//! use embedded_hal::digital::InputPin; 
//! use hal::delay::Delay;
//! use hal::flash::FlashExt;    
//! use embedded_hal::prelude::*;  
//!
//! use cortex_m_rt::entry;
//!
//! #[entry]
//! fn main() -> ! {
//!
//!     let hw = stm32l052::Peripherals::take().unwrap();
//!     let cp = cortex_m::Peripherals::take().unwrap();
//!     let mut rcc = hw.RCC.constrain();
//!     let mut flash = hw.FLASH.constrain();
//!     let clocks = rcc.cfgr.freeze(&mut flash.acr);
//!     
//!     let mut gpioa = hw.GPIOA.split(&mut rcc.iop);
//!     let mut led = gpioa.pa5.into_output(&mut gpioa.moder).push_pull(&mut gpioa.otyper);
//!     let button = gpioa.pa4.into_input(&mut gpioa.moder).pull_up(&mut gpioa.pupdr);
//!     let mut delay = Delay::new(cp.SYST, clocks);
//!
//!     loop{
//!         if !button.is_low(){
//!             led.set_low();
//!          }
//!         else {
//!            led.set_high();
//!             delay.delay_ms(1_000_u32);
//!         }
//!     }
//! 
//! }
//! 
//! #[allow(deprecated)]
//! #[panic_implementation]
//! fn panic(_info: &PanicInfo) -> ! {
//!     bkpt();
//! 
//!     loop {
//!         atomic::compiler_fence(Ordering::SeqCst)
//!     }
//! }
//! ```

use cast::u32;
use cortex_m::peripheral::syst::SystClkSource;
use cortex_m::peripheral::SYST;

use embedded_hal::blocking::delay::{DelayMs, DelayUs};

use rcc::Clocks;

pub struct Delay {
    clocks: Clocks,
    syst: SYST,
}

impl Delay {
    pub fn new (mut syst: SYST, clocks: Clocks) -> Delay {
        syst.set_clock_source(SystClkSource::Core);

        Delay { syst, clocks }
    }

    pub fn free(self) -> SYST {
        self.syst
    }
}

impl DelayMs<u32> for Delay {
    fn delay_ms(&mut self, ms: u32) {
        self.delay_us(ms * 1_000);
    }
}

impl DelayMs<u16> for Delay {
    fn delay_ms(&mut self, ms: u16) {
        self.delay_ms(u32(ms));
    }
}

impl DelayMs<u8> for Delay {
    fn delay_ms(&mut self, ms: u8) {
        self.delay_ms(u32(ms));
    }
}

impl DelayUs<u32> for Delay {
    fn delay_us(&mut self, us: u32) {
        // The RVR register is 24 bits wide, as SysTick is based on a 24 bit counter
        const MAX_RVR: u32 = (1 << 24);

        let mut total_rvr = us * (self.clocks.sysclk().0 / 1_000_000);

        while total_rvr != 0 {
            let current_rvr = if total_rvr < MAX_RVR {
                total_rvr
            } else {
                MAX_RVR
            };

            self.syst.set_reload(current_rvr);
            self.syst.clear_current();
            self.syst.enable_counter();

            // Update the tracking variable while we are waiting...
            total_rvr -= current_rvr;

            while !self.syst.has_wrapped() {}

            self.syst.disable_counter();
        }
    }
}

impl DelayUs<u16> for Delay {
    fn delay_us(&mut self, us: u16) {
        self.delay_us(u32(us))
    }
}

impl DelayUs<u8> for Delay {
    fn delay_us(&mut self, us: u8) {
        self.delay_us(u32(us))
    }
}
