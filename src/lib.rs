//! `μfmt`, a smaller and faster alternative to `core::fmt`
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
//! - `core::write!`-like macro for string interpolation
//! - A generic `Formatter<'_, impl uWrite>` instead of a single `core::Formatter`; the `uWrite`
//!   trait has an associated error type so each writer can choose its error type. For example,
//!   the implementation for `std::String` uses [`Infallible`] as its error type.
//! - `core::Formatter::debug_struct`-like API
//! - `#[derive(uDebug)]`
//! - Pretty formatting (`{:#?}`) for `uDebug`
//!
//! [`Infallible`]: https://doc.rust-lang.org/core/convert/enum.Infallible.html
//!
//! # Non-features
//!
//! These are out of scope
//!
//! - Padding, alignment and other formatting options
//! - Formatting floating point numbers
//!
//! # Examples
//!
//! ```
//! #![feature(proc_macro_hygiene)]
//!
//! use ufmt::{derive::uDebug, uwrite};
//!
//! #[derive(uDebug)]
//! struct Pair { x: u32, y: u32 }
//!
//! let mut s = String::new();
//! let pair = Pair { x: 1, y: 2 };
//! uwrite!(&mut s, "{:?}", pair).unwrap();
//! assert_eq!(s, "Pair { x: 1, y: 2 }");
//! ```
//!
//! # Benchmarks
//!
//! The benchmarks ran on a ARM Cortex-M3 chip (`thumbv7m-none-eabi`).
//!
//! The benchmarks were compiled with `nightly-2019-05-01`, `-C opt-level=3`, `lto = true`,
//! `codegen-units = 1`.
//!
//! In all benchmarks `x = i32::MIN` and `y = i32::MIN` plus some obfuscation was applied to
//! prevent const-propagation.
//!
//! The unit of time is one core clock cycle: 125 ns (8 MHz)
//!
//! The `.text` and `.rodata` columns indicate the delta (in bytes) when commenting out the
//! `*write!` statement.
//!
//! |Code                                      |Time|%        |`.text`|%        |`.rodata`|%       |
//! |------------------------------------------|----|---------|-------|---------|---------|--------|
//! |`write!("Hello, world!")`                 |154 |~        |1906   |~        |248      |~       |
//! |`uwrite!("Hello, world!")`                |20  |**13.0%**|34     |**1.8%** |16       |**6.5%**|
//! |`write!(w, "{}", 0i32)`                   |256 |~        |1958   |~        |232      |~       |
//! |`uwrite!(w, "{}", 0i32)`                  |37  |**14.5%**|288    |**14.7%**|0        |**0%**  |
//! |`write!(w, "{}", x)`                      |381 |~        |
//! |`uwrite!(w, "{}", x)`                     |295 |77.4%    |
//! |`write!(w, "{:?}", Pair { x: 0, y: 0 })`  |996 |~        |4704   |~        |312      |~       |
//! |`uwrite!(w, "{:?}", Pair { x: 0, y: 0 })` |254 |**25.5%**|752    |**16.0%**|24       |**7.7%**|
//! |`write!(w, "{:?}", Pair { x, y })`        |1264|~        |
//! |`uwrite!(w, "{:?}", Pair { x, y })`       |776 |61.4%    |
//! |`write!(w, "{:#?}", Pair { x: 0, y: 0 })` |2853|~        |4710   |~        |348      |~       |
//! |`uwrite!(w, "{:#?}", Pair { x: 0, y: 0 })`|301 |**10.6%**|754    |**16.0%**|24       |**6.9%**|
//! |`write!(w, "{:#?}", Pair { x, y })`       |3693|~        |
//! |`uwrite!(w, "{:#?}", Pair { x, y })`      |823 |**22.3%**|
//!
//!
//! Benchmark program:
//!
//! ``` ignore
//! static X: AtomicI32 = AtomicI32::new(i32::MIN); // or `0`
//! static Y: AtomicI32 = AtomicI32::new(i32::MIN); // or `0`
//!
//! #[exception]
//! fn PendSV() {
//!    // read DWT.CYCCNT here
//!
//!    let x = X.load(Ordering::Relaxed);
//!    let y = Y.load(Ordering::Relaxed);
//!
//!    let p = Pair { x, y };
//!
//!    uwrite!(&mut W, "{:#?}", p).ok();
//!
//!    // write!(&mut W, "{:#?}", p).ok();
//!
//!    asm::bkpt(); // read DWT.CYCCNT here
//! }
//! ```
//!
//! Writer used in the benchmarks:
//!
//! ```
//! use core::{convert::Infallible, fmt, ptr};
//!
//! use ufmt::uWrite;
//!
//! struct W;
//!
//! impl uWrite for W {
//!     type Error = Infallible;
//!
//!     fn write_str(&mut self, s: &str) -> Result<(), Infallible> {
//!         s.as_bytes()
//!             .iter()
//!             .for_each(|b| unsafe { drop(ptr::read_volatile(b)) });
//!
//!         Ok(())
//!     }
//! }
//!
//! impl fmt::Write for W {
//!     fn write_str(&mut self, s: &str) -> fmt::Result {
//!         s.as_bytes()
//!             .iter()
//!             .for_each(|b| unsafe { drop(ptr::read_volatile(b)) });
//!
//!         Ok(())
//!     }
//! }
//! ```
//!
//! # Minimum Supported Rust Version (MSRV)
//!
//! Rust 1.34.0 for everything but the `uwrite!` macro which requires the unstable
//! `proc_macro_hygiene` feature at call site and thus nightly.

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(test, feature(proc_macro_hygiene))]
#![deny(missing_docs)]
#![deny(rust_2018_compatibility)]
#![deny(rust_2018_idioms)]
#![deny(warnings)]

