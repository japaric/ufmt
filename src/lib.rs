//! `ufmt`, a smaller and faster `core::fmt`
//!
//! **IMPORTANT** This is work in progress; some stuff, specially the macros, may panic, or worst go
//! into infinite loops, at compile time under some inputs.
//!
//! # Design goals
//!
//! Prioritized in that order
//!
//! - Optimized for binary size and speed (rather than for compilation time)
//! - No trait objects
//! - No panicking branches when optimized
//! - No recursion (if / where possible)
//!
//! # Features
//!
//! - `Debug` and `Display`-like traits
//! - `core::write!`-like macro with string interpolation
//! - `Write` trait instead of a single `core::Formatter`; this trait has an associated error type
//!   so each writer can choose its error type. For example, `alloc::String` can use `!` as the
//!   never type.
//! - `core::Formatter::debug_struct`-like API
//! - `#[derive(uDebug)]`
//!
//! # Examples
//!
//! ``` ignore
//! #![no_std]
//!
//! use ufmt::uDebug;
//!
//! #[derive(uDebug)]
//! struct Pair { x: u32, y: u32 }
//!
//! struct MyWriter;
//!
//! impl uWrite for MyWriter { .. }
//!
//! let pair = Pair { x: 1, y: 2 };
//! write!(&mut MyWriter, "{?}", pair); // "Pair { x: 1, y: 2 }"
//! // ^ OR `pair.fmt(&mut MyWriter)`
//! ```
//!
//! # Benchmarks
//!
//! All benchmarks ran on a ARM Cortex-M3 chip (`thumbv7m-none-eabi`).
//!
//! - \[baseline\] `write!(_, "{}", &(x as i32))` - 281 clock cycles - 2946 bytes of .text
//! - `uwrite!(_, "{}", &(x as i32))` - **44 (15.6%)** clock cycles - 252 bytes of .text
//!
//! - \[baseline\] `write!(_, "{}", &Pair { x, y })` - 969 clock cycles - 5064 bytes of .text
//! - `uwrite!(_, "{}", &Pair { x, y })` - **157 (16.2%)** clock cycles - 656 bytes of .text
//!
//! # Minimum Supported Rust Version (MSRV)
//!
//! Rust 1.34.0, but the `uwrite!` macro requires the unstable `proc_macro_hygiene` feature at call
//! site ad thus nightly.

#![cfg_attr(not(test), no_std)]
#![cfg_attr(test, feature(never_type))]
#![cfg_attr(test, feature(proc_macro_hygiene))]
#![deny(missing_docs)]
#![deny(warnings)]

extern crate self as ufmt;

use core::str;

pub use ufmt_macros::uwrite;

/// Derive macros
pub mod derive {
    pub use ufmt_macros::uDebug;
}

macro_rules! debug_unreachable {
    () => {
        if cfg!(debug_assertions) {
            unreachable!()
        } else {
            core::hint::unreachable_unchecked()
        }
    };
}

/// `?` formatting
#[allow(non_camel_case_types)]
pub trait uDebug {
    /// Formats the value using the given `Write`-r.
    fn fmt<W>(&self, _: &mut W) -> Result<(), W::Error>
    where
        W: uWrite;
}

/// Format trait for an empty format, `{}`
#[allow(non_camel_case_types)]
pub trait uDisplay {
    /// Formats the value using the given `Write`-r.
    fn fmt<W>(&self, _: &mut W) -> Result<(), W::Error>
    where
        W: uWrite;
}

/// A collection of methods that are required / used to format a message into a stream.
#[allow(non_camel_case_types)]
pub trait uWrite {
    /// The error associated to this writer
    type Error;

    /// Writes a string slice into this writer, returning whether the write succeeded.
    ///
    /// This method can only succeed if the entire byte slice was successfully written, and this
    /// method will not return until all data has been written or an error occurs.
    fn write(&mut self, s: &str) -> Result<(), Self::Error>;

    /// Creates a `DebugStruct` builder designed to assist with creation of `fmt::Debug`
    /// implementations for structs.
    fn debug_struct(&mut self, name: &str) -> Result<DebugStruct<Self>, Self::Error> {
        self.write(name)?;

        self.write(" { ")?;

        Ok(DebugStruct {
            first: true,
            writer: self,
        })
    }

