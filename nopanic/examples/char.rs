#![no_main]
#![no_std]

use core::sync::atomic::{AtomicU8, Ordering};

use common::W;
use cortex_m_rt::{entry, exception};
use ufmt::uwrite;

static A: AtomicU8 = AtomicU8::new(0);

#[entry]
fn main() -> ! {
    loop {
        A.fetch_add(1, Ordering::Relaxed);
    }
}

#[exception]
fn PendSV() {
    let a = A.load(Ordering::Relaxed) as char;
    uwrite!(&mut W, "{}", a).unwrap();
    // TODO
    // uwrite!(&mut W, "{:?}", a).unwrap();
}
