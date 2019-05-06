//! `ufmt`'s Write trait

#![cfg_attr(not(feature = "std"), no_std)]
#![deny(missing_docs)]
#![deny(rust_2018_compatibility)]
#![deny(rust_2018_idioms)]
#![deny(warnings)]

#[cfg(feature = "std")]
use core::convert::Infallible;

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
}

#[cfg(feature = "std")]
impl uWrite for String {
    type Error = Infallible;

    fn write(&mut self, s: &str) -> Result<(), Infallible> {
        self.push_str(s);
        Ok(())
    }
}
