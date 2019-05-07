#![feature(proc_macro_hygiene)]
#![no_main]
#![no_std]

use core::sync::atomic::{AtomicU16, AtomicU8, Ordering};

use common::W;
use cortex_m_rt::{entry, exception};
use ufmt::{derive::uDebug, uwrite};

static A: AtomicU8 = AtomicU8::new(0);
static B: AtomicU16 = AtomicU16::new(0);

#[derive(uDebug)]
enum X {
    A,
    B(u8, u16),
    C { x: u8, y: u16 },
}

#[entry]
fn main() -> ! {
    loop {
        A.fetch_add(1, Ordering::Relaxed);
        B.fetch_add(1, Ordering::Relaxed);
    }
}

#[exception]
fn PendSV() {
    let x = A.load(Ordering::Relaxed);
    let y = B.load(Ordering::Relaxed);

    uwrite!(&mut W, "{:?}", X::A).unwrap();
    uwrite!(&mut W, "{:#?}", X::A).unwrap();

    uwrite!(&mut W, "{:?}", X::B(x, y)).unwrap();
    uwrite!(&mut W, "{:#?}", X::B(x, y)).unwrap();

    uwrite!(&mut W, "{:?}", X::C { x, y }).unwrap();
    uwrite!(&mut W, "{:#?}", X::C { x, y }).unwrap();
}
