//! `μfmt`'s `uWrite` trait

#![cfg_attr(not(feature = "std"), no_std)]
#![deny(missing_docs)]
#![deny(rust_2018_compatibility)]
#![deny(rust_2018_idioms)]
#![deny(warnings)]

#[cfg(feature = "std")]
use core::convert::Infallible;

use core::mem::MaybeUninit;

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
        let mut buf: MaybeUninit<[u8; 4]> = MaybeUninit::uninit();
        let encoded = unsafe { encode_utf8_raw(c as u32, &mut buf) };
        self.write_str(encoded)
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

#[inline]
fn encode_utf8_raw(code: u32, dst: &mut MaybeUninit<[u8; 4]>) -> &str {
    let len = len_utf8(code);

    unsafe {
        let dst = dst.as_mut_ptr();
        let a = dst as *mut u8;
        let b = a.add(1);
        let c = a.add(2);
        let d = a.add(3);
        match len {
            1 => {
                *a = code as u8;
            }
            2 => {
                *a = (code >> 6 & 0x1F) as u8 | TAG_TWO_B;
                *b = (code & 0x3F) as u8 | TAG_CONT;
            }
            3 => {
                *a = (code >> 12 & 0x0F) as u8 | TAG_THREE_B;
                *b = (code >> 6 & 0x3F) as u8 | TAG_CONT;
                *c = (code & 0x3F) as u8 | TAG_CONT;
            }
            4 => {
                *a = (code >> 18 & 0x07) as u8 | TAG_FOUR_B;
                *b = (code >> 12 & 0x3F) as u8 | TAG_CONT;
                *c = (code >> 6 & 0x3F) as u8 | TAG_CONT;
                *d = (code & 0x3F) as u8 | TAG_CONT;
            }
            _ => unreachable!(),
        };

        let bytes = core::slice::from_raw_parts(dst as *const u8, len);
        core::str::from_utf8_unchecked(bytes)
    }
}

#[inline]
fn len_utf8(code: u32) -> usize {
    if code < MAX_ONE_B {
        1
    } else if code < MAX_TWO_B {
        2
    } else if code < MAX_THREE_B {
        3
    } else {
        4
    }
}

// UTF-8 ranges and tags for encoding characters
const TAG_CONT: u8 = 0b1000_0000;
const TAG_TWO_B: u8 = 0b1100_0000;
const TAG_THREE_B: u8 = 0b1110_0000;
const TAG_FOUR_B: u8 = 0b1111_0000;
const MAX_ONE_B: u32 = 0x80;
const MAX_TWO_B: u32 = 0x800;
const MAX_THREE_B: u32 = 0x10000;
