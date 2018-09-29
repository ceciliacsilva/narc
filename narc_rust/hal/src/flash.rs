//! Flash memory.

/// Flash abstraction.

use stm32l0::stm32l0x1::{FLASH, flash};

pub trait FlashExt {
    fn constraint(self) -> Parts;
}

impl FlashExt for FLASH {
    fn constraint(self) -> Parts {
        Parts {
            acr: ACR { _0: ()},
        }
    }
}

pub struct Parts {
    pub acr: ACR,
}

pub struct ACR {
    _0: (),
}

impl ACR {
    pub(crate) fn acr(&mut self) -> &flash::ACR {
        unsafe{ &(*FLASH::ptr()).acr }
    }
}