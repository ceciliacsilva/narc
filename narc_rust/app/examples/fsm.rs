#![feature(panic_implementation)]
#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate stm32l0;
extern crate hal;
extern crate embedded_hal;
extern crate cortex_m_semihosting as sh;

use core::panic::PanicInfo;
use core::sync::atomic::{self, Ordering};
use cortex_m::asm::bkpt;

use stm32l0::stm32l0x1;
use hal::rcc::RccExt;
use hal::gpio::GpioExt;
use hal::pwm::PwmExt;
use hal::flash::FlashExt;
use hal::time::U32Ext;
use hal::delay::Delay;

use embedded_hal::PwmPin;
use embedded_hal::prelude::*;

use cortex_m_rt::entry;

use core::fmt::Write;

use sh::hio;

#[entry]
fn main() -> ! {
    let hw = stm32l0x1::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    let mut rcc = hw.RCC.constrain();
    let mut flash = hw.FLASH.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut delay = Delay::new(cp.SYST, clocks);

    let mut hstdout = hio::hstdout().unwrap();

    writeln!(hstdout, "Hello, world!").unwrap();

    let mut finite_machine = FiniteMachine::new();

    let t_ref: u32 = 2048;

    let t_h: u32 = 20;

    let mut t_current = t_ref;

    fsm_state_change(&mut finite_machine, 0, 0, 0);

    loop{
        writeln!(hstdout,"Estado atual: {:?}", finite_machine.state).unwrap();
        
        let t_amb = atuador(&finite_machine, t_current);
        t_current = t_amb;
        writeln!(hstdout,"Temperatura ambiente: {}", t_amb).unwrap();
        
        fsm_state_change(&mut finite_machine, t_amb, t_ref, t_h);

        delay.delay_ms(4_000_u16);
    }
}

fn atuador(fsm: &FiniteMachine, t_current: u32) -> u32{
    let num = 5;

    match fsm.state {
        StateType::Cold => t_current - num,
        StateType::Warm => t_current + num,
        _ => t_current + num,
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

/// FSM
pub struct FiniteMachine {
    pub state: StateType,
}

impl FiniteMachine {
    pub fn new() -> FiniteMachine {
        FiniteMachine{
            state: StateType::Start,
        }
    }
}

#[derive(Debug)]
pub enum StateType {
    Start,
    Wait,
    Cold,
    Warm,
}

pub fn fsm_state_change(fsm: &mut FiniteMachine, t_amb: u32, t_ref: u32, t_h: u32) {

    let state =
         match fsm.state {
            StateType::Start => StateType::Wait,
            StateType::Wait => fsm_state_change_wait(t_amb, t_ref, t_h),
            StateType::Cold => fsm_state_change_cold(t_amb, t_ref),
            StateType::Warm => fsm_state_change_warm(t_amb, t_ref),
    };

    fsm.state = state;
}

fn fsm_state_change_wait(t_amb: u32, t_ref: u32, t_h: u32) -> StateType {
    // println!("Estado Wait");

    if t_amb > (t_ref + t_h){
        StateType::Cold
    } else if t_amb < (t_ref - t_h) {
        StateType::Warm
    } else {
        StateType::Wait
    }
}

fn fsm_state_change_cold(t_amb: u32, t_ref: u32) -> StateType {
    // println!("Estado Cold");

    if t_amb < t_ref {
        StateType::Wait
    } else {
        StateType::Cold
    }
}

fn fsm_state_change_warm(t_amb: u32, t_ref: u32) -> StateType {
    // println!("Estado Warm");

    if t_amb > t_ref {
        StateType::Wait
    } else {
        StateType::Warm
    }
}