    /// Creates a `DebugTuple` builder designed to assist with creation of `fmt::Debug`
    /// implementations for tuple structs.
    fn debug_tuple(&mut self, name: &str) -> Result<DebugTuple<Self>, Self::Error> {
        self.write(name)?;

        self.write("(")?;

        Ok(DebugTuple {
            first: true,
            writer: self,
        })
    }

    /// Creates a `DebugMap` builder designed to assist with creation of `fmt::Debug`
    /// implementations for map-like structures.
    fn debug_map(&mut self) -> Result<DebugMap<Self>, Self::Error> {
        self.write("{")?;

        Ok(DebugMap {
            first: true,
            writer: self,
        })
    }
}

/// A struct to help with `fmt::Debug` implementations.
pub struct DebugStruct<'a, W>
where
    W: uWrite + ?Sized,
{
    first: bool,
    writer: &'a mut W,
}

impl<'a, W> DebugStruct<'a, W>
where
    W: uWrite,
{
    /// Adds a new field to the generated struct output.
    pub fn field(&mut self, name: &str, value: &impl uDebug) -> Result<&mut Self, W::Error> {
        if self.first {
            self.first = false;
        } else {
            self.writer.write(", ")?;
        }

        self.writer.write(name)?;

        self.writer.write(": ")?;

        value.fmt(self.writer)?;

        Ok(self)
    }

    /// Finishes output and returns any error encountered.
    pub fn finish(&mut self) -> Result<&mut Self, W::Error> {
        self.writer.write(" }")?;

        Ok(self)
    }
}

/// A struct to help with `fmt::Debug` implementations.
pub struct DebugTuple<'a, W>
where
    W: uWrite + ?Sized,
{
    first: bool,
    writer: &'a mut W,
}

impl<'a, W> DebugTuple<'a, W>
where
    W: uWrite,
{
    /// Adds a new field to the generated tuple struct output.
    pub fn field(&mut self, value: &impl uDebug) -> Result<&mut Self, W::Error> {
        if self.first {
            self.first = false;
        } else {
            self.writer.write(", ")?;
        }

        value.fmt(self.writer)?;

        Ok(self)
    }

    /// Finishes output and returns any error encountered.
    pub fn finish(&mut self) -> Result<&mut Self, W::Error> {
        self.writer.write(")")?;

        Ok(self)
    }
}

/// A struct to help with `fmt::Debug` implementations.
///
/// This is useful when you wish to output a formatted map as a part of your `Debug::fmt`
/// implementation.
pub struct DebugMap<'a, W>
where
    W: uWrite + ?Sized,
{
    first: bool,
    writer: &'a mut W,
}

impl<'a, W> DebugMap<'a, W>
where
    W: uWrite,
{
    /// Adds a new entry to the map output.
    pub fn entry(&mut self, key: &impl uDebug, value: &impl uDebug) -> Result<&mut Self, W::Error> {
        if self.first {
            self.first = false;
        } else {
            self.writer.write(", ")?;
        }

        key.fmt(self.writer)?;
        self.writer.write(": ")?;
        value.fmt(self.writer)?;

        Ok(self)
    }

    /// Adds the contents of an iterator of entries to the map output.
    pub fn entries<K, V, I>(&mut self, entries: I) -> Result<&mut Self, W::Error>
    where
        I: IntoIterator<Item = (K, V)>,
        K: uDebug,
        V: uDebug,
    {
        for (k, v) in entries.into_iter() {
            self.entry(&k, &v)?;
        }

        Ok(self)
    }

    /// Finishes output and returns any error encountered.
    pub fn finish(&mut self) -> Result<&mut Self, W::Error> {
        self.writer.write("}")?;

        Ok(self)
    }
}

macro_rules! ixx {
    ($uxx:ty, $expr:expr, $w:expr, $N:expr) => {{
        let mut buf: [u8; $N] = unsafe { core::mem::uninitialized() };

        let n = $expr;
        let negative = n.is_negative();
        let mut n = if negative {
            match n.checked_abs() {
                Some(n) => n as $uxx,
                None => <$uxx>::max_value() / 2 + 1,
            }
        } else {
            n as $uxx
        };
        let mut i = $N - 1;
        loop {
            *buf.get_mut(i)
                .unwrap_or_else(|| unsafe { debug_unreachable!() }) = (n % 10) as u8 + b'0';
            n = n / 10;

            if n == 0 {
                break;
            } else {
                i -= 1;
            }
        }

        if negative {
            i -= 1;
            buf[i] = b'-';
        }

        unsafe {
            $w.write(str::from_utf8_unchecked(
                buf.get(i..).unwrap_or_else(|| debug_unreachable!()),
            ))
        }
    }};
}

