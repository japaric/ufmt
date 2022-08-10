#![no_main]
#![no_std]

use ufmt::uwrite;

use common::W;

#[no_mangle]
fn _start(s: &str) {
    uwrite!(&mut W, "{}", s).unwrap();
    // TODO
    // uwrite!(&mut W, "{:?}", s).unwrap();
}