// this lets us use `uwrite!` in the test suite
#[allow(unused_extern_crates)]
#[cfg(test)]
extern crate self as ufmt;

use core::str;

pub use ufmt_macros::uwrite;
pub use ufmt_write::uWrite;

pub use crate::helpers::{DebugList, DebugMap, DebugStruct, DebugTuple};

macro_rules! debug_unreachable {
    () => {
        if cfg!(debug_assertions) {
            unreachable!()
        } else {
            core::hint::unreachable_unchecked()
        }
    };
}

mod helpers;
mod impls;
#[cfg(test)]
mod tests;
/// Derive macros
pub mod derive {
    pub use ufmt_macros::uDebug;
}

/// `?` formatting
#[allow(non_camel_case_types)]
pub trait uDebug {
    /// Formats the value using the given `Write`-r.
    fn fmt<W>(&self, _: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite;
}

/// Format trait for an empty format, `{}`
#[allow(non_camel_case_types)]
pub trait uDisplay {
    /// Formats the value using the given `Write`-r.
    fn fmt<W>(&self, _: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite;
}

/// Configuration for formatting
pub struct Formatter<'w, W>
where
    W: uWrite,
{
    indentation: u8,
    pretty: bool,
    writer: &'w mut W,
}

impl<'w, W> Formatter<'w, W>
where
    W: uWrite,
{
    fn new(writer: &'w mut W) -> Self {
        Formatter {
            indentation: 0,
            pretty: false,
            writer,
        }
    }

    /// Write whitespace according to the current `self.indentation`
    fn indent(&mut self) -> Result<(), W::Error> {
        for _ in 0..self.indentation {
            self.write_str("    ")?;
        }

        Ok(())
    }

    // IMPLEMENTATION DETAIL
    #[doc(hidden)]
    pub fn pretty(
        &mut self,
        f: impl FnOnce(&mut Self) -> Result<(), W::Error>,
    ) -> Result<(), W::Error> {
        let pretty = self.pretty;
        self.pretty = true;
        f(self)?;
        self.pretty = pretty;
        Ok(())
    }

    /// Writes a character to the underlying buffer contained within this formatter.
    pub fn write_char(&mut self, c: char) -> Result<(), W::Error> {
        self.writer.write_char(c)
    }

    /// Writes a string slice to the underlying buffer contained within this formatter.
    pub fn write_str(&mut self, s: &str) -> Result<(), W::Error> {
        self.writer.write_str(s)
    }
}

// IMPLEMENTATION DETAIL
// We use a trait here to avoid nesting `Formatter`s
#[doc(hidden)]
pub fn do_<D>(
    fmt: D,
    f: impl FnOnce(&mut Formatter<'_, D::Writer>) -> Result<(), <D::Writer as uWrite>::Error>,
) -> Result<(), <D::Writer as uWrite>::Error>
where
    D: DoAsFormatter,
{
    fmt.do_as_formatter(f)
}

// IMPLEMENTATION DETAIL
#[doc(hidden)]
pub trait DoAsFormatter {
    type Writer: uWrite;

    fn do_as_formatter(
        self,
        f: impl FnOnce(&mut Formatter<'_, Self::Writer>) -> Result<(), <Self::Writer as uWrite>::Error>,
    ) -> Result<(), <Self::Writer as uWrite>::Error>;
}

impl<W> DoAsFormatter for &'_ mut W
where
    W: uWrite,
{
    type Writer = W;

    fn do_as_formatter(
        self,
        f: impl FnOnce(&mut Formatter<'_, W>) -> Result<(), W::Error>,
    ) -> Result<(), W::Error> {
        f(&mut Formatter::new(self))
    }
}

impl<W> DoAsFormatter for &'_ mut Formatter<'_, W>
where
    W: uWrite,
{
    type Writer = W;

    fn do_as_formatter(
        self,
        f: impl FnOnce(&mut Formatter<'_, W>) -> Result<(), W::Error>,
    ) -> Result<(), W::Error> {
        f(self)
    }
}
