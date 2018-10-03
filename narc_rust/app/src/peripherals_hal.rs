use stm32l0::stm32l0x1;

use hal::{rcc::RccExt, gpio::GpioExt, pwm::PwmExt, pwm::Mode, pwm::setup, pwm::resto};

use embedded_hal::digital::OutputPin;
use embedded_hal::digital::InputPin;
use embedded_hal::Pwm;

pub fn gpio_test() {
    let hw = stm32l0x1::Peripherals::take().unwrap();

    let mut rcc = hw.RCC.constrain();
    
    let mut gpioa = hw.GPIOA.split(&mut rcc.iop);

    let mut tim2 = hw.TIM2.split(&mut rcc.apb1);

    tim2.enable(());
    tim2.set_duty((),());

/*     setup(&mut tim2, Mode::PwmEgdeAligned);
    resto(&mut tim2);
 */
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