impl uDebug for i8 {
    fn fmt<W>(&self, w: &mut W) -> Result<(), W::Error>
    where
        W: uWrite,
    {
        ixx!(u8, *self, w, 4)
    }
}

impl uDisplay for i8 {
    #[inline(always)]
    fn fmt<W>(&self, w: &mut W) -> Result<(), W::Error>
    where
        W: uWrite,
    {
        <i8 as uDebug>::fmt(self, w)
    }
}

impl uDebug for i16 {
    fn fmt<W>(&self, w: &mut W) -> Result<(), W::Error>
    where
        W: uWrite,
    {
        ixx!(u16, *self, w, 6)
    }
}

impl uDisplay for i16 {
    #[inline(always)]
    fn fmt<W>(&self, w: &mut W) -> Result<(), W::Error>
    where
        W: uWrite,
    {
        <i16 as uDebug>::fmt(self, w)
    }
}

impl uDebug for i32 {
    fn fmt<W>(&self, w: &mut W) -> Result<(), W::Error>
    where
        W: uWrite,
    {
        ixx!(u32, *self, w, 11)
    }
}

impl uDisplay for i32 {
    #[inline(always)]
    fn fmt<W>(&self, w: &mut W) -> Result<(), W::Error>
    where
        W: uWrite,
    {
        <i32 as uDebug>::fmt(self, w)
    }
}

impl uDebug for i64 {
    fn fmt<W>(&self, w: &mut W) -> Result<(), W::Error>
    where
        W: uWrite,
    {
        ixx!(u64, *self, w, 20)
    }
}

impl uDisplay for i64 {
    #[inline(always)]
    fn fmt<W>(&self, w: &mut W) -> Result<(), W::Error>
    where
        W: uWrite,
    {
        <i64 as uDebug>::fmt(self, w)
    }
}

impl uDebug for i128 {
    fn fmt<W>(&self, w: &mut W) -> Result<(), W::Error>
    where
        W: uWrite,
    {
        ixx!(u128, *self, w, 40)
    }
}

impl uDisplay for i128 {
    #[inline(always)]
    fn fmt<W>(&self, w: &mut W) -> Result<(), W::Error>
    where
        W: uWrite,
    {
        <i128 as uDebug>::fmt(self, w)
    }
}

macro_rules! uxx {
    ($expr:expr, $w:expr, $N:expr) => {{
        let mut buf: [u8; $N] = unsafe { core::mem::uninitialized() };

        let mut n = $expr;
        let mut i = $N - 1;
        loop {
            *buf.get_mut(i)
                .unwrap_or_else(|| unsafe { debug_unreachable!() }) = (n % 10) as u8 + b'0';
            n = n / 10;

            if n == 0 {
                break;
            } else {
                i -= 1;
            }
        }

        unsafe {
            $w.write(str::from_utf8_unchecked(
                buf.get(i..).unwrap_or_else(|| debug_unreachable!()),
            ))
        }
    }};
}

impl uDebug for u8 {
    fn fmt<W>(&self, w: &mut W) -> Result<(), W::Error>
    where
        W: uWrite,
    {
        uxx!(*self, w, 3)
    }
}

impl uDisplay for u8 {
    #[inline(always)]
    fn fmt<W>(&self, w: &mut W) -> Result<(), W::Error>
    where
        W: uWrite,
    {
        <u8 as uDebug>::fmt(self, w)
    }
}

impl uDebug for u16 {
    fn fmt<W>(&self, w: &mut W) -> Result<(), W::Error>
    where
        W: uWrite,
    {
        uxx!(*self, w, 5)
    }
}

impl uDisplay for u16 {
    #[inline(always)]
    fn fmt<W>(&self, w: &mut W) -> Result<(), W::Error>
    where
        W: uWrite,
    {
        <u16 as uDebug>::fmt(self, w)
    }
}

