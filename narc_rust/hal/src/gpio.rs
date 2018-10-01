//! General Purpose Input Output

// It should provide the same interface as the program in C.
// TODO: Assert that

use core::marker::PhantomData;
use rcc::IOP;

pub trait GpioExt {
    type Parts;

    fn split(self, iop: &mut IOP) -> Self::Parts;
}

/// Input Mode.
pub struct Input<MODE> {
    _mode: PhantomData<MODE>,
}

/// Input Mode types. 
pub struct Floating;
pub struct PullDown;
pub struct PullUp;

pub struct Output<MODE> {
    _mode: PhantomData<MODE>,
}

/// Output Mode types.
pub struct PushPull;
pub struct OpenDrain;

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
                        afrh: AFRH { _0: () },
                        afrl: AFRL { _0: () },
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
                i: u8,
                _mode: PhantomData<MODE>,
            }

            // TODO Esse trait deveria ser importado do embedded hal, mas para seguir
            // a interface definida em C, estou criando um proprio.
            /// Digital Output Pin Interface.
            pub trait OutputPin {
                fn set (&mut self);
                fn reset (&mut self);
            }

            $(
                /// Pin
                pub struct $PXi<MODE> {
                    _mode: PhantomData<MODE>,
                }

                impl<MODE> $PXi<MODE> {
                    // TODO all modes.

                    pub fn into_pull_up_input (
                        self, 
                        moder: &mut MODER, 
                        pupdr: &mut PUPDR) 
                    -> $PXi<Input<PullUp>> {
                        
                        let offset = 2 * $i;

                        // input mode
                        moder
                            .moder()
                            .modify(|r, w| unsafe { w.bits(r.bits() & !(0b11 << offset)) });

                        // pull-up
                        pupdr.pupdr().modify(|r, w| unsafe {
                            w.bits((r.bits() & !(0b11 << offset)) | (0b01 << offset))
                        });

                        $PXi { _mode: PhantomData }
                    }

                    pub fn into_push_pull_output(
                        self,
                        moder: &mut MODER,
                        otyper: &mut OTYPER,
                    ) -> $PXi<Output<PushPull>> {
                        let offset = 2 * $i;

                        // output mode
                        let mode = 0b01;
                        moder.moder().modify(|r, w| unsafe {
                            w.bits((r.bits() & !(0b11 << offset)) | (mode << offset))
                        });

                        // push pull
                        otyper
                            .otyper()
                            .modify(|r, w| unsafe { w.bits(r.bits() & !(0b1 << $i)) });

                        $PXi { _mode: PhantomData }
                    }
                }

                impl<MODE> OutputPin for $PXi<Output<MODE>> {
                    fn set(&mut self) {
                        // NOTE(unsafe) atomic write to a stateless register
                        unsafe { (*$GPIOX::ptr()).bsrr.write(|w| w.bits(1 << $i)) }
                    }

                    fn reset(&mut self) {
                        // NOTE(unsafe) atomic write to a stateless register
                        unsafe { (*$GPIOX::ptr()).bsrr.write(|w| w.bits(1 << (16 + $i))) }
                    }
                }
                
            )+
        }   
    };
}

gpio!(GPIOA, gpioa, gpioa, iopaen, ioparst, PAx, [
    PA0: (pa0, 0, Input<Floating>, AFRL),
    PA1: (pa1, 1, Input<Floating>, AFRL),
    PA2: (pa2, 2, Input<Floating>, AFRL),
    PA3: (pa3, 3, Input<Floating>, AFRL),
    PA4: (pa4, 4, Input<Floating>, AFRL),
    PA5: (pa5, 5, Input<Floating>, AFRL),
    PA6: (pa6, 6, Input<Floating>, AFRL),
    PA7: (pa7, 7, Input<Floating>, AFRL),
    PA8: (pa8, 8, Input<Floating>, AFRH),
    PA9: (pa9, 9, Input<Floating>, AFRH),
    PA10: (pa10, 10, Input<Floating>, AFRH),
    PA11: (pa11, 11, Input<Floating>, AFRH),
    PA12: (pa12, 12, Input<Floating>, AFRH),
]);