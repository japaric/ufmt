#![feature(proc_macro_hygiene)]
#![no_main]
#![no_std]

use core::sync::atomic::{AtomicU32, Ordering};

use common::W;
use cortex_m_rt::{entry, exception};
use ufmt::{derive::uDebug, uwrite};

static X: AtomicU32 = AtomicU32::new(0);

#[derive(uDebug)]
struct Node {
    value: u32,
    next: Option<&'static Node>,
}

#[entry]
fn main() -> ! {
    loop {
        X.fetch_add(1, Ordering::Relaxed);
    }
}

#[exception]
fn PendSV() {
    static TAIL: Node = Node {
        value: 0,
        next: None,
    };

    let x = X.load(Ordering::Relaxed);

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
