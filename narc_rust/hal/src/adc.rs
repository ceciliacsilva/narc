use stm32l052::ADC;
use rcc::APB2;

use cortex_m::asm::bkpt;

pub fn adc_read(adc: &ADC) -> u32 {
    adc.dr.read().bits()
}

pub fn adc_config(apb2: &mut APB2, adc: &mut ADC){
    apb2.enr().modify(|_, w| w.adcen().set_bit());
    apb2.rstr().modify(|_, w| w.adcrst().set_bit());
    apb2.rstr().modify(|_, w| w.adcrst().clear_bit());    

    //div1
    adc.cfgr2.modify(|_, w| unsafe{ w.ckmode().bits(0b11) });

    if adc.cr.read().aden().bit() {
        adc.cr.modify(|_, w| w.aden().clear_bit());
    }

    adc.cr.modify(|_, w| w.adcal().set_bit());

    while adc.isr.read().eocal().bit() == false {}

    // adc.isr.modify(|_, w| w.eocal().set_bit());
    adc.isr.modify(|_, w| w.adrdy().set_bit());

    adc.cr.modify(|_, w| w.aden().set_bit());

    adc.cfgr1.modify(|_, w| w.cont().set_bit());

    adc.smpr.modify(|_, w| unsafe{ w.smpr().bits(0b000) });
    adc.chselr.modify(|_, w| w.chsel2().set_bit());
    adc.cfgr1.modify(|_, w| unsafe{ w.exten().bits(0b00) });

    // adc.ccr.modify(|_, w| w.tsen().set_bit());
    adc.cr.modify(|_, w| w.adstart().set_bit());

    // while adc.isr.read().eoc().bit() == false {}

    //     ADC1->CR2   |= ADC_CR2_ADON;
    //   ADC1->CR2   |= ADC_CR2_RSTCAL;
    //   ADC1->CR2   |= ADC_CR2_CAL;

    //   ADC1->CR2   |= ADC_CR2_EXTTRIG;
    //   ADC1->CR2   |= ADC_CR2_EXTSEL_0 | ADC_CR2_EXTSEL_1 | ADC_CR2_EXTSEL_2;

    //   ADC1->CR2   |= ADC_CR2_CONT;

    //   //ADC1->CR2   |= ADC_CR2_TSVREFE; //sensor-temp
    //   //ADC1->SQR3  |= ADC_SQR3_SQ1_4; //IN16

    //   //ADC1->SQR3  |= ADC_SQR3_SQ1_0 | ADC_SQR3_SQ1_2; //PA5-IN5

    //   ADC1->SQR3  |= ADC_SQR3_SQ1_3 | ADC_SQR3_SQ1_1; //PC0-IN10
    
    //   ADC1->CR2   |= ADC_CR2_SWSTART;
}

