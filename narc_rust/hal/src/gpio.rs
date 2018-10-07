//! General Purpose Input Output

// It should provide the same interface as the program in C.
// TODO: Assert that

use core::marker::PhantomData;
use rcc::IOP;

use embedded_hal::digital::{OutputPin, InputPin};

pub trait GpioExt {
    type Parts;

    fn split(self, iop: &mut IOP) -> Self::Parts;
}

/// Digital Input Mode.
pub struct InputDigital;
pub struct Input<MODE> {
    _mode: PhantomData<MODE>,
}
/// Input Mode types. 
pub struct PullDown;
pub struct PullUp;

/// Digital Output Mode.
pub struct OutputDigital;
pub struct Output<MODE> {
    _mode: PhantomData<MODE>,
}
/// Output Mode types.
pub struct PushPull;
pub struct OpenDrain;

/// Alternate Function Mode.
pub struct Alternate;
/// Alternate function types
pub struct AF0;
pub struct AF1;
pub struct AF2;
pub struct AF3;
pub struct AF4;
pub struct AF5;
pub struct AF6;
pub struct AF7;
pub struct AF8;
pub struct AF9;
pub struct AF10;
pub struct AF11;
pub struct AF12;
pub struct AF13;
pub struct AF14;
pub struct AF15;

pub struct Analog;

