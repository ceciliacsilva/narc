use stm32l0::stm32l0x1;

use hal::{rcc::RccExt, gpio::GpioExt};
use hal::gpio::gpioa::OutputPin;

pub fn gpio_test() {
    let hw = stm32l0x1::Peripherals::take().unwrap();

    let mut rcc = hw.RCC.constrain();
    
    let mut gpioa = hw.GPIOA.split(&mut rcc.iop);

    let mut led = gpioa.pa5.into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);

    led.set();
}