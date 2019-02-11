//! Timers

use cast::{u16, u32};
use embedded_hal::timer::{CountDown, Periodic};
use nb;
use stm32l052::{TIM6, TIM2};
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

pub trait TimerExt: Sized {
    type apb;

    fn timer<T>(self, timeout: T, clocks: Clocks, apb: &mut Self::apb) -> Timer<Self> where T: Into<Hertz>;
}

impl TimerExt for TIM6 {

    type apb = APB1;

    fn timer<T>(self, timeout: T, clocks: Clocks, apb: &mut Self::apb) -> Timer<Self>
    where
        T: Into<Hertz>
    {
        Timer::_tim6(self, timeout, clocks, apb)
    }
}

impl TimerExt for TIM2 {
    type apb = APB1;

    fn timer<T>(self, timeout: T, clocks: Clocks, apb: &mut Self::apb) -> Timer<Self>
    where
        T: Into<Hertz>
    {
        Timer::_tim2(self, timeout, clocks, apb)
    }
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
                    self.tim.egr.write(|w| w.ug().set_bit());
                    self.tim.cr1.modify(|_, w| w.urs().set_bit());

                    self.timeout = timeout.into();
                    let freq = self.timeout.0;
                    let clk = self.clocks.pclk1().0 * if self.clocks.ppre1() == 1 { 1 } else { 2 };
                    let ticks = clk / freq;
                    let psc = u16(ticks / (1 << 16)).unwrap();
                    let psc = psc as u32;
                    self.tim.psc.write(|w| unsafe { w.bits(psc) });
                    let arr = u16(ticks / u32(psc + 1)).unwrap();
                    let arr = arr as u32;
                    self.tim.arr.write(|w| unsafe { w.bits(arr) });

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
                fn $tim<T>(tim: $TIM, timeout: T, clocks: Clocks, apb1: &mut APB1) -> Self
                where
                    T: Into<Hertz>
                {
                    apb1.enr().modify(|_, w| w.$timXen().set_bit());
                    apb1.rstr().modify(|_, w| w.$timXrst().set_bit());
                    apb1.rstr().modify(|_, w| w.$timXrst().clear_bit());

                    let mut timer = Timer {
                        clocks,
                        tim: tim,
                        timeout: Hertz(1),
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

                pub fn clear_it(&mut self) {
                    self.tim.sr.modify(|_, w| w.uif().clear_bit());
                }

                pub fn free(self) -> $TIM {
                    self.tim.cr1.modify(|_, w| w.cen().clear_bit());
                    self.tim
                }
            }
        )+
    }
}

/// TIM21-22 not supported, ppre1.
hal! {
    TIM6: (_tim6, tim6en, tim6rst),
    TIM2: (_tim2, tim2en, tim2rst),
}
