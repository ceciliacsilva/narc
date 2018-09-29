use core::panic::PanicInfo;
use core::sync::atomic::{self, Ordering};

pub use cortex_m::asm::bkpt;
pub use cortex_m::peripheral::ITM;
use cortex_m_rt::ExceptionFrame;


#[allow(deprecated)]
#[panic_implementation]
fn panic(info: &PanicInfo) -> ! {
    // let itm = unsafe { &mut *ITM::ptr() };

    // iprintln!(&mut itm.stim[0], "{}", info);

    bkpt();

    loop {
        // add some side effect to prevent LLVM from turning this loop into a UDF (abort) instruction
        // see rust-lang/rust#28728 for details
        atomic::compiler_fence(Ordering::SeqCst)
    }
}

exception!(HardFault, hard_fault);

fn hard_fault(_ef: &ExceptionFrame) -> ! {
    bkpt();

    loop {
        atomic::compiler_fence(Ordering::SeqCst)
    }
}

exception!(*, default_handler);

fn default_handler(_irqn: i16) {
    loop {
        atomic::compiler_fence(Ordering::SeqCst)
    }
}
