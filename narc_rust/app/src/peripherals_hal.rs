use hal::stm32l052;

use hal::{rcc::RccExt, gpio::GpioExt, pwm::PwmExt};
use hal::{flash::FlashExt};
use hal::time::U32Ext;

use embedded_hal::digital::OutputPin;
use embedded_hal::digital::InputPin;
use embedded_hal::{PwmPin};

pub fn gpio_test() {
    let hw = stm32l052::Peripherals::take().unwrap();

    let mut rcc = hw.RCC.constrain();
    let mut flash = hw.FLASH.constrain();

    let mut gpioa = hw.GPIOA.split(&mut rcc.iop);
    let clocks = rcc.cfgr.freeze(&mut flash.acr);//system clock config

    // let mut b = gpioa.pa5.into_output(&mut gpioa.moder).push_pull(&mut gpioa.otyper);
    // b.set_high();

    let c1 = gpioa.pa5.into_alternate(&mut gpioa.moder).af5(&mut gpioa.afrl);

    // let button = gpioa.pa4.into_pull_up_input(&mut gpioa.moder, &mut gpioa.pupdr);

    let mut pwm = hw.TIM2
                    .pwm(
                        c1,
                        1.hz(),
                        clocks,
                        &mut rcc.apb1,
                    );

    
    let max = pwm.get_max_duty();
    pwm.enable();

    pwm.set_duty(max / 2);

    // loop{
    //     match button.is_high() {
    //         true => pwm.set_duty(max / 4),
    //         false => pwm.set_duty(max / 2),
    //     }
    // }
    
    // let mut led = gpioa.pa5.into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);

    // let button = gpioa.pa4.into_pull_up_input(&mut gpioa.moder, &mut gpioa.pupdr);

    // loop {
    //     match button.is_high() {
    //         true => led.set_high(),
    //         false => led.set_low(),
    //     }
    // }

}