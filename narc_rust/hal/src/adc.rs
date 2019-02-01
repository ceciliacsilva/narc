use stm32l052::ADC;
use rcc::APB2;
use gpio::{Analog, gpioa::PA2};
pub trait Pin<Adc> {}

impl Pin<ADC> for PA2<Analog> {}

pub trait AdcExt:Sized {
    fn read(&self) -> u32;
    fn config<PIN>(&mut self, _pin: PIN, apb2: &mut APB2)
        where PIN: Pin<Self>;
}

impl AdcExt for ADC {
    fn read(&self) -> u32 {
        self.dr.read().bits()
    }

    fn config<PIN>(&mut self, _pin: PIN, apb2: &mut APB2)
    where
        PIN: Pin<Self>
    {
        apb2.enr().modify(|_, w| w.adcen().set_bit());
        apb2.rstr().modify(|_, w| w.adcrst().set_bit());
        apb2.rstr().modify(|_, w| w.adcrst().clear_bit());

        //div1
        self.cfgr2.modify(|_, w| unsafe{ w.ckmode().bits(0b11) });

        if self.cr.read().aden().bit() {
            self.cr.modify(|_, w| w.aden().clear_bit());
        }

        self.cr.modify(|_, w| w.adcal().set_bit());

        while self.isr.read().eocal().bit() == false {}

        // self.isr.modify(|_, w| w.eocal().set_bit());
        self.isr.modify(|_, w| w.adrdy().set_bit());

        self.cr.modify(|_, w| w.aden().set_bit());

        self.cfgr1.modify(|_, w| w.cont().set_bit());

        self.smpr.modify(|_, w| unsafe{ w.smpr().bits(0b000) });
        self.chselr.modify(|_, w| w.chsel2().set_bit());
        self.cfgr1.modify(|_, w| unsafe{ w.exten().bits(0b00) });

        // self.ccr.modify(|_, w| w.tsen().set_bit());
        self.cr.modify(|_, w| w.adstart().set_bit());

        // while self.isr.read().eoc().bit() == false {}
    }
}



