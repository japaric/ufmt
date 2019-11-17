#![no_main]
#![no_std]

use core::sync::atomic::{AtomicI8, AtomicU8, Ordering};

use common::W;
use cortex_m_rt::{entry, exception};
use ufmt::uwrite;

static A: AtomicU8 = AtomicU8::new(0);
static B: AtomicI8 = AtomicI8::new(0);

#[entry]
fn main() -> ! {
    loop {
        A.fetch_add(1, Ordering::Relaxed);
        B.fetch_add(1, Ordering::Relaxed);
    }
}

#[exception]
fn PendSV() {
    let a = A.load(Ordering::Relaxed);
    let b = B.load(Ordering::Relaxed);

    uwrite!(&mut W, "{:?}", ()).unwrap();
    uwrite!(&mut W, "{:#?}", ()).unwrap();

    uwrite!(&mut W, "{:?}", (a,)).unwrap();
    uwrite!(&mut W, "{:#?}", (a,)).unwrap();

    uwrite!(&mut W, "{:?}", (a, b)).unwrap();
    uwrite!(&mut W, "{:#?}", (a, b)).unwrap();
}
