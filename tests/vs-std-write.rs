use core::convert::Infallible;
use std::collections::{BTreeMap, BTreeSet};

use ufmt::{derive::uDebug, uDebug, uWrite, uwrite, uwriteln, Formatter};

macro_rules! uformat {
    ($($tt:tt)*) => {{
        let mut s = String::new();
        #[allow(unreachable_code)]
        match ufmt::uwrite!(&mut s, $($tt)*) {
            Ok(_) => Ok(s),
            Err(e) => Err(e),
        }
    }};
}

macro_rules! cmp {
    ($($tt:tt)*) => {
        assert_eq!(
            uformat!($($tt)*),
            Ok(format!($($tt)*)),
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

#[test]
fn formatter_uwrite() {
    #[derive(uDebug)]
    struct X;

    struct Y;

    impl uDebug for Y {
        fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
        where
            W: uWrite + ?Sized,
        {
            uwrite!(f, "{:?}", X)
        }
    }

    assert_eq!(uformat!("{:?}", Y).unwrap(), "X")
}

#[test]
fn generic() {
    #[derive(uDebug, Debug)]
    struct X<T>(T);

    cmp!("{:?}", X(0));

    #[derive(uDebug, Debug)]
    enum Y<T> {
        Z(T),
    }

    cmp!("{:?}", Y::Z(0));
}

// compile-pass test
#[allow(dead_code)]
fn static_lifetime(x: &'static mut u32) {
    fn foo(x: &'static mut u32) -> *mut u32 {
        x as *mut u32
    }

    uwrite!(&mut String::new(), "{:?}", foo(x)).ok();
}

// test dynamically sized writer
#[test]
fn dst() {
    struct Cursor<B>
    where
        B: ?Sized,
    {
        pos: usize,
        buffer: B,
    }

    impl<B> Cursor<B> {
        fn new(buffer: B) -> Self {
            Cursor { pos: 0, buffer }
        }
    }

    impl uWrite for Cursor<[u8]> {
        type Error = Infallible;

        fn write_str(&mut self, s: &str) -> Result<(), Infallible> {
            let bytes = s.as_bytes();
            let len = bytes.len();
            let start = self.pos;
            if let Some(buffer) = self.buffer.get_mut(start..start + len) {
                buffer.copy_from_slice(bytes);
                self.pos += len;
            }

            Ok(())
        }
    }

    let mut cursor = Cursor::new([0; 256]);
    let cursor: &mut Cursor<[u8]> = &mut cursor;

    uwrite!(cursor, "The answer is {}", 42).ok();

    let msg = b"The answer is 42";
    assert_eq!(&cursor.buffer[..msg.len()], msg);
}

#[test]
fn hex() {
    cmp!("{:x}", 771u32);
    cmp!("{:x}", -10000);
    cmp!("{:4x}", 33);
    cmp!("{:4x}", 89001);
    cmp!("{:04x}", 33);
    cmp!("{:#03x}", 33);
    cmp!("{:#09x}", 33);
    cmp!("{:#x}", 71);

    // extreme values
    cmp!("{:x}", i8::min_value());
    cmp!("{:x}", i8::max_value());
    cmp!("{:x}", i16::min_value());
    cmp!("{:x}", i16::max_value());
    cmp!("{:x}", i32::min_value());
    cmp!("{:x}", i32::max_value());
    cmp!("{:x}", i64::min_value());
    cmp!("{:x}", i64::max_value());
    cmp!("{:x}", i128::min_value());
    cmp!("{:x}", i128::max_value());
    cmp!("{:x}", isize::min_value());
    cmp!("{:x}", isize::max_value());

    // <i8 as std::fmt::Display>::fmt(-128)
}

#[test]
fn f32() {
    cmp!("{:10.3}", 3.14_f32);

    cmp!("{:.0}", 1.1_f32);
    cmp!("{:.0}", 0.7_f32);
    cmp!("{:.0}", 0.0_f32);
    cmp!("{:.0}", -0.7_f32);
    cmp!("{:.0}", -1.1_f32);
    cmp!("{:.1}", 1.1_f32);
    cmp!("{:.1}", 0.7_f32);
    cmp!("{:.1}", 0.0_f32);
    cmp!("{:.1}", -0.7_f32);
    cmp!("{:.1}", -1.1_f32);
    cmp!("{:.3}", 1.10234_f32);
    cmp!("{:.3}", 0.70555_f32);
    cmp!("{:.3}", 0.0_f32);
    cmp!("{:.3}", -0.70234_f32);
    cmp!("{:.3}", -1.10555_f32);
    cmp!("{:.5}", 1.10234_f32);
    cmp!("{:.5}", 0.70555_f32);
    cmp!("{:.5}", 0.0_f32);
    cmp!("{:.5}", -0.70234_f32);
    cmp!("{:.5}", -1.10555_f32);
    cmp!("{:.6}", -0.702341234_f32);
    cmp!("{:.6}", -1.105554321_f32);

    const F_MAX32: f32 = 8388608.0; // 2**23
    cmp!("{:.0}", F_MAX32 - 1.0_f32);
    cmp!("{:.0}", F_MAX32);
    cmp!("{:.0}", -F_MAX32);
    cmp!("{:.0}", -F_MAX32 + 1.0_f32);
    cmp!("{:.3}", f32::NAN);
    cmp!("{:.3}", f32::EPSILON);
    cmp!("{:.3}", f32::MIN_POSITIVE);
    cmp!("{:.6}", 8388607.1234567_f32);

    assert_eq!(
        uformat!("{:.3}", F_MAX32 + 1.0_f32),
        Ok(String::from("ovfl")) // std::format "8388609.000"
    );
    assert_eq!(
        uformat!("{:.3}", -F_MAX32 - 1.0_f32),
        Ok(String::from("uflw")) // std::format "-8388609.000"
    );
    assert_eq!(
        uformat!("{:.3}", f32::INFINITY),
        Ok(String::from("ovfl")) // std::format "inf"
    );
    assert_eq!(
        uformat!("{:.3}", f32::NEG_INFINITY),
        Ok(String::from("uflw")) // std::format "-inf"
    );
    assert_eq!(
        uformat!("{}", 321.123456_f32),
        Ok(String::from("321.123")) // std::format "321.12344"
    );
    assert_eq!(
        uformat!("{}", 321.0_f32),
        Ok(String::from("321.000")) // std::format "321"
    );
}

#[test]
fn f64() {
    cmp!("{:10.3}", 3.14_f64);

    cmp!("{:.0}", 1.1_f64);
    cmp!("{:.0}", 0.7_f64);
    cmp!("{:.0}", 0.0_f64);
    cmp!("{:.0}", -0.7_f64);
    cmp!("{:.0}", -1.1_f64);
    cmp!("{:.1}", 1.1_f64);
    cmp!("{:.1}", 0.7_f64);
    cmp!("{:.1}", 0.0_f64);
    cmp!("{:.1}", -0.7_f64);
    cmp!("{:.1}", -1.1_f64);
    cmp!("{:.3}", 1.10234_f64);
    cmp!("{:.3}", 0.70555_f64);
    cmp!("{:.3}", 0.0_f64);
    cmp!("{:.3}", -0.70234_f64);
    cmp!("{:.3}", -1.10555_f64);
    cmp!("{:.5}", 1.10234_f64);
    cmp!("{:.5}", 0.70555_f64);
    cmp!("{:.5}", 0.0_f64);
    cmp!("{:.5}", -0.70234_f64);
    cmp!("{:.5}", -1.10555_f64);
    cmp!("{:.6}", -0.702341234_f64);
    cmp!("{:.6}", -1.105554321_f64);
    cmp!("{:.6}", 4_294_967_294.123456_f64);

    const F_MAX64: f64 = 4_294_967_295.0; // 2**64 / 10
    cmp!("{:.0}", F_MAX64 - 1.0_f64);
    cmp!("{:.0}", F_MAX64);
    cmp!("{:.0}", -F_MAX64);
    cmp!("{:.0}", -F_MAX64 + 1.0_f64);
    cmp!("{:.3}", f64::NAN);
    cmp!("{:.3}", f64::EPSILON);
    cmp!("{:.3}", f64::MIN_POSITIVE);

    assert_eq!(
        uformat!("{:.3}", F_MAX64 + 1.0_f64),
        Ok(String::from("ovfl")) // std::format "1844674407371.000"
    );
    assert_eq!(
        uformat!("{:.3}", -F_MAX64 - 1.0_f64),
        Ok(String::from("uflw")) // std::format "-1844674407371.000"
    );
    assert_eq!(
        uformat!("{:.3}", f64::INFINITY),
        Ok(String::from("ovfl")) // std::format "inf"
    );
    assert_eq!(
        uformat!("{:.3}", f64::NEG_INFINITY),
        Ok(String::from("uflw")) // std::format "-inf"
    );
    assert_eq!(
        uformat!("{}", 321.123456_f64),
        Ok(String::from("321.123")) // std::format "321.123456"
    );
    assert_eq!(
        uformat!("{}", 321.0_f64),
        Ok(String::from("321.000")) // std::format "321"
    );
}
