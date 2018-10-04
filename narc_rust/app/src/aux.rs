use core::panic::PanicInfo;
use core::sync::atomic::{self, Ordering};

pub use cortex_m::asm::bkpt;
pub use cortex_m::peripheral::ITM;
// use cortex_m_rt::ExceptionFrame;
use cortex_m_rt::{exception};
// use cortex_m_semihosting::hio::{self, HStdout};

// use core::fmt::Write;


#[allow(deprecated)]
#[panic_implementation]
fn panic(_info: &PanicInfo) -> ! {
    // let itm = unsafe { &mut *ITM::ptr() };

    // iprintln!(&mut itm.stim[0], "{}", info);

    bkpt();

    loop {
        // add some side effect to prevent LLVM from turning this loop into a UDF (abort) instruction
        // see rust-lang/rust#28728 for details
        atomic::compiler_fence(Ordering::SeqCst)
    }
}

// Don't dead
// Open  inside

#[exception]
fn DefaultHandler(_irqn: i16) -> ! {
    bkpt();
    loop {}
}

/* 
#[exception]
fn SysTick() -> ! {
    bkpt();
    /* static mut STATE: u32 = 0;

    *STATE += 1;

    if *STATE > 10 {
        bkpt();
    } */
    loop {
        atomic::compiler_fence(Ordering::SeqCst)
    }
}


 */