macro_rules! gpio {
    ($GPIOX:ident, $gpiox:ident, $gpioy:ident, $iopxenr:ident, $iopxrst:ident, $PXx:ident, [
        $($PXi:ident: ($pxi:ident, $i:expr, $MODE:ty, $CR:ident),)+
    ]) => {
        pub mod $gpiox {
            use core::marker::PhantomData;

            use stm32l0::stm32l0x1::{$gpioy, $GPIOX};
            use rcc::IOP;

            use super::*;

            pub struct Parts {
                pub moder: MODER,
                pub otyper: OTYPER,
                pub pupdr: PUPDR,
                pub afrh: AFRH,
                pub afrl: AFRL,
                $(
                    pub $pxi: $PXi<$MODE>,
                )+
            }

            impl GpioExt for $GPIOX {
                type Parts = Parts;

                fn split (self, iop: &mut IOP) -> Parts {
                    iop.enr().modify(|_, w| w.$iopxenr().set_bit());
                    iop.rstr().modify(|_, w| w.$iopxrst().set_bit());
                    iop.rstr().modify(|_, w| w.$iopxrst().clear_bit());

                    Parts {
                        moder: MODER { _0: () },
                        otyper: OTYPER { _0: () },
                        pupdr: PUPDR { _0: () },
                        afrl: AFRL { _0: () },
                        afrh: AFRH { _0: () },
                        $(
                            $pxi: $PXi { _mode: PhantomData },
                        )+
                    }
                }
            }

            pub struct MODER {
                _0: (),
            }

            impl MODER {
                pub(crate) fn moder(&mut self) -> &$gpioy::MODER {
                    unsafe { &(*$GPIOX::ptr()).moder }
                }
            }

            pub struct OTYPER {
                _0: (),
            }

            impl OTYPER {
                pub(crate) fn otyper(&mut self) -> &$gpioy::OTYPER {
                    unsafe { &(*$GPIOX::ptr()).otyper }
                }
            }
            
            pub struct PUPDR {
                _0: (),
            }

            impl PUPDR {
                pub(crate) fn pupdr(&mut self) -> &$gpioy::PUPDR {
                    unsafe { &(*$GPIOX::ptr()).pupdr }
                }
            }

            pub struct AFRH {
                _0: (),
            }

            impl AFRH {
                pub(crate) fn afr(&mut self) -> &$gpioy::AFRH {
                    unsafe { &(*$GPIOX::ptr()).afrh }
                }
            }

            pub struct AFRL {
                _0: (),
            }

            impl AFRL {
                pub(crate) fn afr(&mut self) -> &$gpioy::AFRL {
                    unsafe { &(*$GPIOX::ptr()).afrl }
                }
            }

            pub struct $PXx<MODE> {
                _mode: PhantomData<MODE>,
            }

            $(
                /// Pin
                pub struct $PXi<MODE> {
                    _mode: PhantomData<MODE>,
                }

                impl $PXi<OutputDigital> {
                    pub fn push_pull(&self, otyper: &mut OTYPER) -> $PXi<Output<PushPull>>{
                        let output_type = 0b0;
                        otyper
                            .otyper()
                            .modify(|r, w| unsafe { w.bits(r.bits() & !(output_type << $i)) });

                        $PXi { _mode: PhantomData }
                    }
                    pub fn open_drain(&self, otyper: &mut OTYPER) -> $PXi<Output<PushPull>>{
                        let output_type = 0b1;
                        otyper
                            .otyper()
                            .modify(|r, w| unsafe { w.bits(r.bits() & !(output_type << $i)) });

                        $PXi { _mode: PhantomData }
                    }
                }

                impl $PXi<Alternate> {
                    //TODO all others.
                    pub fn af5(&self, afrl: &mut AFRL) -> $PXi<AF5> {
                        let af = 5;
                        let offset = 4 * ($i % 8);

                        afrl.afr().modify(|r, w| unsafe {
                            w.bits((r.bits() & !(0b1111 << offset)) | (af << offset))
                        });

                        $PXi { _mode: PhantomData }
                    }
                }

                impl $PXi<InputDigital> {
                    pub fn pull_up(&self, pupdr: &mut PUPDR) -> $PXi<Input<PullUp>>{
                        let offset = 2 * $i;

                        let pull_type = 0b01;
                        pupdr.pupdr().modify(|r, w| unsafe {
                            w.bits((r.bits() & !(0b11 << offset)) | (pull_type << offset))
                        });

                        $PXi { _mode: PhantomData }
                    }
                    pub fn pull_down(&self, pupdr: &mut PUPDR) -> $PXi<Input<PullDown>>{
                        let offset = 2 * $i;

                        let pull_type = 0b10;
                        pupdr.pupdr().modify(|r, w| unsafe {
                            w.bits((r.bits() & !(0b11 << offset)) | (pull_type << offset))
                        });

                        $PXi { _mode: PhantomData }
                    }
                }

                impl<MODE> $PXi<MODE> {
                    // TODO all modes.
                    pub fn into_output (self, moder: &mut MODER) -> $PXi<OutputDigital> {
                        let offset = 2 * $i;

                        let mode = 0b01;
                        moder.moder().modify(|r, w| unsafe {
                            w.bits((r.bits() & !(0b11 << offset)) | (mode << offset)) });

                        $PXi { _mode: PhantomData }
                    }

                    pub fn into_input (self, moder: &mut MODER) -> $PXi<InputDigital> {
                        let offset = 2 * $i;

                        let mode = 0b11;
                        moder.moder().modify(|r, w| unsafe {
                            w.bits((r.bits() & !(0b11 << offset)) | (mode << offset)) });

                        $PXi { _mode: PhantomData }
                    }

                    pub fn into_alternate (self, moder: &mut MODER) -> $PXi<Alternate> {
                        let offset = 2 * $i;

                        // alternative function
                        let mode = 0b10;
                        moder.moder().modify(|r, w| unsafe{
                            w.bits((r.bits() & !(0b11 << offset)) | (mode << offset))
                        });

                        $PXi { _mode: PhantomData }
                    }

                    /// PUPD(i) = 00, os estados são reservados.
                    pub fn into_analog (self, moder: &mut MODER, pupdr: &mut PUPDR) -> $PXi<Analog> {
                        let offset = 2 * $i;

                        // Analog mode
                        let mode = 0b11;
                        moder.moder().modify(|r, w| unsafe{
                            w.bits((r.bits() & !(0b11 << offset)) | (mode << offset))
                        });

                        let pull_type = 0b00;
                        pupdr.pupdr().modify(|r, w| unsafe {
                            w.bits((r.bits() & !(0b11 << offset)) | (pull_type << offset))
                        });

                        $PXi { _mode: PhantomData }
                    }
                }
                
                impl<MODE> OutputPin for $PXi<Output<MODE>> {
                    fn set_high(&mut self) {
                        unsafe { (*$GPIOX::ptr()).bsrr.write(|w| w.bits(1 << $i)) }
                    }

                    fn set_low(&mut self) {
                        unsafe { (*$GPIOX::ptr()).bsrr.write(|w| w.bits(1 << (16 + $i))) }
                    }
                }
                
                impl<MODE> InputPin for $PXi<Input<MODE>> {
                    fn is_high(&self) -> bool {
                        !self.is_low()
                    }

                    fn is_low(&self) -> bool {
                        unsafe { (*$GPIOX::ptr()).idr.read().bits() & (1 << $i) == 0 }
                    }
                }
            )+
        }   
    };
}

gpio!(GPIOA, gpioa, gpioa, iopaen, ioparst, PAx, [
    PA0: (pa0, 0, Analog, AFRL),
    PA1: (pa1, 1, Analog, AFRL),
    PA2: (pa2, 2, Analog, AFRL),
    PA3: (pa3, 3, Analog, AFRL),
    PA4: (pa4, 4, Analog, AFRL),
    PA5: (pa5, 5, Analog, AFRL),
    PA6: (pa6, 6, Analog, AFRL),
    PA7: (pa7, 7, Analog, AFRL),
    PA8: (pa8, 8, Analog, AFRH),
    PA9: (pa9, 9, Analog, AFRH),
    PA10: (pa10, 10, Analog, AFRH),
    PA11: (pa11, 11, Analog, AFRH),
    PA12: (pa12, 12, Analog, AFRH),
    PA13: (pa13, 13, Analog, AFRH),
    PA14: (pa14, 14, Analog, AFRH),
    PA15: (pa15, 15, Analog, AFRH),
]);