impl uDebug for u32 {
    fn fmt<W>(&self, w: &mut W) -> Result<(), W::Error>
    where
        W: uWrite,
    {
        uxx!(*self, w, 10)
    }
}

impl uDisplay for u32 {
    #[inline(always)]
    fn fmt<W>(&self, w: &mut W) -> Result<(), W::Error>
    where
        W: uWrite,
    {
        <u32 as uDebug>::fmt(self, w)
    }
}

impl uDebug for u64 {
    fn fmt<W>(&self, w: &mut W) -> Result<(), W::Error>
    where
        W: uWrite,
    {
        uxx!(*self, w, 20)
    }
}

impl uDisplay for u64 {
    #[inline(always)]
    fn fmt<W>(&self, w: &mut W) -> Result<(), W::Error>
    where
        W: uWrite,
    {
        <u64 as uDebug>::fmt(self, w)
    }
}

impl uDebug for u128 {
    fn fmt<W>(&self, w: &mut W) -> Result<(), W::Error>
    where
        W: uWrite,
    {
        uxx!(*self, w, 39)
    }
}

impl uDisplay for u128 {
    #[inline(always)]
    fn fmt<W>(&self, w: &mut W) -> Result<(), W::Error>
    where
        W: uWrite,
    {
        <u128 as uDebug>::fmt(self, w)
    }
}

impl uDebug for str {
    fn fmt<W>(&self, w: &mut W) -> Result<(), W::Error>
    where
        W: uWrite,
    {
        w.write(self)
    }
}

impl uDisplay for str {
    #[inline(always)]
    fn fmt<W>(&self, w: &mut W) -> Result<(), W::Error>
    where
        W: uWrite,
    {
        <str as uDebug>::fmt(self, w)
    }
}

impl<'a, T> uDebug for &'a T
where
    T: uDebug,
{
    #[inline(always)]
    fn fmt<W>(&self, w: &mut W) -> Result<(), W::Error>
    where
        W: uWrite,
    {
        <T as uDebug>::fmt(self, w)
    }
}

impl<'a, T> uDisplay for &'a T
where
    T: uDisplay,
{
    #[inline(always)]
    fn fmt<W>(&self, w: &mut W) -> Result<(), W::Error>
    where
        W: uWrite,
    {
        <T as uDisplay>::fmt(self, w)
    }
}

impl<'a, T> uDebug for &'a mut T
where
    T: uDebug,
{
    #[inline(always)]
    fn fmt<W>(&self, w: &mut W) -> Result<(), W::Error>
    where
        W: uWrite,
    {
        <T as uDebug>::fmt(self, w)
    }
}

impl<'a, T> uDisplay for &'a mut T
where
    T: uDisplay,
{
    #[inline(always)]
    fn fmt<W>(&self, w: &mut W) -> Result<(), W::Error>
    where
        W: uWrite,
    {
        <T as uDisplay>::fmt(self, w)
    }
}

macro_rules! tuple {
    ($($T:ident),*; $($i:tt),*) => {
        impl<$($T,)*> uDebug for ($($T,)*)
        where
            $($T: uDebug,)*
        {
            fn fmt<W>(&self, w: &mut W) -> Result<(), W::Error>
                where
                W: uWrite,
            {
                w.debug_tuple("")?$(.field(&self.$i)?)*.finish()?;
                Ok(())
            }
        }

    }
}

tuple!(;);

impl<A> uDebug for (A,)
where
    A: uDebug,
{
    fn fmt<W>(&self, w: &mut W) -> Result<(), W::Error>
    where
        W: uWrite,
    {
        w.write("(")?;
        self.0.fmt(w)?;
        w.write(",)")?;
        Ok(())
    }
}

tuple!(A, B; 0, 1);
tuple!(A, B, C; 0, 1, 2);
tuple!(A, B, C, D; 0, 1, 2, 3);
tuple!(A, B, C, D, E; 0, 1, 2, 3, 4);
tuple!(A, B, C, D, E, F; 0, 1, 2, 3, 4, 5);
tuple!(A, B, C, D, E, F, G; 0, 1, 2, 3, 4, 5, 6);
tuple!(A, B, C, D, E, F, G, H; 0, 1, 2, 3, 4, 5, 6, 7);
tuple!(A, B, C, D, E, F, G, H, I; 0, 1, 2, 3, 4, 5, 6, 7, 8);
tuple!(A, B, C, D, E, F, G, H, I, J; 0, 1, 2, 3, 4, 5, 6, 7, 8, 9);
tuple!(A, B, C, D, E, F, G, H, I, J, K; 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10);
tuple!(A, B, C, D, E, F, G, H, I, J, K, L; 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11);

