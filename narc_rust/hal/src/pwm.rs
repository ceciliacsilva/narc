use core::marker::PhantomData;

use stm32l0::stm32l0x1::{TIM2, tim2};

use gpio::{AF2};
use rcc::{APB1};

pub trait PwmExt {
    type Parts;

    fn split(self, apb1: &mut APB1) -> Self::Parts;
}

pub struct Parts{
    pub ccmr1_output: CCMR1_OUTPUT,
    pub psc: PSC,
    pub arr: ARR,
    pub cr1: CR1,
    pub ccr1: CCR1,
    pub egr: EGR,
    pub ccer: CCER,
}

pub struct CCMR1_OUTPUT {
    _0: (),
}

impl CCMR1_OUTPUT {
    pub/* (crate) */ fn ccmr1_output(&mut self) -> &tim2::CCMR1_OUTPUT {
        unsafe{ &(*TIM2::ptr()).ccmr1_output }
    }
}


pub struct PSC {
    _0: (),   
}

impl PSC {
    pub/* (crate) */ fn psc(&mut self) -> &tim2::PSC {
        unsafe{ &(*TIM2::ptr()).psc }
    }
}

pub struct ARR {
    _0: (),
}

impl ARR {
    pub/* (crate) */ fn arr(&mut self) -> &tim2::ARR {
        unsafe{ &(*TIM2::ptr()).arr }
    }
}

pub struct CR1 {
    _0: ()
}

impl CR1 {
    pub/* (crate) */ fn cr1(&mut self) -> &tim2::CR1 {
        unsafe{ &(*TIM2::ptr()).cr1 }
    }
}

pub struct CCR1 {
    _0: ()
}

impl CCR1 {
    pub/* (crate) */ fn ccr1(&mut self) -> &tim2::CCR1 {
        unsafe{ &(*TIM2::ptr()).ccr1 }
    }
}

pub struct EGR {
    _0: ()
}

impl EGR {
    pub/* (crate) */ fn egr(&mut self) -> &tim2::EGR {
        unsafe{ &(*TIM2::ptr()).egr }
    }
}

pub struct CCER {
    _0: ()
}

impl CCER {
    pub/* (crate) */ fn ccer(&mut self) -> &tim2::CCER {
        unsafe{ &(*TIM2::ptr()).ccer }
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
            cr1: CR1 { _0: () },
            ccr1: CCR1 { _0: () },
            egr: EGR { _0: () },
            ccer: CCER { _0: () },
        }
    }
}


