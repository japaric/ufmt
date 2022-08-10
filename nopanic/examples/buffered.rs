#![no_main]
#![no_std]

use ufmt::uwriteln;
use ufmt_utils::LineBuffered;

use common::W;

#[no_mangle]
fn _start(a: i8, b: i16) {
    let mut w = LineBuffered::<_, 64>::new(W);

    uwriteln!(&mut w, "{}", a).unwrap();
    uwriteln!(&mut w, "{}", b).unwrap();
    uwriteln!(&mut w, "{:?}", (a, b)).unwrap();
}
