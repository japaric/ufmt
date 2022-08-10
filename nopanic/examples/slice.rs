#![no_main]
#![no_std]

use ufmt::uwrite;

use common::W;

#[no_mangle]
fn _start(a: &[i8]) {
    uwrite!(&mut W, "{:?}", a).unwrap();
    uwrite!(&mut W, "{:#?}", a).unwrap();
}
