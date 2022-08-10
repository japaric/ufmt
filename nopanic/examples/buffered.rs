#![no_main]
#![no_std]

use core::sync::atomic::{AtomicI16, AtomicI8, Ordering};

use common::W;
use cortex_m_rt::{entry, exception};
use ufmt::uwriteln;
use ufmt_utils::LineBuffered;

static A: AtomicI8 = AtomicI8::new(0);
static B: AtomicI16 = AtomicI16::new(0);

#[entry]
fn main() -> ! {
    loop {
        A.fetch_add(1, Ordering::Relaxed);
        B.fetch_add(1, Ordering::Relaxed);
    }
}

#[exception]
fn PendSV() {
    let mut w = LineBuffered::<_, 64>::new(W);
    let a = A.load(Ordering::Relaxed);
    let b = B.load(Ordering::Relaxed);

    uwriteln!(&mut w, "{}", a).unwrap();
    uwriteln!(&mut w, "{}", b).unwrap();
    uwriteln!(&mut w, "{:?}", (a, b)).unwrap();
}
