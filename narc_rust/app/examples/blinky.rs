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

use embedded_hal::digital::OutputPin;
use embedded_hal::digital::InputPin;

use embedded_hal::PwmPin;
use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    
    let freq = 1;
    
    let hw = stm32l052::Peripherals::take().unwrap();

        let mut rcc = hw.RCC.constrain();
        let mut flash = hw.FLASH.constrain();

        let mut gpioa = hw.GPIOA.split(&mut rcc.iop);
        let clocks = rcc.cfgr.freeze(&mut flash.acr);

        let led = gpioa.pa5.into_alternate(&mut gpioa.moder).af5(&mut gpioa.afrl);
        let button = gpioa.pa4.into_input(&mut gpioa.moder).pull_up(&mut gpioa.pupdr);


        let mut pwm = hw.TIM2
                    .pwm(
                        led,
                        3.hz(),
                        clocks,
                        &mut rcc.apb1,
                    );

        let max = pwm.get_max_duty();
        pwm.enable();

    loop{
        
        if button.is_low(){
                pwm.set_duty(max / 2);
             }
        else{
            pwm.set_duty(max / 1);
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
