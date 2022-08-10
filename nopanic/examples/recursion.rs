#![no_main]
#![no_std]

use ufmt::{derive::uDebug, uwrite};

use common::W;

#[derive(uDebug)]
struct Node {
    value: u32,
    next: Option<&'static Node>,
}

#[no_mangle]
fn _start(x: u32) {
    static TAIL: Node = Node {
        value: 0,
        next: None,
    };

    uwrite!(
        &mut W,
        "{:#?}",
        Node {
            value: x,
            next: Some(&TAIL),
        }
    )
    .unwrap();
}
