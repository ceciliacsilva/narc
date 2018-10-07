//! Reset and Clock Control

use stm32l0::stm32l0x1::{RCC, rcc};

use cast::u32;

use flash::ACR;
use time::Hertz;

pub trait RccExt {
    /// Constrains the 'Rcc' peripheral so it play nicely with other abstractions.
    fn constrain(self) -> Rcc;
}

impl RccExt for RCC {
    fn constrain(self) -> Rcc {
        Rcc {
            iop: IOP { _0: () },
            apb1: APB1 { _0: () },
            cfgr: CFGR {
                hclk: None,
                pclk1: None,
                pclk2: None,
                sysclk: None,
            },
        }
    }
}

// TODO. It is not complete.
pub struct Rcc {
    // TODO AHB
    pub apb1: APB1,
    // TODO APB2
    pub iop: IOP,
    pub cfgr: CFGR,
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

const HSI: u32 = 16_000_000; // Hz

pub struct CFGR {
    hclk: Option<u32>,
    pclk1: Option<u32>,
    pclk2: Option<u32>,
    sysclk: Option<u32>,
}

impl CFGR {
    pub fn hclk<F>(mut self, freq: F) -> Self 
    where
        F: Into<Hertz>
    {
        self.hclk = Some(freq.into().0);
        self
    }

    pub fn pclk1<F>(mut self, freq: F) -> Self
    where
        F: Into<Hertz>,
    {
        self.pclk1 = Some(freq.into().0);
        self
    }

    pub fn pclk2<F>(mut self, freq: F) -> Self
    where
        F: Into<Hertz>,
    {
        self.pclk2 = Some(freq.into().0);
        self
    }

    pub fn sysclk<F>(mut self, freq: F) -> Self
    where
        F: Into<Hertz>,
    {
        self.sysclk = Some(freq.into().0);
        self
    }

    pub fn freeze(self, _acr: &mut ACR) -> Clocks {
        // MSI

        let sysclk = 2_097_000;
        let hclk = 2_097_000;

        let ppre1 = 1;
        let ppre2 = 1;
            
        let pclk1 =  2_097_000;
        let pclk2 =  2_097_000;

        Clocks {
            hclk: Hertz(hclk),
            pclk1: Hertz(pclk1),
            pclk2: Hertz(pclk2),
            ppre1,
            ppre2,
            sysclk: Hertz(sysclk),
        }
    }

