use stm32l0::stm32l0x1;

use hal::{rcc::RccExt, gpio::GpioExt, pwm::PwmExt};

use embedded_hal::digital::OutputPin;
use embedded_hal::digital::InputPin;

pub fn gpio_test() {
    let hw = stm32l0x1::Peripherals::take().unwrap();

    let mut rcc = hw.RCC.constrain();
    
    let mut gpioa = hw.GPIOA.split(&mut rcc.iop);

    let mut tim2 = hw.TIM2.split(&mut rcc.apb1);

    tim2
        .psc
        .psc()
        .modify(|_, w| unsafe{ w.bits(15) });

    tim2
        .arr
        .arr()
        .modify(|_, w| unsafe{ w.bits(8) });

    tim2
        .ccr1
        .ccr1()
        .modify(|_, w| unsafe{ w.bits(4) });

    tim2
        .ccmr1_output
        .ccmr1_output()
        .modify(|_, w| unsafe{ w.oc1m().bits(0b110) });


    tim2
        .ccmr1_output
        .ccmr1_output()
        .modify(|_, w| w.oc1pe().set_bit());
    
    tim2
        .ccer
        .ccer()
        .modify(|_, w| w.cc1e().set_bit());

    tim2
        .cr1
        .cr1()
        .modify(|_, w| w.cen().set_bit());

    tim2
        .egr
        .egr()
        .write(|w| w.ug().set_bit());

    let _motor = gpioa.pa5.into_push_pull_af5(&mut gpioa.moder, &mut gpioa.afrl);

    /* let mut led = gpioa.pa5.into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);

    let button = gpioa.pa4.into_pull_up_input(&mut gpioa.moder, &mut gpioa.pupdr);

    loop {
        match button.is_high() {
            true => led.set_high(),
            false => led.set_low(),
        }
    }
 */
}