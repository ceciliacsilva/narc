#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate narc_hal;
extern crate embedded_hal;

use narc_hal::stm32l052;
use narc_hal::gpio::GpioExt;
use narc_hal::rcc::RccExt;
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

    led.set_high();
    loop{
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
    }
}
