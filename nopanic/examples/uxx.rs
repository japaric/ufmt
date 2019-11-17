#![no_main]
#![no_std]

use core::sync::atomic::{AtomicU16, AtomicU32, AtomicU8, AtomicUsize, Ordering};

use common::W;
use cortex_m::interrupt;
use cortex_m_rt::{entry, exception};
use ufmt::uwrite;

static A: AtomicU8 = AtomicU8::new(0);
static B: AtomicU16 = AtomicU16::new(0);
static C: AtomicU32 = AtomicU32::new(0);
static D: AtomicUsize = AtomicUsize::new(0);
static mut E: u64 = 0;
static mut F: u128 = 0;

#[entry]
fn main() -> ! {
    loop {
        A.fetch_add(1, Ordering::Relaxed);
        B.fetch_add(1, Ordering::Relaxed);
        C.fetch_add(1, Ordering::Relaxed);
        D.fetch_add(1, Ordering::Relaxed);
        interrupt::free(|_| unsafe {
            E += 1;
            F += 1;
        })
    }
}

#[exception]
fn PendSV() {
    uwrite!(&mut W, "{}", A.load(Ordering::Relaxed)).unwrap();
    uwrite!(&mut W, "{}", B.load(Ordering::Relaxed)).unwrap();
    uwrite!(&mut W, "{}", C.load(Ordering::Relaxed)).unwrap();
    uwrite!(&mut W, "{}", D.load(Ordering::Relaxed)).unwrap();
    unsafe {
        uwrite!(&mut W, "{}", E).unwrap();
        uwrite!(&mut W, "{}", F).unwrap();
    }
}
