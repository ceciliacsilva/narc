use core::marker::PhantomData;

use stm32l0::stm32l0x1::{TIM2, tim2};

use gpio::{AF2};
use rcc::{APB1};

use embedded_hal::{PwmPin, Pwm};

pub enum Mode {
    PwmEgdeAligned,
    PwmCenterAligned,
}

pub trait PwmExt {
    type Parts;

    fn split(self, apb1: &mut APB1) -> Self::Parts;

    // fn setup(&mut self, mode: Mode);
/* 
    fn enable(&mut self);

    fn disable(&mut self);

    fn set_prescaler(&mut self, psc: u16);

    fn set_arr(&mut self, arr: u16);

    fn set_duty(&mut self, duty: u16);

    fn get_duty(&self);

    fn get_duty_max(&self) -> u16; */
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

pub struct CR1 {
    _0: ()
}

impl CR1 {
    pub(crate) fn cr1(&mut self) -> &tim2::CR1 {
        unsafe{ &(*TIM2::ptr()).cr1 }
    }
}

pub struct CCR1 {
    _0: ()
}

impl CCR1 {
    pub(crate) fn ccr1(&mut self) -> &tim2::CCR1 {
        unsafe{ &(*TIM2::ptr()).ccr1 }
    }
}

pub struct EGR {
    _0: ()
}

impl EGR {
    pub(crate) fn egr(&mut self) -> &tim2::EGR {
        unsafe{ &(*TIM2::ptr()).egr }
    }
}

pub struct CCER {
    _0: ()
}

impl CCER {
    pub(crate) fn ccer(&mut self) -> &tim2::CCER {
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

impl Pwm for Parts {
    type Channel = ();
    type Time = ();
    type Duty = ();
    
    fn disable(&mut self, channel: Self::Channel){
        
    }

    
    fn enable(&mut self, channel: Self::Channel){
        self
            .ccmr1_output
            .ccmr1_output()
            .modify(|_, w| unsafe{ w.oc1m().bits(0b110) });

        self
            .ccmr1_output
            .ccmr1_output()
            .modify(|_, w| w.oc1pe().set_bit());

        self
            .ccer
            .ccer()
            .modify(|_, w| w.cc1e().set_bit());

        self
            .cr1
            .cr1()
            .modify(|_, w| w.cen().set_bit());

        self
            .egr
            .egr()
            .write(|w| w.ug().set_bit());

    }

    
    fn get_period(&self) -> Self::Time{
        ()
    }

    fn get_duty(&self, channel: Self::Channel) -> Self::Duty{
        ()
    }

    fn get_max_duty(&self) -> Self::Duty{
        ()
    }

    fn set_duty(&mut self, channel: Self::Channel, duty: Self::Duty){
        self
            .psc
            .psc()
            .modify(|_, w| unsafe{ w.bits(1599) });

        self
            .arr
            .arr()
            .modify(|_, w| unsafe{ w.bits(999) });

        self
            .ccr1
            .ccr1()
            .modify(|_, w| unsafe{ w.bits(500) });
    }

    fn set_period<P>(&mut self, period: P)
    where
        P: Into<Self::Time> {

    }
}


pub fn setup(tim: &mut Parts, _mode: Mode){
    tim
        .ccmr1_output
        .ccmr1_output()
        .modify(|_, w| unsafe{ w.oc1m().bits(0b110) });

    tim
        .ccmr1_output
        .ccmr1_output()
        .modify(|_, w| w.oc1pe().set_bit());

    tim
        .ccer
        .ccer()
        .modify(|_, w| w.cc1e().set_bit());

    tim
        .cr1
        .cr1()
        .modify(|_, w| w.cen().set_bit());

    tim
        .egr
        .egr()
        .write(|w| w.ug().set_bit());

}

pub fn resto(tim: &mut Parts){
    tim
        .psc
        .psc()
        .modify(|_, w| unsafe{ w.bits(1599) });

    tim
        .arr
        .arr()
        .modify(|_, w| unsafe{ w.bits(999) });

    tim
        .ccr1
        .ccr1()
        .modify(|_, w| unsafe{ w.bits(500) });
}