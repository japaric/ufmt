//! `μfmt`, a (6-40x) smaller, (2-9x) faster and panic-free alternative to `core::fmt`
//!
//! # Design goals
//!
//! From highest priority to lowest priority
//!
//! - Optimized for binary size and speed (rather than for compilation time)
//! - No trait objects
//! - No panicking branches when optimized
//! - No recursion (if / where possible)
//!
//! # Features
//!
//! - [`Debug`] and [`Display`]-like traits
//! - [`core::write!`][uwrite]-like macro
//! - A generic [`Formatter<'_, impl uWrite>`][formatter] instead of a single `core::Formatter`; the
//!   [`uWrite`] trait has an associated error type so each writer can choose its error type. For
//!   example, the implementation for `std::String` uses [`Infallible`] as its error type.
//! - [`core::fmt::Formatter::debug_struct`][debug_struct]-like API
//! - [`#[derive(uDebug)]`][derive]
//! - Pretty formatting (`{:#?}`) for `uDebug`
//!
//! [`Debug`]: trait.uDebug.html
//! [`Display`]: trait.uDisplay.html
//! [uwrite]: index.html#reexports
//! [formatter]: struct.Formatter.html
//! [`uWrite`]: trait.uWrite.html
//! [`Infallible`]: https://doc.rust-lang.org/core/convert/enum.Infallible.html
//! [debug_struct]: file:///home/japaric/rust/ufmt/target/doc/ufmt/struct.Formatter.html#method.debug_list
//! [derive]: derive/index.html
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
//! - on nightly: `uwrite!` / `uwriteln!`
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
//! - on stable: `Formatter`
//!
//! The `uwrite!` macro requires nightly. On stable you can directly use the `Formatter` API.
//!
//! ```
//! use ufmt::{derive::uDebug, uDebug, Formatter, uwrite};
//!
//! #[derive(uDebug)]
//! struct Pair { x: u32, y: u32 }
//!
//! let mut s = String::new();
//! let pair = Pair { x: 1, y: 2 };
//!
//! // equivalent to `uwrite!("{:#?}", pair).unwrap()`
//! {
//!     let mut f = Formatter::new(&mut s);
//!
//!     f.pretty(|f| uDebug::fmt(&pair, f))
//! }.unwrap();
//!
//! let pretty = "\
//! Pair {
//!     x: 1,
//!     y: 2,
//! }";
//!
//! assert_eq!(s, pretty);
//! ```
//!
//! - on stable: implementing `uWrite`
//!
//! When implementing the `uWrite` trait you should prefer the `ufmt_write::uWrite` crate over the
//! `ufmt::uWrite` crate for better forward compatibility.
//!
//! ```
//! use core::convert::Infallible;
//!
//! use ufmt_write::uWrite;
//!
//! struct MyWriter;
//!
//! impl uWrite for MyWriter {
//!     type Error = Infallible;
//!
//!     fn write_str(&mut self, s: &str) -> Result<(), Self::Error> {
//!         // ..
//!         Ok(())
//!     }
//! }
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
//! prevent const-propagation of the `*write!` arguments.
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
//! Rust 1.34 for everything but the `uwrite!` macro which requires the unstable
//! `proc_macro_hygiene` feature at call site and thus nightly. However, it's possible to use the
//! stable `Formatter` API instead of `uwrite!`.

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

pub use ufmt_macros::{uwrite, uwriteln};
pub use ufmt_write::uWrite;

pub use crate::helpers::{DebugList, DebugMap, DebugStruct, DebugTuple};

#[macro_use]
mod macros;

mod helpers;
mod impls;
mod sealed;
#[cfg(all(test, feature = "std"))]
mod tests;
/// Derive macros
pub mod derive {
    pub use ufmt_macros::uDebug;
}

#[cfg(all(test, not(feature = "std")))]
compile_error!("run `cargo test --features std` instead of `cargo test`");

/// Just like `core::fmt::Debug`
#[allow(non_camel_case_types)]
pub trait uDebug {
    /// Formats the value using the given formatter
    fn fmt<W>(&self, _: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite;
}

/// Just like `core::fmt::Display`
#[allow(non_camel_case_types)]
pub trait uDisplay {
    /// Formats the value using the given formatter
    fn fmt<W>(&self, _: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite;
}

/// Configuration for formatting
#[allow(non_camel_case_types)]
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
    /// Creates a formatter from the given writer
    pub fn new(writer: &'w mut W) -> Self {
        Self {
            indentation: 0,
            pretty: false,
            writer,
        }
    }

    /// Execute the closure with pretty-printing enabled
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

    /// Write whitespace according to the current `self.indentation`
    fn indent(&mut self) -> Result<(), W::Error> {
        for _ in 0..self.indentation {
            self.write_str("    ")?;
        }

        Ok(())
    }
}

// IMPLEMENTATION DETAIL
// We use a trait here to avoid nesting `Formatter`s
#[doc(hidden)]
pub fn unstable_do<D>(
    fmt: D,
    f: impl FnOnce(&mut Formatter<'_, D::Writer>) -> Result<(), <D::Writer as uWrite>::Error>,
) -> Result<(), <D::Writer as uWrite>::Error>
where
    D: sealed::DoAsFormatter,
{
    fmt.do_as_formatter(f)
}
