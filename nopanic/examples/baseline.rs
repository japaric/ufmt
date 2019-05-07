#![feature(proc_macro_hygiene)]
#![no_main]
#![no_std]

use core::sync::atomic::{AtomicUsize, Ordering};

use common::W;
use cortex_m_rt::{entry, exception};
use ufmt::uwrite;

#[entry]
fn main() -> ! {
    loop {}
}