    // PLL is not working. Is never activated.
    pub fn freeze_old(self, acr: &mut ACR) -> Clocks {
        // TODO ADC & USB clocks
        // TODO verify asserts.

        let pllmul: u32 = 24;
        let plldiv: u32 = 3;

        let pllmul_bits = 
            match pllmul {
                3  => Some(0b0000),
                4  => Some(0b0001),
                6  => Some(0b0010),
                8  => Some(0b0011),
                12 => Some(0b0100),
                16 => Some(0b0101),
                24 => Some(0b0110),
                32 => Some(0b0111),
                48 => Some(0b1000),
                _  => None, //min value
            };

        let plldiv_bits =
            match plldiv {
                2 => 0b01,
                3 => 0b10,
                4 => 0b11,
                _ => 0b11, //max value
            };

        let sysclk = ((HSI / 4) * pllmul) / plldiv;
        
        assert!(sysclk <= 32_000_000);

        let hpre_bits = self.hclk
            .map(|hclk| match sysclk / hclk {
                0 => unreachable!(),
                1 => 0b0111, //0xxx: SYSCLK not divided
                2 => 0b1000,
                3...5 => 0b1001,
                6...11 => 0b1010,
                12...39 => 0b1011,
                40...95 => 0b1100,
                96...191 => 0b1101,
                192...383 => 0b1110,
                _ => 0b1111,
            })
            .unwrap_or(0b0111);

        let hclk = sysclk / (1 << (hpre_bits - 0b0111));

        assert!(hclk <= 32_000_000);

        let ppre1_bits = self.pclk1
            .map(|pclk1| match hclk / pclk1 {
                0 => unreachable!(),
                1 => 0b011,
                2 => 0b100,
                3...5 => 0b101,
                6...11 => 0b110,
                _ => 0b111,
            })
            .unwrap_or(0b011);

        let ppre1 = 1 << (ppre1_bits - 0b011);
        let pclk1 = hclk / u32(ppre1);

        assert!(pclk1 <= 32_000_000);

        let ppre2_bits = self.pclk2
            .map(|pclk2| match hclk / pclk2 {
                0 => unreachable!(),
                1 => 0b011,
                2 => 0b100,
                3...5 => 0b101,
                6...11 => 0b110,
                _ => 0b111,
            })
            .unwrap_or(0b011);

        let ppre2 = 1 << (ppre2_bits - 0b011);
        let pclk2 = hclk / u32(ppre2);

        assert!(pclk2 <= 32_000_000);

        let rcc = unsafe { &*RCC::ptr() };
        if let Some(pllmul_bits) = pllmul_bits {
            // use PLL as source
            let pllsw = 0b11;

            if rcc.cfgr.read().sws().bits() == pllsw {
                let hsisw = 0b01;
                rcc.cfgr.modify(|_, w| unsafe {
                                    w.sw().bits(hsisw)});
                
                while rcc.cfgr.read().sws().bits() != hsisw {}
            }

            rcc.cr.write(|w| w.pllon().clear_bit());

            while rcc.cr.read().pllrdy().bit() != false {}

            //Range 1 - 1.65 V - 1.95 V
            //Table 13
            acr.acr().write (|w| {
                w.latency().bit(
                    if sysclk <= 16_000_000 {
                        //0b0
                        false
                    } else {
                        //0b1
                        true
                    })
            });
        
            rcc.cfgr.write(|w| unsafe { w.pllmul().bits(pllmul_bits) });
            rcc.cfgr.write(|w| unsafe { w.plldiv().bits(plldiv_bits) });

            rcc.cr.write(|w| w.pllon().set_bit());

            //is_unclecked
            while rcc.cr.read().pllrdy().bit() == true {}

            rcc.cfgr.write(|w| unsafe {
                    w
                    .ppre2()
                    .bits(ppre2_bits)
                    .ppre1()
                    .bits(ppre1_bits)
                    .hpre()
                    .bits(hpre_bits)
                    .sw()
                    // .pll()
                    .bits(pllsw)
            });

            while rcc.cfgr.read().sws().bits() != pllsw {}         
        } else {
            // use HSI as source
            // nao foi verificado

            rcc.cfgr.write(|w| unsafe {
                w.ppre2()
                    .bits(ppre2_bits)
                    .ppre1()
                    .bits(ppre1_bits)
                    .hpre()
                    .bits(hpre_bits)
                    .sw()
                    // .hsi()
                    .bits(0b01)
            });
        }

        Clocks {
            hclk: Hertz(hclk),
            pclk1: Hertz(pclk1),
            pclk2: Hertz(pclk2),
            ppre1,
            ppre2,
            sysclk: Hertz(sysclk),
        }
    }
}

#[derive(Clone, Copy)]
pub struct Clocks {
    hclk: Hertz,
    pclk1: Hertz,
    pclk2: Hertz,
    ppre1: u8,
    ppre2: u8,
    sysclk: Hertz,
}

impl Clocks {
    pub fn hclk(&self) -> Hertz {
        self.hclk
    }

    pub fn pclk1(&self) -> Hertz {
        self.pclk1
    }

    pub fn pclk2(&self) -> Hertz {
        self.pclk2
    }

    pub(crate) fn ppre1(&self) -> u8 {
        self.ppre1
    }

    // TODO remove `allow`
    #[allow(dead_code)]
    pub(crate) fn ppre2(&self) -> u8 {
        self.ppre2
    }

    pub fn sysclk(&self) -> Hertz {
        self.sysclk
    }
}
