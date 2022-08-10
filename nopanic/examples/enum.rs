#![no_main]
#![no_std]

use ufmt::{derive::uDebug, uwrite};

use common::W;

#[derive(uDebug)]
enum X {
    A,
    B(u8, u16),
    C { x: u8, y: u16 },
}

#[no_mangle]
fn _start(x: u8, y: u16) {
    uwrite!(&mut W, "{:?}", X::A).unwrap();
    uwrite!(&mut W, "{:#?}", X::A).unwrap();

    uwrite!(&mut W, "{:?}", X::B(x, y)).unwrap();
    uwrite!(&mut W, "{:#?}", X::B(x, y)).unwrap();

    uwrite!(&mut W, "{:?}", X::C { x, y }).unwrap();
    uwrite!(&mut W, "{:#?}", X::C { x, y }).unwrap();
}
