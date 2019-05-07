#![feature(proc_macro_hygiene)]
#![no_main]
#![no_std]

use core::sync::atomic::{AtomicI16, AtomicI32, AtomicI8, AtomicIsize, Ordering};

use common::W;
use cortex_m::interrupt;
use cortex_m_rt::{entry, exception};
use ufmt::uwrite;

static A: AtomicI8 = AtomicI8::new(0);
static B: AtomicI16 = AtomicI16::new(0);
static C: AtomicI32 = AtomicI32::new(0);
static D: AtomicIsize = AtomicIsize::new(0);
static mut E: i64 = 0;
static mut F: i128 = 0;

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
