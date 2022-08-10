#![no_main]
#![no_std]

use ufmt::uwrite;

use common::W;

#[no_mangle]
fn _start(p: *const u8) {
    uwrite!(&mut W, "{:?}", p).unwrap();
}
