//! Minimal `cortex-m-rt` based program

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

extern crate cortex_m_rt as rt;
extern crate panic_halt;

use rt::entry;
use core::sync::atomic::{Ordering, compiler_fence};

// the program entry point
#[entry]
fn main() -> ! {
    loop {
        compiler_fence(Ordering::SeqCst);
    }
}
