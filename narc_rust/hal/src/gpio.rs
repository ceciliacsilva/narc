//! General Purpose Input Output

// It should provide the same interface as the program in C.
// TODO: Assert that

//!
//! # Example
//! ```
//! #![feature(panic_implementation)]
//! #![no_std]
//! #![no_main]
//!
//! extern crate cortex_m;
//! extern crate cortex_m_rt;
//! extern crate hal;
//! extern crate embedded_hal;
//!  
//! use stm32l0::stm32l0x1;
//! use hal::gpio::GpioExt;
//! use hal::rcc::RccExt; 
//! use core::panic::PanicInfo;
//! use core::sync::atomic::{self, Ordering};
//! use cortex_m::asm::bkpt;
//! use embedded_hal::digital::OutputPin; 
//! use embedded_hal::digital::InputPin;   
//!
//! use cortex_m_rt::entry;
//!
//! #[entry]
//! fn main() -> ! {
//!
//!    let hw = stm32l0x1::Peripherals::take().unwrap();
//!    let cp = cortex_m::Peripherals::take().unwrap();
//!    let mut rcc = hw.RCC.constrain();
//!    
//!    let mut gpioa = hw.GPIOA.split(&mut rcc.iop);
//!    let mut led = gpioa.pa5.into_output(&mut gpioa.moder).push_pull(&mut gpioa.otyper);
//!    let mut botao = gpioa.pa4.into_input(&mut gpioa.moder).pull_up(&mut gpioa.pupdr);
//!
//!    loop{
//!        if !botao.is_low(){
//!            led.set_high();
//!         }
//!        else {
//!           led.set_low();
//!        }
//!    }
//! }
//!
//! #[allow(deprecated)]
//! #[panic_implementation]
//! fn panic(_info: &PanicInfo) -> ! {
//!    bkpt();
//!
//!    loop {
//!        atomic::compiler_fence(Ordering::SeqCst)
//!    }
//! }
//! ```

use core::marker::PhantomData;
use rcc::IOP;

use embedded_hal::digital::{OutputPin, InputPin};
/// Extenção da trait para o pino GPIO e seus registros
pub trait GpioExt {
    ///Split dentro do GPIO
    type Parts;
    /// Divide o GPIO em pinos e em registros
    fn split(self, iop: &mut IOP) -> Self::Parts;

}

/// Digital Input Mode
pub struct InputDigital;
/// Digital Input Mode
pub struct Input<MODE> {
    _mode: PhantomData<MODE>,
}
/// Input Mode types 
pub struct PullDown;
/// Input Mode types
pub struct PullUp;

/// Digital Output Mode
pub struct OutputDigital;
/// Digital Output Mode
pub struct Output<MODE> {
    _mode: PhantomData<MODE>,
}
/// Output Mode types
pub struct PushPull;
/// Output Mode types
pub struct OpenDrain;

/// Alternate Function Mode
pub struct Alternate;
/// Alternate function 0
pub struct AF0;
/// Alternate function 1
pub struct AF1;
/// Alternate function 2
pub struct AF2;
/// Alternate function 3
pub struct AF3;
/// Alternate function 4
pub struct AF4;
/// Alternate function 5
pub struct AF5;
/// Alternate function 6
pub struct AF6;
/// Alternate function 7
pub struct AF7;
/// Alternate function 8
pub struct AF8;
/// Alternate function 9
pub struct AF9;
/// Alternate function 10
pub struct AF10;
/// Alternate function 11
pub struct AF11;
/// Alternate function 12
pub struct AF12;
/// Alternate function 13
pub struct AF13;
/// Alternate function 14
pub struct AF14;
/// Alternate function 15
pub struct AF15;
/// Analog function Mode (reset state)
pub struct Analog;

