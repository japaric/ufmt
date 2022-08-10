#![no_main]
#![no_std]

use ufmt::{derive::uDebug, uwrite};

use common::W;

#[derive(uDebug)]
struct Braces {}

#[derive(uDebug)]
struct Parens();

#[derive(uDebug)]
struct I32(i32);

#[derive(uDebug)]
struct Tuple(i32, i32);

#[derive(uDebug)]
struct Nested {
    first: Pair,
    second: Pair,
}

#[derive(Clone, Copy, uDebug)]
struct Pair {
    x: i32,
    y: i32,
}

#[no_mangle]
fn _start(x: i32, y: i32) {
    uwrite!(&mut W, "{:?}", Braces {}).unwrap();
    uwrite!(&mut W, "{:#?}", Braces {}).unwrap();

    uwrite!(&mut W, "{:?}", Parens()).unwrap();
    uwrite!(&mut W, "{:#?}", Parens()).unwrap();

    uwrite!(&mut W, "{:?}", I32(x)).unwrap();
    uwrite!(&mut W, "{:#?}", I32(x)).unwrap();

    uwrite!(&mut W, "{:?}", Tuple(x, y)).unwrap();
    uwrite!(&mut W, "{:#?}", Tuple(x, y)).unwrap();

    let pair = Pair { x, y };
    uwrite!(&mut W, "{:?}", pair).unwrap();
    uwrite!(&mut W, "{:#?}", pair).unwrap();

    let first = pair;
    let second = pair;
    uwrite!(&mut W, "{:?}", Nested { first, second }).unwrap();
    uwrite!(&mut W, "{:#?}", Nested { first, second }).unwrap();
}
