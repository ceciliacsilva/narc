use stm32l0::stm32l0x1;

pub fn gpio_test() {
    let mut peripherals = stm32l0x1::Peripherals::take().unwrap();

    let rcc = &peripherals.RCC;

    rcc.iopenr.modify(|_, w| w.iopaen().set_bit());

    let gpioa = &peripherals.GPIOA;

    gpioa.moder.modify(|_, w| unsafe{ w.mode4().bits(0) } );
    gpioa.pupdr.modify(|_, w| unsafe { w.pupd4().bits(1) });

    gpioa.moder.modify(|_, w| unsafe{ w.mode5().bits(1) });

    gpioa.odr.modify(|_, w| w.od5().set_bit());

    loop{
        let button = gpioa.idr.read().id4();

        if !button.bit_is_set() {
            gpioa.odr.modify(|_, w| w.od5().set_bit());
        } else {
            gpioa.odr.modify(|_, w| w.od5().clear_bit());
        }    
    
    }
}

