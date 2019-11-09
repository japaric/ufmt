#![feature(proc_macro_hygiene)]
#![no_main]
#![no_std]

use common::W;
use cortex_m::interrupt;
use cortex_m_rt::{entry, exception};
use heapless::{consts::*, Vec, i};
use ufmt::uwrite;

static mut A: Vec<i8, U32> = Vec(i::Vec::new());

#[entry]
fn main() -> ! {
    let mut x = 0;
    loop {
        interrupt::free(|_| unsafe {
            A.push(x).ok();
        });
        x += 1;
    }
}

#[exception]
fn PendSV() {
    unsafe {
        let a: &[i8] = &A;
        uwrite!(&mut W, "{:?}", a).unwrap();
        uwrite!(&mut W, "{:#?}", a).unwrap();
    }
}
