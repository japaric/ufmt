//! `μfmt` utilities
//!
//! # Minimum Supported Rust Version (MSRV)
//!
//! This crate is guaranteed to compile on stable Rust 1.36 and up. It *might* compile on older
//! versions but that may change in any new patch release.

#![deny(missing_docs)]
#![deny(rust_2018_compatibility)]
#![deny(rust_2018_idioms)]
#![deny(warnings)]
#![no_std]

use core::{convert::Infallible, str, fmt};

pub use heapless::consts;
use heapless::{ArrayLength, String};
use ufmt_write::uWrite;


macro_rules! assume_unreachable {
    () => {
        if cfg!(debug_assertions) {
            panic!()
        } else {
            core::hint::unreachable_unchecked()
        }
    };
}

/// A write adapter that ignores all errors
pub struct Ignore<W>
where
    W: uWrite,
{
    writer: W,
}

impl<W> Ignore<W>
where
    W: uWrite,
{
    /// Creates a new `Ignore` adapter
    pub fn new(writer: W) -> Self {
        Self { writer }
    }

    /// Destroys the adapter and returns the underlying writer
    pub fn free(self) -> W {
        self.writer
    }
}

impl<W> uWrite for Ignore<W>
where
    W: uWrite,
{
    type Error = Infallible;

    fn write_str(&mut self, s: &str) -> Result<(), Infallible> {
        let _ = self.writer.write_str(s);
        Ok(())
    }
}

/// A write adapter that buffers writes and automatically flushes on newlines
pub struct LineBuffered<W, N>
where
    N: ArrayLength<u8>,
    W: uWrite,
{
    buffer: String<N>,
    writer: W,
}

impl<W, N> LineBuffered<W, N>
where
    N: ArrayLength<u8>,
    W: uWrite,
{
    /// Creates a new `LineBuffered` adapter
    pub fn new(writer: W) -> Self {
        Self {
            buffer: String::new(),
            writer,
        }
    }

    /// Flushes the contents of the buffer
    pub fn flush(&mut self) -> Result<(), W::Error> {
        let ret = self.writer.write_str(&self.buffer);
        self.buffer.clear();
        ret
    }

    /// Destroys the adapter and returns the underlying writer
    pub fn free(self) -> W {
        self.writer
    }

    fn push_str(&mut self, s: &str) -> Result<(), W::Error> {
        let len = s.as_bytes().len();
        if self.buffer.len() + len > self.buffer.capacity() {
            self.flush()?;
        }

        if len > self.buffer.capacity() {
            self.writer.write_str(s)?;
        } else {
            self.buffer
                .push_str(s)
                .unwrap_or_else(|_| unsafe { assume_unreachable!() })
        }

        Ok(())
    }
}

impl<W, N> uWrite for LineBuffered<W, N>
where
    N: ArrayLength<u8>,
    W: uWrite,
{
    type Error = W::Error;

    fn write_str(&mut self, mut s: &str) -> Result<(), W::Error> {
        while let Some(pos) = s.as_bytes().iter().position(|b| *b == b'\n') {
            let line = s
                .get(..pos + 1)
                .unwrap_or_else(|| unsafe { assume_unreachable!() });

            self.push_str(line)?;
            self.flush()?;

            s = s
                .get(pos + 1..)
                .unwrap_or_else(|| unsafe { assume_unreachable!() });
        }

        self.push_str(s)
    }
}


/// An adapter struct allowing to use `ufmt` on types which implement `core::fmt::Write`
///
/// For example:
///
/// ```
/// use ufmt::uwrite;
/// use ufmt_write::uWrite;
/// use ufmt_utils::WriteAdapter;
///
/// let fancy_number: u8 = 42;
///
/// let mut s = String::new();
/// uwrite!(WriteAdapter(&mut s), "{:?}", fancy_number);
/// ```
pub struct WriteAdapter<W>(pub W) where W: fmt::Write;

impl<W> uWrite for WriteAdapter<W> where W: fmt::Write {
    type Error = fmt::Error;

    fn write_char(&mut self, c: char) -> Result<(), Self::Error> {
        self.0.write_char(c)
    }

    fn write_str(&mut self, s: &str) -> Result<(), Self::Error> {
        self.0.write_str(s)
    }
}
