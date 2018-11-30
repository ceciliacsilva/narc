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
    
    let hw = stm32l052::Peripherals::take().unwrap();

        let mut rcc = hw.RCC.constrain();
        let mut flash = hw.FLASH.constrain();

        let mut gpioa = hw.GPIOA.split(&mut rcc.iop);
        let mut gpiob = hw.GPIOB.split(&mut rcc.iop);
        //let mut gpioc = hw.GPIOC.split(&mut rcc.iop);

        let clocks = rcc.cfgr.freeze(&mut flash.acr);

        let mot2 = gpioa.pa0.into_alternate(&mut gpioa.moder).af2(&mut gpioa.afrl);
        let mut led = gpioa.pa5.into_output(&mut gpioa.moder).push_pull(&mut gpioa.otyper);
        //let mot1 = gpioa.pa1.into_alternate(&mut gpioa.moder).af2(&mut gpioa.afrl);
        let mut mot2_in2 = gpiob.pb6.into_output(&mut gpiob.moder).push_pull(&mut gpiob.otyper);
        let mut mot2_in1 = gpiob.pb7.into_output(&mut gpiob.moder).push_pull(&mut gpiob.otyper);
        //let mot1_in1 = gpioc.pc14.into_input(&mut gpioc.moder).pull_down(&mut gpioc.pupdr);
        //let mot1_in2 = gpioc.pc15.into_input(&mut gpioc.moder).pull_down(&mut gpioc.pupdr);
        
        mot2_in1.set_high();
        mot2_in2.set_low();

        let mut mot2_pwm = hw.TIM2
                    .pwm(
                        mot2,
                        60.hz(),
                        clocks,
                        &mut rcc.apb1,
                    );
        
        /*let mut mot1_pwm = hw.TIM2
                    .pwm(
                        mot1,
                        60.hz(),
                        clocks,
                        &mut rcc.apb1,
                    );*/

        led.set_high();

        let max = mot2_pwm.get_max_duty();
        mot2_pwm.enable(); 

    loop{
        
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