macro_rules! gpio {
    ($GPIOX:ident, $gpiox:ident, $gpioy:ident, $iopxenr:ident, $iopxrst:ident, $PXx:ident, [
        $($PXi:ident: ($pxi:ident, $i:expr, $MODE:ty, $CR:ident),)+
    ]) => {
        /// Define port GPIOA
        pub mod $gpiox {
            use core::marker::PhantomData;

            use stm32l052::{$gpioy, $GPIOX};
            use rcc::IOP;

            use super::*;
            /// GPIO parts
            pub struct Parts {
                /// Registro do MODER
                pub moder: MODER,
                /// Registro do OTYPER
                pub otyper: OTYPER,
                /// Registro do PUPDR
                pub pupdr: PUPDR,
                /// Registro do AFRH
                pub afrh: AFRH,
                /// Registro do AFRL
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
            /// Mode Register
            pub struct MODER {
                _0: (),
            }
         
            impl MODER {
                pub(crate) fn moder(&mut self) -> &$gpioy::MODER {
                    unsafe { &(*$GPIOX::ptr()).moder }
                }
            }
            /// Output Type Register (Topology)
            pub struct OTYPER {
                _0: (),
            }

            impl OTYPER {
                pub(crate) fn otyper(&mut self) -> &$gpioy::OTYPER {
                    unsafe { &(*$GPIOX::ptr()).otyper }
                }
            }
            /// Pull-Up/Pull-Down Register
            pub struct PUPDR {
                _0: (),
            }

            impl PUPDR {
                pub(crate) fn pupdr(&mut self) -> &$gpioy::PUPDR {
                    unsafe { &(*$GPIOX::ptr()).pupdr }
                }
            }
            /// Alternate Function High Register
            pub struct AFRH {
                _0: (),
            }
            
            impl AFRH {
                pub(crate) fn afr(&mut self) -> &$gpioy::AFRH {
                    unsafe { &(*$GPIOX::ptr()).afrh }
                }
            }
            /// Alternate Function Low Register
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
                /// Configura o pin como Output 
                impl $PXi<OutputDigital> {
                    //! Define o pin como Push Pull 
                    pub fn push_pull(self, otyper: &mut OTYPER) -> $PXi<Output<PushPull>>{
                        let output_type = 0b0;
                        otyper
                            .otyper()
                            .modify(|r, w| unsafe { w.bits(r.bits() & !(output_type << $i)) });

                        $PXi { _mode: PhantomData }
                    }
                    /// Configura o pin Open Drain
                    pub fn open_drain(self, otyper: &mut OTYPER) -> $PXi<Output<OpenDrain>>{
                        let output_type = 0b1;
                        otyper
                            .otyper()
                            .modify(|r, w| unsafe { w.bits(r.bits() & !(output_type << $i)) });

                        $PXi { _mode: PhantomData }
                    }
                }

                impl $PXi<Alternate> {
                    /// Configura o pin 5 como alternative
                    pub fn af5(self, afrl: &mut AFRL) -> $PXi<AF5> {
                        let af = 5;
                        let offset = 4 * ($i % 8);

                        afr.afr().modify(|r, w| unsafe {
                            w.bits((r.bits() & !(0b1111 << offset)) | (af << offset))
                        });

                        $PXi { _mode: PhantomData }
                    }
                }
                /// Configura o pin como Input
                impl $PXi<InputDigital> {
                    /// Define o pin como Pull Up
                    pub fn pull_up(self, pupdr: &mut PUPDR) -> $PXi<Input<PullUp>>{
                        let offset = 2 * $i;

                        let pull_type = 0b01;
                        pupdr.pupdr().modify(|r, w| unsafe {
                            w.bits((r.bits() & !(0b11 << offset)) | (pull_type << offset))
                        });

                        $PXi { _mode: PhantomData }
                    }
                    /// Define o pin com Pull Down
                    pub fn pull_down(self, pupdr: &mut PUPDR) -> $PXi<Input<PullDown>>{
                        let offset = 2 * $i;

                        let pull_type = 0b10;
                        pupdr.pupdr().modify(|r, w| unsafe {
                            w.bits((r.bits() & !(0b11 << offset)) | (pull_type << offset))
                        });

                        $PXi { _mode: PhantomData }
                    }
                }

                impl $PXi<Analog> {
                    // TODO all modes.
                    // TODO change generic MODE to Analog
                    
                    /// Define o pin como Output
                    pub fn into_output (self, moder: &mut MODER) -> $PXi<OutputDigital> {
                        let offset = 2 * $i;

                        let mode = 0b01;
                        moder.moder().modify(|r, w| unsafe {
                            w.bits((r.bits() & !(0b11 << offset)) | (mode << offset)) });

                        $PXi { _mode: PhantomData }
                    }
                    /// Define o pin como Input
                    pub fn into_input (self, moder: &mut MODER) -> $PXi<InputDigital> {
                        let offset = 2 * $i;

                        let mode = 0b00;
                        moder.moder().modify(|r, w| unsafe {
                            w.bits((r.bits() & !(0b11 << offset)) | (mode << offset)) });

                        $PXi { _mode: PhantomData }
                    }
                    /// Define o pin como função Alternativa
                    pub fn into_alternate (self, moder: &mut MODER) -> $PXi<Alternate> {
                        let offset = 2 * $i;

                        // alternative function
                        let mode = 0b10;
                        moder.moder().modify(|r, w| unsafe{
                            w.bits((r.bits() & !(0b11 << offset)) | (mode << offset))
                        });

                        $PXi { _mode: PhantomData }
                    }

                    /// Estado de reset dos pinos
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
                /// Faz a configuração do estado do pin
                impl<MODE> OutputPin for $PXi<Output<MODE>> {
                    fn set_high(&mut self) {
                        unsafe { (*$GPIOX::ptr()).bsrr.write(|w| w.bits(1 << $i)) }
                    }

                    fn set_low(&mut self) {
                        unsafe { (*$GPIOX::ptr()).bsrr.write(|w| w.bits(1 << (16 + $i))) }
                    }
                }
                /// Faz a leitura do estado do pin
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