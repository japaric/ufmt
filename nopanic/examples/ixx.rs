#![no_main]
#![no_std]

use ufmt::uwrite;

use common::W;

#[no_mangle]
fn _start(a: i8, b: i16, c: i32, d: isize, e: i64, f: i128) {
    uwrite!(&mut W, "{}", a).unwrap();
    uwrite!(&mut W, "{}", b).unwrap();
    uwrite!(&mut W, "{}", c).unwrap();
    uwrite!(&mut W, "{}", d).unwrap();
    uwrite!(&mut W, "{}", e).unwrap();
    uwrite!(&mut W, "{}", f).unwrap();
}
