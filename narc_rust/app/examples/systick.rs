#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate narc_hal;
extern crate embedded_hal;

use core::panic::PanicInfo;
use core::sync::atomic::{self, Ordering};
use cortex_m::asm::bkpt;

use narc_hal::stm32l052;
use narc_hal::rcc::RccExt;

use cortex_m::peripheral::syst::SystClkSource;

use cortex_m_rt::{entry, exception};

use stm32l052::GPIOA;

#[entry]
fn main() -> ! {
    let hw = stm32l052::Peripherals::take().unwrap();
    let mut cp = cortex_m::Peripherals::take().unwrap();

    let _rcc = hw.RCC.constrain();

    cp.SYST.set_clock_source(SystClkSource::Core);
    cp.SYST.set_reload(2_000_000); // 1s
    cp.SYST.clear_current();
    cp.SYST.enable_counter();
    cp.SYST.enable_interrupt();

    loop{
    }
}

#[exception]
fn SysTick() {
    static mut ON: bool = true;

    *ON = !*ON;

    if *ON {
        unsafe {
            (*GPIOA::ptr()).bsrr.write(|w| w.bs5().set_bit());
        }
    } else {
        unsafe {
            (*GPIOA::ptr()).bsrr.write(|w| w.br5().set_bit());
        }
    }

    // unsafe {
    //     (*GPIOA::ptr()).bsrr.write(|w| w.bs5().set_bit());
    // }

}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    bkpt();

    loop {
        atomic::compiler_fence(Ordering::SeqCst)
    }
}
