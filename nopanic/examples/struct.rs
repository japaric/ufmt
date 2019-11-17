#![no_main]
#![no_std]

use core::sync::atomic::{AtomicI32, Ordering};

use common::W;
use cortex_m_rt::{entry, exception};
use ufmt::{derive::uDebug, uwrite};

#[derive(uDebug)]
struct Braces {}

#[derive(uDebug)]
struct Parens();

#[derive(uDebug)]
struct I32(i32);

#[derive(uDebug)]
struct Tuple(i32, i32);

#[derive(uDebug)]
struct Nested {
    first: Pair,
    second: Pair,
}

#[entry]
fn main() -> ! {
    loop {
        X.fetch_add(1, Ordering::Relaxed);
        Y.fetch_add(1, Ordering::Relaxed);
    }
}

#[derive(Clone, Copy, uDebug)]
struct Pair {
    x: i32,
    y: i32,
}

static X: AtomicI32 = AtomicI32::new(0);
static Y: AtomicI32 = AtomicI32::new(0);

#[exception]
fn PendSV() {
    let x = X.load(Ordering::Relaxed);
    let y = Y.load(Ordering::Relaxed);

    uwrite!(&mut W, "{:?}", Braces {}).unwrap();
    uwrite!(&mut W, "{:#?}", Braces {}).unwrap();

    uwrite!(&mut W, "{:?}", Parens()).unwrap();
    uwrite!(&mut W, "{:#?}", Parens()).unwrap();

    uwrite!(&mut W, "{:?}", I32(x)).unwrap();
    uwrite!(&mut W, "{:#?}", I32(x)).unwrap();

    uwrite!(&mut W, "{:?}", Tuple(x, y)).unwrap();
    uwrite!(&mut W, "{:#?}", Tuple(x, y)).unwrap();

    let pair = Pair { x, y };
    uwrite!(&mut W, "{:?}", pair).unwrap();
    uwrite!(&mut W, "{:#?}", pair).unwrap();

    let first = pair;
    let second = pair;
    uwrite!(&mut W, "{:?}", Nested { first, second }).unwrap();
    uwrite!(&mut W, "{:#?}", Nested { first, second }).unwrap();
}
