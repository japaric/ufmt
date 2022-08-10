#![no_main]
#![no_std]

use ufmt::uwrite;

use common::W;

#[no_mangle]
fn _start(c: char) {
    uwrite!(&mut W, "{}", c).unwrap();
    // TODO
    // uwrite!(&mut W, "{:?}", c).unwrap();
}
