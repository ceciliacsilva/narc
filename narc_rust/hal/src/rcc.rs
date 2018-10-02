//! Reset and Clock Control

use stm32l0::stm32l0x1::{RCC, rcc};

pub trait RccExt {
    /// Constrains the 'Rcc' peripheral so it play nicely with other abstractions.
    fn constrain(self) -> Rcc;
}

impl RccExt for RCC {
    fn constrain(self) -> Rcc {
        Rcc {
            iop: IOP { _0: () },
            apb1: APB1 { _0: () },
        }
    }
}

// TODO. It is not complete.
pub struct Rcc {
    // TODO AHB
    pub apb1: APB1,
    // TODO APB2
    pub iop: IOP,
    // TODO Clock Configuration Register CFGR
}

pub struct IOP {
    _0: (),
}

impl IOP {
    pub(crate) fn enr (&mut self) -> &rcc::IOPENR {
        unsafe { &(*RCC::ptr()).iopenr }
    }

    pub(crate) fn rstr (&mut self) -> &rcc::IOPRSTR {
        unsafe { &(*RCC::ptr()).ioprstr }
    }
}

pub struct APB1 {
    _0: (),
}

impl APB1 {
    pub(crate) fn enr (&mut self) -> &rcc::APB1ENR {
        unsafe { &(*RCC::ptr()).apb1enr }
    }

    pub(crate) fn rstr (&mut self) -> &rcc::APB1RSTR {
        unsafe { &(*RCC::ptr()).apb1rstr }
    }
}