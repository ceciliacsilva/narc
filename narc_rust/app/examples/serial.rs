#![feature(panic_implementation)]
#![no_std]
#![no_main]

#[macro_use]
extern crate cortex_m;
extern crate cortex_m_rt;
extern crate hal;
extern crate embedded_hal;
#[macro_use(block)]
extern crate nb;

use core::panic::PanicInfo;
use core::sync::atomic::{self, Ordering};
use cortex_m::asm::bkpt;

use embedded_hal::prelude::*;

use hal::stm32l052;
use hal::dma::{Half, Event};
use hal::rcc::RccExt;
use hal::gpio::GpioExt;
use hal::flash::FlashExt;
use hal::serial::Serial;
use hal::time::U32Ext;
use hal::dma::DmaExt;

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    let hw = stm32l052::Peripherals::take().unwrap();

    let mut rcc = hw.RCC.constrain();
    let mut flash = hw.FLASH.constrain();

    let mut gpioa = hw.GPIOA.split(&mut rcc.iop);
    let mut channels = hw.DMA1.split(&mut rcc.ahb);
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let tx = gpioa.pa9.into_alternate(&mut gpioa.moder).af4(&mut gpioa.afrh);
    let rx = gpioa.pa10.into_alternate(&mut gpioa.moder).af4(&mut gpioa.afrh);

    let serial = Serial::usart1(
        hw.USART1,
        (tx, rx),
        9_600.bps(),
        clocks,
        &mut rcc.apb2,
        None,
    );

    let (mut tx, mut rx, _) = serial.split();

    let buf_r = singleton!(: [[u8; 8]; 2] = [[0; 8]; 2]).unwrap();
    let buf_s = singleton!(: [u8; 8] = [97; 8]).unwrap();

    channels.3.listen(Event::HalfTransfer);
    channels.3.listen(Event::TransferComplete);

    channels.2.listen(Event::TransferComplete);

    let sent = b'X';
    block!(tx.write(sent)).ok();

    let (_buf, _c, mut tx) = tx.write_all(channels.2, buf_s).wait();

    let mut circ_buffer = rx.circ_buf(channels.3, buf_r);
    while circ_buffer.readable_half().unwrap() != Half::First {}
    let _first_half = circ_buffer.peek(|half, _| *half).unwrap();

    while circ_buffer.readable_half().unwrap() != Half::Second {}
    let _second_half = circ_buffer.peek(|half, _| *half).unwrap();

    let sent = b'Y';
    block!(tx.write(sent)).ok();

    loop{
    }
}

#[allow(deprecated)]
#[panic_implementation]
fn panic(_info: &PanicInfo) -> ! {
    bkpt();

    loop {
        atomic::compiler_fence(Ordering::SeqCst)
    }
}
