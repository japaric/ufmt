#![no_main]
#![no_std]

use ufmt::uwrite;

use common::W;

#[no_mangle]
fn _start(a: f32, b: f64) {
    uwrite!(&mut W, "{:.0}", a).unwrap();
    uwrite!(&mut W, "{:.0}", b).unwrap();
    uwrite!(&mut W, "{:.9}", a).unwrap();
    uwrite!(&mut W, "{:.9}", b).unwrap();
}
