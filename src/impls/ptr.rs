use core::str;

use crate::{uDebug, uWrite, Formatter};

macro_rules! hex {
    ($self:expr, $f:expr, $N:expr) => {{
        let mut buf: [u8; $N] = unsafe { crate::uninitialized() };

        let i = hex(*$self as usize, &mut buf);

        unsafe {
            $f.write_str(str::from_utf8_unchecked(
                buf.get(i..).unwrap_or_else(|| assume_unreachable!()),
            ))
        }
    }};
}

fn hex(mut n: usize, buf: &mut [u8]) -> usize {
    let mut i = buf.len() - 1;

    loop {
        let d = (n % 16) as u8;
        *buf.get_mut(i)
            .unwrap_or_else(|| unsafe { assume_unreachable!() }) =
            if d < 10 { d + b'0' } else { (d - 10) + b'a' };
        n /= 16;

        i -= 1;
        if n == 0 {
            break;
        }
    }

    *buf.get_mut(i)
        .unwrap_or_else(|| unsafe { assume_unreachable!() }) = b'x';
    i -= 1;

    *buf.get_mut(i)
        .unwrap_or_else(|| unsafe { assume_unreachable!() }) = b'0';

    i
}

impl<T> uDebug for *const T {
    #[cfg(target_pointer_width = "16")]
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        hex!(self, f, 6)
    }

    #[cfg(target_pointer_width = "32")]
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        hex!(self, f, 10)
    }

    #[cfg(target_pointer_width = "64")]
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        hex!(self, f, 18)
    }
}

impl<T> uDebug for *mut T {
    #[inline(always)]
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        (*self as *const T).fmt(f)
    }
}
