#![feature(panic_implementation)]
#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate stm32l0;
extern crate hal;
extern crate embedded_hal;

use core::panic::PanicInfo;
use core::sync::atomic::{self, Ordering};
use cortex_m::asm::bkpt;

use stm32l0::stm32l0x1;
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
    blinky();

    loop{}
}

fn blinky() {
    let hw = stm32l0x1::Peripherals::take().unwrap();

    let mut rcc = hw.RCC.constrain();
    let mut flash = hw.FLASH.constrain();

    let mut gpioa = hw.GPIOA.split(&mut rcc.iop);
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let led = gpioa.pa5.into_alternate(&mut gpioa.moder).af5(&mut gpioa.afrl);

    let mut pwm = hw.TIM2
                    .pwm(
                        led,
                        1.hz(),
                        clocks,
                        &mut rcc.apb1,
                    );

    let max = pwm.get_max_duty();
    pwm.enable();
<<<<<<< HEAD
    pwm.set_duty(max / 4);
=======
    pwm.set_duty(max / 2);

    // let mut led = gpioa.pa5.into_output(&mut gpioa.moder).push_pull(&mut gpioa.otyper);
    // let bot = gpioa.pa4.into_input(&mut gpioa.moder).pull_up(&mut gpioa.pupdr);

    // loop{
    //     if bot.is_high(){
    //         led.set_high();
    //     } else {
    //         led.set_low();
    //     }
    // }

    loop{

    }
>>>>>>> upstream/master
}

#[allow(deprecated)]
#[panic_implementation]
fn panic(_info: &PanicInfo) -> ! {
    bkpt();

    loop {
        atomic::compiler_fence(Ordering::SeqCst)
    }
}
