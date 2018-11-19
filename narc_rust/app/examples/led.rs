#![feature(panic_implementation)]
#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate hal;
extern crate embedded_hal;

use hal::stm32l052;
use hal::gpio::GpioExt;
use hal::rcc::RccExt; 
use core::panic::PanicInfo;
use core::sync::atomic::{self, Ordering};
use cortex_m::asm::bkpt;
use embedded_hal::digital::OutputPin; 
use embedded_hal::digital::InputPin;   


use cortex_m_rt::entry;


#[entry]
fn main() -> ! {

    let hw = stm32l052::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();
    let mut rcc = hw.RCC.constrain();
    
    let mut gpioa = hw.GPIOA.split(&mut rcc.iop);
    let mut led = gpioa.pa5.into_output(&mut gpioa.moder).push_pull(&mut gpioa.otyper);
    let button = gpioa.pa4.into_input(&mut gpioa.moder).pull_up(&mut gpioa.pupdr);


    loop{
        if !button.is_low(){
            led.set_high();
         }
        else {
           led.set_low();
        }

    }

}

#[allow(deprecated)]
#[panic_implementation]
fn panic(_info: &PanicInfo) -> ! {
    bkpt();

    loop {
        atomic::compiler_fence(Ordering::SeqCst)
    }
}