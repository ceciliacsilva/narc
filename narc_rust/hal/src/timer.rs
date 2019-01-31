//! Timers

use cast::{u16, u32};
use embedded_hal::timer::{CountDown, Periodic};
use nb;
use stm32l052::{TIM6};
use void::Void;

use rcc::{APB1, Clocks};
use time::Hertz;

pub struct Timer<TIM> {
    clocks: Clocks,
    tim: TIM,
    timeout: Hertz,
}

pub enum Event {
    TimeOut,
}


macro_rules! hal {
    ($($TIM:ident: ($tim:ident, $timXen:ident, $timXrst:ident), )+) => {
        $(
            impl Periodic for Timer<$TIM> {}

            impl CountDown for Timer<$TIM> {
                type Time = Hertz;

                fn start<T>(&mut self, timeout: T)
                where
                    T: Into<Hertz>
                {
                    self.tim.cr1.modify(|_, w| w.cen().clear_bit());

                    self.tim.cnt.reset();

                    self.timeout = timeout.into();

                    let frequency = self.timeout.0;

                    let ticks = self.clocks.pclk1().0 * if self.clocks.ppre1() == 1 { 1 } else { 2 }
                               / frequency;

                    let psc = u16((ticks - 1) / (1 << 16)).unwrap();

                    self.tim.psc.write(|w| unsafe { w.psc().bits(psc) });

                    let arr = u16(ticks / u32(psc + 1)).unwrap();

                    self.tim.arr.write(|w| unsafe { w.arr().bits(arr) });

                    self.tim.cr1.modify(|_, w| w.cen().set_bit());
                }

                fn wait(&mut self) -> nb::Result<(), Void> {
                    if self.tim.sr.read().uif().bit_is_clear() {
                        Err(nb::Error::WouldBlock)
                    } else {
                        self.tim.sr.modify(|_, w| w.uif().clear_bit());
                        Ok(())
                    }
                }
            }

            impl Timer<$TIM> {
                pub fn $tim<T>(tim: $TIM, timeout: T, clocks: Clocks, apb1: &mut APB1) -> Self
                where
                    T: Into<Hertz>
                {
                    apb1.enr().modify(|_, w| w.$timXen().set_bit());
                    apb1.rstr().modify(|_, w| w.$timXrst().set_bit());
                    apb1.rstr().modify(|_, w| w.$timXrst().clear_bit());

                    let mut timer = Timer {
                        clocks,
                        tim,
                        timeout: Hertz(0),
                    };

                    timer.start(timeout);

                    timer
                }

                pub fn listen(&mut self, event: Event) {
                    match event {
                        Event::TimeOut => {
                            self.tim.dier.write(|w| w.uie().set_bit());
                        }
                    }
                }

                pub fn unlisten(&mut self, event: Event) {
                    match event {
                        Event::TimeOut => {
                            self.tim.dier.write(|w| w.uie().clear_bit());
                        }
                    }
                }

                pub fn free(self) -> $TIM {
                    self.tim.cr1.modify(|_, w| w.cen().clear_bit());
                    self.tim
                }
            }
        )+
    }
}

hal! {
    TIM6: (tim6, tim6en, tim6rst),
}
