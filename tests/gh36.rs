//! `uwrite!` works in presence of a third-party `Ok` constructor

mod third_party {
    #[allow(dead_code)]
    pub enum Result<A, B> {
        Ok(A),
        Err(B),
    }
}

use ufmt::{derive::uDebug, uwrite, uwriteln};

#[allow(unused_imports)]
use crate::third_party::Result::{self, Err, Ok};

#[derive(uDebug)]
struct Pair {
    x: u32,
    y: u32,
}

#[test]
fn uwrite() {
    let mut s = String::new();
    let pair = Pair { x: 1, y: 2 };

    uwrite!(s, "{:?}", pair).unwrap();
    assert_eq!(s, "Pair { x: 1, y: 2 }");
}

#[test]
fn uwriteln() {
    let mut s = String::new();
    let pair = Pair { x: 1, y: 2 };

    uwriteln!(s, "{:?}", pair).unwrap();
    assert_eq!(s, "Pair { x: 1, y: 2 }\n");
}