macro_rules! hex {
    ($self:expr, $w:expr, $N:expr) => {{
        let mut buf: [u8; $N] = unsafe { core::mem::uninitialized() };

        let i = hex(*$self as usize, &mut buf);

        unsafe {
            $w.write(str::from_utf8_unchecked(
                buf.get(i..).unwrap_or_else(|| debug_unreachable!()),
            ))
        }
    }};
}

fn hex(mut n: usize, buf: &mut [u8]) -> usize {
    let mut i = buf.len() - 1;

    loop {
        let d = (n % 16) as u8;
        *buf.get_mut(i)
            .unwrap_or_else(|| unsafe { debug_unreachable!() }) =
            if d < 10 { d + b'0' } else { (d - 10) + b'a' };
        n = n / 16;

        i -= 1;
        if n == 0 {
            break;
        }
    }

    *buf.get_mut(i)
        .unwrap_or_else(|| unsafe { debug_unreachable!() }) = b'x';
    i -= 1;

    *buf.get_mut(i)
        .unwrap_or_else(|| unsafe { debug_unreachable!() }) = b'0';

    i
}

impl<T> uDebug for *const T {
    #[cfg(target_pointer_width = "32")]
    fn fmt<W>(&self, w: &mut W) -> Result<(), W::Error>
    where
        W: uWrite,
    {
        hex!(self, w, 10)
    }

    #[cfg(target_pointer_width = "64")]
    fn fmt<W>(&self, w: &mut W) -> Result<(), W::Error>
    where
        W: uWrite,
    {
        hex!(self, w, 18)
    }
}

impl<T> uDebug for *mut T {
    #[inline(always)]
    fn fmt<W>(&self, w: &mut W) -> Result<(), W::Error>
    where
        W: uWrite,
    {
        (*self as *const T).fmt(w)
    }
}

#[cfg(test)]
mod tests {
    use super::{derive::uDebug, uDebug, uWrite, uwrite};

    impl uWrite for String {
        type Error = !;

        fn write(&mut self, s: &str) -> Result<(), !> {
            self.push_str(s);
            Ok(())
        }
    }

    macro_rules! uformat {
        ($($expr:expr),*) => {{
            let mut s = String::new();
            match uwrite!(&mut s, $($expr,)*) {
                Ok(_) => Ok(s),
                Err(e) => Err(e),
            }
        }};
    }

    #[test]
    fn uxx() {
        assert_eq!(uformat!("{}", 0u8).unwrap(), "0");
        assert_eq!(uformat!("{}", 10u8).unwrap(), "10");
        assert_eq!(uformat!("{}", 100u8).unwrap(), "100");
        assert_eq!(uformat!("{}", 255u8).unwrap(), "255");
    }

    #[test]
    fn ixx() {
        assert_eq!(uformat!("{}", 0i8).unwrap(), "0");
        assert_eq!(uformat!("{}", 10i8).unwrap(), "10");
        assert_eq!(uformat!("{}", 100i8).unwrap(), "100");
        assert_eq!(uformat!("{}", 127i8).unwrap(), "127");
        assert_eq!(uformat!("{}", -128i8).unwrap(), "-128");
    }

    #[test]
    fn fmt() {
        assert_eq!(uformat!("Hello, world!").unwrap(), "Hello, world!");
        assert_eq!(
            uformat!("The answer is {}", 42).unwrap(),
            "The answer is 42"
        );
    }

    #[test]
    fn debug_map() {
        struct Map;

        impl uDebug for Map {
            fn fmt<W>(&self, w: &mut W) -> Result<(), W::Error>
            where
                W: uWrite,
            {
                w.debug_map()?
                    .entry(&1, &2)?
                    .entries([(2, 3), (3, 4)].iter().cloned())?
                    .finish()?;
                Ok(())
            }
        }

        assert_eq!(uformat!("{:?}", Map).unwrap(), "{1: 2, 2: 3, 3: 4}");
    }

