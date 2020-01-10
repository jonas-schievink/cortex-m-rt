//! This test is run in Qemu during CI (see `ci/script.sh`).
//!
//! Anything printed via semihosting will be compared with `qemu.out`. Any
//! discrepancies fail the test.

#![no_main]
#![no_std]

extern crate cortex_m_rt as rt;
extern crate cortex_m_semihosting as semihosting;

extern crate panic_halt;

use rt::{exception, entry, ExceptionFrame};
use semihosting::{hprintln, debug::{exit, EXIT_SUCCESS, EXIT_FAILURE}};
use core::mem;

#[exception]
fn HardFault(ef: &ExceptionFrame) -> ! {
    hprintln!("HardFault").unwrap();

    // We can't test against the entire stacked frame, but a few register values are known.
    // PC for example will always be 0xE0000000.
    hprintln!("PC={:#x}", ef.pc).unwrap();

    // The call to the `not_executable` function sets LR to some point at the beginning of flash,
    // since that's where `raise_hard_fault` will be put.
    if ef.lr < 0x1000 {
        hprintln!("LR looks about right").unwrap();
    } else {
        hprintln!("LR={:#x}", ef.lr).unwrap();
    }

    loop {
        exit(EXIT_SUCCESS);
    }
}

#[exception]
fn PendSV() {}

#[entry]
fn main() -> ! {
    let x = 42;

    hprintln!("x = {}", x).unwrap();

    raise_pendsv();

    hprintln!("Return from NMI").unwrap();

    raise_hard_fault();

    hprintln!("No fault?!").unwrap();

    loop {
        exit(EXIT_FAILURE);
    }
}

/// Causes a `PendSV` exception.
fn raise_pendsv() {
    cortex_m::peripheral::SCB::set_pendsv();
}

/// Raises a `HardFault` exception.
fn raise_hard_fault() {
    // The area at 0xE0000000 is `XN` (eXecute Never). Any instruction fetch will cause a
    // `HardFault`.
    let not_executable = unsafe { mem::transmute::<u32, extern "C" fn()>(0xE0000000) };
    not_executable();
}
