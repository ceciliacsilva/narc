//! Reset and Clock Control

use stm32l0::stm32l0x1::{RCC, rcc};

pub trait RccExt {
    /// Constrains the 'Rcc' peripheral so it play nicely with other abstractions.
    fn constrain(self) -> Rcc;
}

impl RccExt for RCC {
    fn constrain(self) -> Rcc {
        Rcc {
            iop: IOP { _0: () }
        }
    }
}

// TODO. It is not complete.
pub struct Rcc {
    // TODO AHB
    // TODO APB1
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