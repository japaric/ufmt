#![no_main]
#![no_std]

use core::sync::atomic::{AtomicUsize, Ordering};

use common::W;
use cortex_m_rt::{entry, exception};
use ufmt::uwrite;

static A: AtomicUsize = AtomicUsize::new(0);

#[entry]
fn main() -> ! {
    loop {
        A.fetch_add(1, Ordering::Relaxed);
    }
}

#[exception]
fn PendSV() {
    uwrite!(&mut W, "{:?}", A.load(Ordering::Relaxed) as *const u8).unwrap();
}
