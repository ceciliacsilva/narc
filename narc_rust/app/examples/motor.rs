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
 use narc_hal::gpio::GpioExt;
 use narc_hal::pwm::PwmExt;
 use narc_hal::flash::FlashExt;
 use narc_hal::time::U32Ext;
 use narc_hal::qei::*;

 use embedded_hal::digital::OutputPin;
 use embedded_hal::digital::InputPin;
 use embedded_hal::{Direction, Qei};
 
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
    //let mot1_in2 = gpioc.pc15.into_input(&mut gpioc.moder).pull_down(&mut gpioc.pupdr)w;
    let outa = gpioa.pa6.into_analog(&mut gpioa.moder, &mut gpioa.pupdr);
    let outb = gpioa.pa7.into_analog(&mut gpioa.moder, &mut gpioa.pupdr);
    
    let qei = hw.TIM22
            .qei(
                (outa, outb),
                &mut rcc.apb2);
        
    mot2_in1.set_low();
    mot2_in2.set_high();

    let mut mot2_pwm = hw.TIM2
                .pwm(
                    mot2,
                    60.hz(),
                    clocks,
                    &mut rcc.apb1,
                );

    let max = mot2_pwm.get_max_duty();
    mot2_pwm.enable();
    mot2_pwm.set_duty(max/2); 

    qei.reset();

    match qei.direction() {
        Direction::Downcounting => {
            led.set_high()
        }
        _ => { led.set_low() }
    };

     loop{
        if qei.count() > 0 {
            led.set_high();
        }
     }
 }

 #[panic_handler]
 fn panic(_info: &PanicInfo) -> ! {
     bkpt();
 
     loop {
         atomic::compiler_fence(Ordering::SeqCst)
     }
 }