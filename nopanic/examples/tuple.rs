#![no_main]
#![no_std]

use ufmt::uwrite;

use common::W;

#[no_mangle]
fn _start(a: u8, b: i8) {
    uwrite!(&mut W, "{:?}", ()).unwrap();
    uwrite!(&mut W, "{:#?}", ()).unwrap();

    uwrite!(&mut W, "{:?}", (a,)).unwrap();
    uwrite!(&mut W, "{:#?}", (a,)).unwrap();

    uwrite!(&mut W, "{:?}", (a, b)).unwrap();
    uwrite!(&mut W, "{:#?}", (a, b)).unwrap();
}
