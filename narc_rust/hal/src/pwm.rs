use core::marker::PhantomData;
use core::mem;

use cast::{u16, u32};
use embedded_hal::{PwmPin};
use stm32l0::stm32l0x1::{TIM2};

use gpio::gpioa::{PA0, PA1, PA2, PA3, PA5, PA15};
use gpio::{AF2, AF5};
use rcc::{APB1, Clocks};
use time::Hertz;

pub trait Pins<TIM> {
    const C1: bool;
    const C2: bool;
    const C3: bool;
    const C4: bool;
    type Channels;
}

impl Pins<TIM2> for (PA5<AF5>) {
    const C1: bool = true;
    const C2: bool = false;
    const C3: bool = false;
    const C4: bool = false;
    type Channels = Pwm<TIM2, C1>;
}

impl Pins<TIM2> for (PA15<AF5>) {
    const C1: bool = true;
    const C2: bool = false;
    const C3: bool = false;
    const C4: bool = false;
    type Channels = Pwm<TIM2, C1>;
}

impl Pins<TIM2> for
(PA0<AF2>, PA1<AF2>, PA2<AF2>, PA3<AF2>){
    const C1: bool= true;
    const C2: bool= true;
    const C3: bool= true;
    const C4: bool= true;
    type Channels = (Pwm<TIM2, C1>, Pwm<TIM2, C2>, Pwm<TIM2, C3>, Pwm<TIM2, C4>);
}

pub trait PwmExt: Sized {
    fn pwm<PINS, T>(
        self,
        PINS,
        frequency: T,
        clocks: Clocks,
        apb: &mut APB1,
    ) -> PINS::Channels
    where
        PINS: Pins<Self>,
        T: Into<Hertz>;
}

impl PwmExt for TIM2 {
    fn pwm<PINS, T>(
        self,
        _pins: PINS,
        freq: T,
        clocks: Clocks,
        apb: &mut APB1,
    ) -> PINS::Channels
    where
        PINS: Pins<Self>,
        T: Into<Hertz>,
    {
        tim2(self, _pins, freq.into(), clocks, apb)
    }
}

pub struct Pwm<TIM, CHANNEL> {
    _channel: PhantomData<CHANNEL>,
    _tim: PhantomData<TIM>,
}

pub struct C1;
pub struct C2;
pub struct C3;
pub struct C4;

