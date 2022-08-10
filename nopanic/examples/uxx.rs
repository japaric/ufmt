#![no_main]
#![no_std]

use ufmt::uwrite;

use common::W;

#[no_mangle]
fn _start(a: u8, b: u16, c: u32, d: usize, e: u64, f: u128) {
    uwrite!(&mut W, "{}", a).unwrap();
    uwrite!(&mut W, "{}", b).unwrap();
    uwrite!(&mut W, "{}", c).unwrap();
    uwrite!(&mut W, "{}", d).unwrap();
    uwrite!(&mut W, "{}", e).unwrap();
    uwrite!(&mut W, "{}", f).unwrap();
}
