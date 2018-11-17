use core::u16;

use embedded_hal::{Qei as QeiExt, Direction};

use stm32l052::{TIM2};

use gpio::gpioa::{PA0, PA1};
use gpio::{Analog};
use rcc::{APB1};

pub trait Pins<Tim> {}

impl Pins<TIM2> for (PA0<Analog>, PA1<Analog>) {}

pub struct Qei<TIM, PINS> {
    tim: TIM,
    pins: PINS,
}

impl<PINS> Qei<TIM2, PINS> {
    pub fn tim2(tim: TIM2, pins: PINS, apb: &mut APB1) -> Self 
    where 
        PINS: Pins<TIM2> 
    {
        Qei::_tim2(tim, pins, apb)
    }
}

macro_rules! hal {
    ($($TIMX:ident: ($timX:ident, $timXen:ident, $timXrst:ident),)*) => {
        $(
            impl<PINS> Qei<$TIMX, PINS> {
                fn $timX(tim: $TIMX, pins: PINS, apb: &mut APB1) -> Self {
                    apb.enr().modify(|_, w| w.$timXen().set_bit());
                    apb.rstr().modify(|_, w| w.$timXrst().set_bit());
                    apb.rstr().modify(|_, w| w.$timXrst().clear_bit());

                    // tim.ccmr1_input.modify(|_, w| unsafe { w
                    //                                         .cc1s().bits(0b01)
                    //                                         .cc2s().bits(0b01) });

                    tim.ccmr1_output.modify(|_, w| unsafe { w.bits({ (0b01 << 0) | (0b01 << 8)}) });
                    
                    tim.ccer.modify(|_, w| w
                                            .cc1e().set_bit()
                                            .cc1p().clear_bit()
                                            .cc2e().set_bit()
                                            .cc2p().clear_bit());

                    // Encoder mode 2
                    tim.smcr.modify(|_, w| unsafe { w.sms().bits(0b010) });
                    tim.arr.modify(|_, w| unsafe { w.arr_l().bits(u16::MAX) });
                    tim.cr1.write(|w| w.cen().set_bit());

                    Qei { tim, pins }
                }

                pub fn release(self) -> ($TIMX, PINS) {
                    (self.tim, self.pins)
                }
            }

            impl<PINS> QeiExt for Qei<$TIMX, PINS> {
                type Count = u16;

                fn count(&self) -> u16 {
                    self.tim.cnt.read().cnt_l().bits()
                }

                fn direction(&self) -> Direction {
                    if self.tim.cr1.read().dir().bit_is_clear() {
                        Direction::Upcounting
                    } else {
                        Direction::Downcounting
                    }
                }
            }
        )+        
    };
}

hal! {
    TIM2: (_tim2, tim2en, tim2rst),
}