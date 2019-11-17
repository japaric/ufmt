#![no_main]
#![no_std]

use common::W;
use cortex_m::interrupt;
use cortex_m_rt::{entry, exception};
use heapless::{consts::*, i, String};
use ufmt::uwrite;

static mut A: String<U32> = String(i::String::new());

#[entry]
fn main() -> ! {
    let mut x: u8 = 0;
    loop {
        interrupt::free(|_| unsafe {
            A.push(x as char).ok();
        });
        x += 1;
    }
}

#[exception]
fn PendSV() {
    unsafe {
        let a: &str = &A;
        uwrite!(&mut W, "{}", a).unwrap();
        // TODO
        // uwrite!(&mut W, "{:?}", a).unwrap();
    }
}
