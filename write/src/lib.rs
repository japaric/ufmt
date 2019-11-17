//! `μfmt`'s `uWrite` trait

#![cfg_attr(not(feature = "std"), no_std)]
#![deny(missing_docs)]
#![deny(rust_2018_compatibility)]
#![deny(rust_2018_idioms)]
#![deny(warnings)]

#[cfg(feature = "std")]
use core::convert::Infallible;

#[allow(deprecated)]
unsafe fn uninitialized<T>() -> T {
    core::mem::uninitialized()
}

/// A collection of methods that are required / used to format a message into a stream.
#[allow(non_camel_case_types)]
pub trait uWrite {
    /// The error associated to this writer
    type Error;

    /// Writes a string slice into this writer, returning whether the write succeeded.
    ///
    /// This method can only succeed if the entire string slice was successfully written, and this
    /// method will not return until all data has been written or an error occurs.
    fn write_str(&mut self, s: &str) -> Result<(), Self::Error>;

    /// Writes a [`char`] into this writer, returning whether the write succeeded.
    ///
    /// A single [`char`] may be encoded as more than one byte. This method can only succeed if the
    /// entire byte sequence was successfully written, and this method will not return until all
    /// data has been written or an error occurs.
    fn write_char(&mut self, c: char) -> Result<(), Self::Error> {
        let mut buf: [u8; 4] = unsafe { uninitialized() };
        self.write_str(c.encode_utf8(&mut buf))
    }
}

#[cfg(feature = "std")]
impl uWrite for String {
    type Error = Infallible;

    fn write_str(&mut self, s: &str) -> Result<(), Infallible> {
        self.push_str(s);
        Ok(())
    }
}
