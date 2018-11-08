#![feature(panic_implementation)]
#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate hal;
extern crate embedded_hal;

use core::panic::PanicInfo;
use core::sync::atomic::{self, Ordering};
use cortex_m::asm::bkpt;

use hal::stm32l052;
use hal::rcc::RccExt;
use hal::gpio::GpioExt;
use hal::pwm::PwmExt;
use hal::flash::FlashExt;
use hal::time::U32Ext;
use hal::delay::Delay;

use hal::adc::{adc_config, adc_read};
use hal::gpio::{Output, PushPull, gpioa::PA5};


use embedded_hal::PwmPin;
use embedded_hal::prelude::*;

use cortex_m::peripheral::syst::SystClkSource;

use cortex_m_rt::{entry, exception};

use stm32l052::GPIOA;

#[entry]
fn main() -> ! {
    let hw = stm32l052::Peripherals::take().unwrap();
    let mut cp = cortex_m::Peripherals::take().unwrap();

    let mut rcc = hw.RCC.constrain();
    let mut flash = hw.FLASH.constrain();
    let mut adc = hw.ADC;
    let mut gpioa = hw.GPIOA.split(&mut rcc.iop);

    gpioa.pa5.into_output(&mut gpioa.moder).push_pull(&mut gpioa.otyper);

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

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

#[allow(deprecated)]
#[panic_implementation]
fn panic(_info: &PanicInfo) -> ! {
    bkpt();

    loop {
        atomic::compiler_fence(Ordering::SeqCst)
    }
}
