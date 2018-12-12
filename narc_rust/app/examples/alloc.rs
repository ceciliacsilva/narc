//! Example using dynamic allocation.
//! alloc_cortex_m crate.
#![no_std]
#![no_main]
#![feature(alloc)]
#![feature(lang_items)]

extern crate alloc;
extern crate cortex_m;
extern crate cortex_m_rt;
extern crate narc_hal;
extern crate embedded_hal;
extern crate cortex_m_semihosting as sh;
extern crate alloc_cortex_m;

use core::panic::PanicInfo;
use core::sync::atomic::{self, Ordering};
use cortex_m::asm::bkpt;

use cortex_m_rt::entry;

use alloc_cortex_m::CortexMHeap;
use alloc::prelude::Vec;
use alloc::vec;
use alloc::alloc::Layout;

#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

#[entry]
fn main () -> ! {
    let start = cortex_m_rt::heap_start() as usize;
    let size = 1024;
    unsafe { ALLOCATOR.init(start, size) }
    
    let mut vec: Vec<u32> = vec![1, 2, 3];
    vec.push(4);

    loop {

    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    bkpt();

    loop {
        atomic::compiler_fence(Ordering::SeqCst)
    }
}

// required: define how Out Of Memory (OOM) conditions should be handled
// *if* no other crate has already defined `oom`
#[lang = "oom"]
#[no_mangle]
pub fn rust_oom(_layout: Layout) -> ! {
    bkpt();

    loop {
        atomic::compiler_fence(Ordering::SeqCst)
    }
}