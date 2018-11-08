//! Delays

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
