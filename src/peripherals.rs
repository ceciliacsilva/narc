


use stm32l0::stm32l0x1;

pub fn gpio_test() -> bool {
    let mut peripherals = stm32l0x1::Peripherals::take().unwrap();

    let rcc = &peripherals.RCC;

    rcc.iopenr.modify(|_, w| w.iopaen().set_bit());

    let gpioa = &peripherals.GPIOA;

    gpioa.moder.modify(|_, w| unsafe{ w.mode4().bits(0) } );
    let button = gpioa.idr.read().id4();

    gpioa.moder.modify(|_, w| unsafe{ w.mode5().bits(1) });
    gpioa.odr.modify(|_, w| w.od5().set_bit());

    button.bit_is_set()
}

pub fn set_led() {
    let mut peripherals = stm32l0x1::Peripherals::take().unwrap();

    let gpioa = &peripherals.GPIOA;

    gpioa.odr.modify(|_, w| w.od5().set_bit());
}

pub fn reset_led() {
    let mut peripherals = stm32l0x1::Peripherals::take().unwrap();

    let gpioa = &peripherals.GPIOA;

    gpioa.odr.modify(|_, w| w.od5().clear_bit());
}