use std::collections::{BTreeMap, BTreeSet};

use crate::{derive::uDebug, uwrite, uwriteln};

macro_rules! uformat {
    ($($expr:expr),*) => {{
        let mut s = String::new();
        #[allow(unreachable_code)]
        match uwrite!(&mut s, $($expr,)*) {
            Ok(_) => Ok(s),
            Err(e) => Err(e),
        }
    }};
}

macro_rules! cmp {
    ($s:expr $(,$args:expr)*) => {
        assert_eq!(
            uformat!($s $(,$args)*),
            Ok(format!($s $(,$args)*)),
        )
    }
}

#[test]
fn core() {
    cmp!("{:?}", None::<i32>);
    cmp!("{:#?}", None::<i32>);

    cmp!("{:?}", Some(0));
    cmp!("{:#?}", Some(0));

    cmp!("{:?}", Ok::<_, ()>(1));
    cmp!("{:#?}", Ok::<_, ()>(1));

    cmp!("{:?}", Err::<(), _>(2));
    cmp!("{:#?}", Err::<(), _>(2));
}

#[test]
fn recursion() {
    #[derive(uDebug, Debug)]
    struct Node {
        value: i32,
        next: Option<Box<Node>>,
    }

    fn x() -> Node {
        let tail = Node {
            value: 0,
            next: None,
        };
        Node {
            value: 1,
            next: Some(Box::new(tail)),
        }
    }

    cmp!("{:?}", x());
    cmp!("{:#?}", x());
}

#[test]
fn uxx() {
    cmp!("{}", 0u8);
    cmp!("{}", 10u8);
    cmp!("{}", 100u8);

    // extreme values
    cmp!("{}", u8::max_value());
    cmp!("{}", u16::max_value());
    cmp!("{}", u32::max_value());
    cmp!("{}", u64::max_value());
    cmp!("{}", u128::max_value());
    cmp!("{}", usize::max_value());
}

#[test]
fn ixx() {
    // sanity check
    cmp!("{}", 0i8);
    cmp!("{}", 10i8);
    cmp!("{}", 100i8);

    // extreme values
    cmp!("{}", i8::min_value());
    cmp!("{}", i8::max_value());
    cmp!("{}", i16::min_value());
    cmp!("{}", i16::max_value());
    cmp!("{}", i32::min_value());
    cmp!("{}", i32::max_value());
    cmp!("{}", i64::min_value());
    cmp!("{}", i64::max_value());
    cmp!("{}", i128::min_value());
    cmp!("{}", i128::max_value());
    cmp!("{}", isize::min_value());
    cmp!("{}", isize::max_value());
}

#[test]
fn fmt() {
    cmp!("Hello, world!");
    cmp!("The answer is {}", 42);
}

#[test]
fn map() {
    fn x() -> BTreeMap<i32, i32> {
        let mut m = BTreeMap::new();
        m.insert(1, 2);
        m.insert(3, 4);
        m
    }

    cmp!("{:?}", BTreeMap::<(), ()>::new());
    cmp!("{:?}", x());

    cmp!("{:#?}", BTreeMap::<(), ()>::new());
    cmp!("{:#?}", x());
}

#[test]
fn set() {
    fn x() -> BTreeSet<i32> {
        let mut m = BTreeSet::new();
        m.insert(1);
        m.insert(3);
        m
    }

    cmp!("{:?}", BTreeSet::<()>::new());
    cmp!("{:?}", x());

    cmp!("{:#?}", BTreeSet::<()>::new());
    cmp!("{:#?}", x());
}

#[test]
fn struct_() {
    #[derive(Debug, uDebug)]
    struct Braces {}

    #[derive(Debug, uDebug)]
    struct Parens();

    #[derive(Debug, Default, uDebug)]
    struct I32(i32);

    #[derive(Debug, Default, uDebug)]
    struct Tuple(i32, i32);

    #[derive(Debug, Default, uDebug)]
    struct Pair {
        x: i32,
        y: i32,
    }

    #[derive(Debug, Default, uDebug)]
    struct Nested {
        first: Pair,
        second: Pair,
    }

    cmp!("{:?}", Braces {});
    cmp!("{:?}", Parens());
    cmp!("{:?}", I32::default());
    cmp!("{:?}", Tuple::default());
    cmp!("{:?}", Pair::default());
    cmp!("{:?}", Nested::default());

    cmp!("{:#?}", Braces {});
    cmp!("{:#?}", Parens());
    cmp!("{:#?}", I32::default());
    cmp!("{:#?}", Tuple::default());
    cmp!("{:#?}", Pair::default());
    cmp!("{:#?}", Nested::default());
}

#[test]
fn enum_() {
    #[derive(Debug, uDebug)]
    enum X {
        A,
        B(u8, u16),
        C { x: u8, y: u16 },
    }

    cmp!("{:?}", X::A);
    cmp!("{:?}", X::B(0, 1));
    cmp!("{:?}", X::C { x: 0, y: 1 });

    cmp!("{:#?}", X::A);
    cmp!("{:#?}", X::B(0, 1));
    cmp!("{:#?}", X::C { x: 0, y: 1 });
}

#[test]
fn ptr() {
    cmp!("{:?}", 1 as *const u8);
    cmp!("{:?}", 0xf as *const u8);
    cmp!("{:?}", 0xff as *const u8);
    cmp!("{:?}", 0xfff as *const u8);
    cmp!("{:?}", 0xffff as *const u8);
    cmp!("{:?}", 0xfffff as *const u8);
    cmp!("{:?}", 0xffffff as *const u8);
    cmp!("{:?}", 0xfffffff as *const u8);
    cmp!("{:?}", 0xffffffff as *const u8);

    #[cfg(target_pointer_width = "64")]
    cmp!("{:?}", 0xfffffffff as *const u8);
}

#[test]
fn tuples() {
    cmp!("{:?}", ());
    cmp!("{:?}", (1,));
    cmp!("{:?}", (1, 2));
    cmp!("{:?}", (1, 2, 3));
    cmp!("{:?}", (1, 2, 3, 4));
    cmp!("{:?}", (1, 2, 3, 4, 5));
    cmp!("{:?}", (1, 2, 3, 4, 5, 6));
    cmp!("{:?}", (1, 2, 3, 4, 5, 6, 7));
    cmp!("{:?}", (1, 2, 3, 4, 5, 6, 7, 8));
    cmp!("{:?}", (1, 2, 3, 4, 5, 6, 7, 8, 9));
    cmp!("{:?}", (1, 2, 3, 4, 5, 6, 7, 8, 9, 10));
    cmp!("{:?}", (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11));
    cmp!("{:?}", (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12));

    cmp!("{:#?}", ());
    cmp!("{:#?}", (1,));
    cmp!("{:#?}", (1, 2));
}

#[test]
fn slice() {
    cmp!("{:?}", [0; 0]);
    cmp!("{:?}", [0]);
    cmp!("{:?}", [0, 1]);

    cmp!("{:#?}", [0; 0]);
    cmp!("{:#?}", [0]);
    cmp!("{:#?}", [0, 1]);
}

#[test]
fn uwriteln() {
    let mut s = String::new();
    uwriteln!(&mut s, "Hello").unwrap();
    uwriteln!(&mut s, "World",).unwrap();
    assert_eq!(s, "Hello\nWorld\n");
}