macro_rules! hal {
    ($($TIMX:ident: ($timX:ident, $timXen:ident, $timXrst:ident),)+) => {
        $(
            fn $timX<PINS>(
                tim: $TIMX,
                _pins: PINS,
                freq: Hertz,
                clocks: Clocks,
                apb: &mut APB1,
            ) -> PINS::Channels
            where
                PINS: Pins<$TIMX>,
            {
                apb.enr().modify(|_, w| w.$timXen().set_bit());
                apb.rstr().modify(|_, w| w.$timXrst().set_bit());
                apb.rstr().modify(|_, w| w.$timXrst().clear_bit());

                let pwm1 = 0b110;

                if PINS::C1 {
                    tim.ccmr1_output
                        .modify(|_, w| unsafe{ w.oc1pe().set_bit().oc1m().bits(pwm1) });
                }

                if PINS::C2 {
                    tim.ccmr1_output
                        .modify(|_, w| unsafe{ w.oc2pe().set_bit().oc2m().bits(pwm1) });
                }

                if PINS::C3 {
                    tim.ccmr2_output
                        .modify(|_, w| unsafe{ w.oc3pe().set_bit().oc3m().bits(pwm1) });
                }

                if PINS::C4 {
                    tim.ccmr2_output
                        .modify(|_, w| unsafe{ w.oc4pe().set_bit().oc4m().bits(pwm1) });
                }

                tim.cr1.write(|w| w.cen().set_bit()); 

                tim.egr.write(|w| w.ug().set_bit());
                
                let clk = clocks.pclk1().0 * if clocks.ppre1() == 1 { 1 } else { 2 };
                let freq = freq.0;
                let ticks = clk / freq;
                
                let psc = u16(ticks / (1 << 16)).unwrap();
                let psc = psc as u32;
                tim.psc.write(|w| unsafe{w.bits(psc)} );
                
                let arr = u16(ticks / u32(psc + 1)).unwrap();
                let arr = arr as u32;
                tim.arr.write(|w| unsafe{w.bits(arr)} );

                unsafe { mem::uninitialized() }
            }

            impl PwmPin for Pwm<$TIMX, C1> {
                type Duty = u16;

                fn disable(&mut self) {
                    unsafe { (*$TIMX::ptr()).ccer.modify(|_, w| w.cc1e().clear_bit()) };
                }

                fn enable(&mut self) {
                    unsafe { (*$TIMX::ptr()).ccer.modify(|_, w| w.cc1e().set_bit()) };
                    unsafe { (*$TIMX::ptr()).ccr1.modify(|_, w| w.bits(0)) };
                }

                fn get_duty(&self) -> u16 {
                    (unsafe { (*$TIMX::ptr()).ccr1.read().bits() }) as u16
                }

                fn get_max_duty(&self) -> u16 {
                    (unsafe { (*$TIMX::ptr()).arr.read().arr_l().bits() })
                }

                fn set_duty(&mut self, duty: u16) {
                    let duty = duty as u32;
                    unsafe { (*$TIMX::ptr()).ccr1.modify(|_, w| w.bits(duty)) };
                }
            }

            impl PwmPin for Pwm<$TIMX, C2> {
                type Duty = u16;

                fn disable(&mut self) {
                    unsafe { (*$TIMX::ptr()).ccer.modify(|_, w| w.cc2e().set_bit()) };
                }

                fn enable(&mut self) {
                    unsafe { (*$TIMX::ptr()).ccer.modify(|_, w| w.cc2e().set_bit()) };
                    unsafe { (*$TIMX::ptr()).ccr2.modify(|_, w| w.bits(0)) };
                }

                fn get_duty(&self) -> u16 {
                    (unsafe { (*$TIMX::ptr()).ccr2.read().bits() }) as u16
                }

                fn get_max_duty(&self) -> u16 {
                    (unsafe { (*$TIMX::ptr()).arr.read().arr_l().bits() })
                }

                fn set_duty(&mut self, duty: u16) {
                    let duty = duty as u32;
                    unsafe { (*$TIMX::ptr()).ccr2.modify(|_, w| w.bits(duty)) };
                }
            }

            impl PwmPin for Pwm<$TIMX, C3> {
                type Duty = u16;

                fn disable(&mut self) {
                    unsafe { (*$TIMX::ptr()).ccer.modify(|_, w| w.cc3e().set_bit()) };
                }

                fn enable(&mut self) {
                    unsafe { (*$TIMX::ptr()).ccer.modify(|_, w| w.cc3e().set_bit()) };
                    unsafe { (*$TIMX::ptr()).ccr3.modify(|_, w| w.bits(0)) };
                }

                fn get_duty(&self) -> u16 {
                    (unsafe { (*$TIMX::ptr()).ccr3.read().bits() }) as u16
                }

                fn get_max_duty(&self) -> u16 {
                    (unsafe { (*$TIMX::ptr()).arr.read().arr_l().bits() })
                }

                fn set_duty(&mut self, duty: u16) {
                    let duty = duty as u32;
                    unsafe { (*$TIMX::ptr()).ccr3.modify(|_, w| w.bits(duty)) };
                }
            }

            impl PwmPin for Pwm<$TIMX, C4> {
                type Duty = u16;

                fn disable(&mut self) {
                    unsafe { (*$TIMX::ptr()).ccer.modify(|_, w| w.cc4e().set_bit()) };
                }

                fn enable(&mut self) {
                    unsafe { (*$TIMX::ptr()).ccer.modify(|_, w| w.cc4e().set_bit()) };
                    unsafe { (*$TIMX::ptr()).ccr4.modify(|_, w| w.bits(0)) };
                }

                fn get_duty(&self) -> u16 {
                    (unsafe { (*$TIMX::ptr()).ccr4.read().bits() }) as u16
                }

                fn get_max_duty(&self) -> u16 {
                    (unsafe { (*$TIMX::ptr()).arr.read().arr_l().bits() })
                }

                fn set_duty(&mut self, duty: u16) {
                    let duty = duty as u32;
                    unsafe { (*$TIMX::ptr()).ccr4.modify(|_, w| w.bits(duty)) };
                }
            }
        )+
    }
}

hal! {
    TIM2: (tim2, tim2en, tim2rst),
}