    #[test]
    fn debug_struct() {
        #[derive(uDebug)]
        struct Pair {
            x: i32,
            y: i32,
        }

        assert_eq!(
            uformat!("{:?}", Pair { x: 1, y: 2 }).unwrap(),
            "Pair { x: 1, y: 2 }"
        );
    }

    #[test]
    fn enum_() {
        #[derive(uDebug)]
        enum X {
            A,
            B(u8, u16),
            C { x: u8, y: u16 },
        }

        assert_eq!(uformat!("{:?}", X::A).unwrap(), "A");

        assert_eq!(uformat!("{:?}", X::B(0, 1)).unwrap(), "B(0, 1)");

        assert_eq!(
            uformat!("{:?}", X::C { x: 0, y: 1 }).unwrap(),
            "C { x: 0, y: 1 }"
        );
    }

    #[test]
    fn hex() {
        assert_eq!(uformat!("{:?}", 1 as *const u8).unwrap(), "0x1");
        assert_eq!(uformat!("{:?}", 0xf as *const u8).unwrap(), "0xf");
        assert_eq!(uformat!("{:?}", 0xff as *const u8).unwrap(), "0xff");
        assert_eq!(uformat!("{:?}", 0xfff as *const u8).unwrap(), "0xfff");
        assert_eq!(uformat!("{:?}", 0xffff as *const u8).unwrap(), "0xffff");
        assert_eq!(uformat!("{:?}", 0xfffff as *const u8).unwrap(), "0xfffff");
        assert_eq!(uformat!("{:?}", 0xffffff as *const u8).unwrap(), "0xffffff");
        assert_eq!(
            uformat!("{:?}", 0xfffffff as *const u8).unwrap(),
            "0xfffffff"
        );
        assert_eq!(
            uformat!("{:?}", 0xffffffff as *const u8).unwrap(),
            "0xffffffff"
        );

        #[cfg(target_pointer_width = "64")]
        assert_eq!(uformat!("{:?}", 1 as *mut u8).unwrap(), "0x1");
        #[cfg(target_pointer_width = "64")]
        assert_eq!(
            uformat!("{:?}", 0xfffffffff as *const u8).unwrap(),
            "0xfffffffff"
        );
    }

    #[test]
    fn tuples() {
        assert_eq!(uformat!("{:?}", ()).unwrap(), "()");
        assert_eq!(uformat!("{:?}", (1,)).unwrap(), "(1,)");
        assert_eq!(uformat!("{:?}", (1, 2)).unwrap(), "(1, 2)");
        assert_eq!(uformat!("{:?}", (1, 2, 3)).unwrap(), "(1, 2, 3)");
        assert_eq!(uformat!("{:?}", (1, 2, 3, 4)).unwrap(), "(1, 2, 3, 4)");
        assert_eq!(
            uformat!("{:?}", (1, 2, 3, 4, 5)).unwrap(),
            "(1, 2, 3, 4, 5)"
        );
        assert_eq!(
            uformat!("{:?}", (1, 2, 3, 4, 5, 6)).unwrap(),
            "(1, 2, 3, 4, 5, 6)"
        );
        assert_eq!(
            uformat!("{:?}", (1, 2, 3, 4, 5, 6, 7)).unwrap(),
            "(1, 2, 3, 4, 5, 6, 7)"
        );
        assert_eq!(
            uformat!("{:?}", (1, 2, 3, 4, 5, 6, 7, 8)).unwrap(),
            "(1, 2, 3, 4, 5, 6, 7, 8)"
        );
        assert_eq!(
            uformat!("{:?}", (1, 2, 3, 4, 5, 6, 7, 8, 9)).unwrap(),
            "(1, 2, 3, 4, 5, 6, 7, 8, 9)"
        );
        assert_eq!(
            uformat!("{:?}", (1, 2, 3, 4, 5, 6, 7, 8, 9, 10)).unwrap(),
            "(1, 2, 3, 4, 5, 6, 7, 8, 9, 10)"
        );
        assert_eq!(
            uformat!("{:?}", (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11)).unwrap(),
            "(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11)"
        );
        assert_eq!(
            uformat!("{:?}", (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12)).unwrap(),
            "(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12)"
        );
    }
}
