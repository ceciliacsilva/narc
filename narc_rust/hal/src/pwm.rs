use core::marker::PhantomData;

use stm32l0::stm32l0x1::{TIM2, tim2};

use gpio::{AF2};
use rcc::{APB1};

pub trait PwmExt {
    type Parts;

    fn split(self, apb1: &mut APB1) -> Self::Parts;
}

pub struct Parts{
    ccmr1_output: CCMR1_OUTPUT,
    psc: PSC,
    arr: ARR,
}

pub struct CCMR1_OUTPUT {
    _0: (),
}

impl CCMR1_OUTPUT {
    pub(crate) fn ccmr1_output(&mut self) -> &tim2::CCMR1_OUTPUT {
        unsafe{ &(*TIM2::ptr()).ccmr1_output }
    }
}


pub struct PSC {
    _0: (),   
}

impl PSC {
    pub(crate) fn psc(&mut self) -> &tim2::PSC {
        unsafe{ &(*TIM2::ptr()).psc }
    }
}

pub struct ARR {
    _0: (),
}

impl ARR {
    pub(crate) fn arr(&mut self) -> &tim2::ARR {
        unsafe{ &(*TIM2::ptr()).arr }
    }
}

impl PwmExt for TIM2 {
    type Parts = Parts;

    fn split(self, apb1: &mut APB1) -> Self::Parts {
        apb1.enr().modify(|_, w| w.tim2en().set_bit());
        apb1.rstr().modify(|_, w| w.tim2rst().set_bit());
        apb1.rstr().modify(|_, w| w.tim2rst().clear_bit());

        Parts{
            ccmr1_output: CCMR1_OUTPUT { _0: () },
            psc: PSC { _0: () },
            arr: ARR { _0: () },
        }
    }
}

