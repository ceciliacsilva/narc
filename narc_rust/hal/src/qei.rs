use core::u16;

use embedded_hal::{Qei as QeiExt, Direction};

use stm32l052::{TIM2, TIM22};

use gpio::gpioa::{PA0, PA1, PA6, PA7};
use gpio::{AF5};
use rcc::{APB1, APB2};

pub struct Qei<TIM, PINS> {
    tim: TIM,
    pins: PINS
}

pub trait Pins<Tim> {}

impl Pins<TIM22> for (PA6<AF5>, PA7<AF5>) {}

pub trait QeiFunc: Sized {
    type tim;
    type apb;

    fn qei<PINS>(self, pins: PINS, apb: &mut Self::apb) -> Qei<Self::tim, PINS>
    where PINS: Pins<Self>;
}

impl QeiFunc for TIM22{
    type tim = Self;
    type apb = APB2;

    fn qei<PINS>(self, pins: PINS, apb: &mut Self::apb) -> Qei<Self::tim, PINS>
    where 
        PINS: Pins<TIM22> 
    {
        Qei::_tim22(self, pins, apb)
    }
}

macro_rules! hal {
    ($($TIMX:ident: ($timX:ident, $APBX:ident, $timXen:ident, $timXrst:ident),)*) => {
        $(
            impl<PINS> Qei<$TIMX, PINS> {
                fn $timX(tim: $TIMX, pins: PINS, apb: &mut $APBX) -> Self {
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
                                            .cc1np().clear_bit()
                                            .cc2e().set_bit()
                                            .cc2p().clear_bit()
                                            .cc2np().clear_bit());

                    // Encoder mode 3
                    tim.smcr.modify(|_, w| unsafe { w.sms().bits(0b011) });
                    tim.psc.modify(|_, w| unsafe { w.psc().bits(0) });
                    tim.arr.modify(|_, w| unsafe { w.arr().bits(u16::MAX) });
                    tim.cr1.write(|w| w.cen().set_bit());

                    Qei { tim, pins }
                }

                pub fn release(self) -> ($TIMX, PINS) {
                    (self.tim, self.pins)
                }

                pub fn reset(&self) {
                    self.tim.cnt.write(|w| unsafe{ w.cnt().bits(0) });
                }
            }

            impl<PINS> QeiExt for Qei<$TIMX, PINS> {
                type Count = u16;

                fn count(&self) -> Self::Count {
                    self.tim.cnt.read().cnt().bits()
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
    // TIM2: (_tim2, APB1, tim2en, tim2rst),
    TIM22: (_tim22, APB2, tim22en, tim22rst),
}