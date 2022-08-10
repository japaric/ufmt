use core::{mem::MaybeUninit, slice, str};

use crate::{uDebug, uWrite, Formatter};

macro_rules! hex {
    ($self:expr, $f:expr, $N:expr) => {{
        let mut buf = [MaybeUninit::<u8>::uninit(); $N];

        let i = hex(*$self as usize, &mut buf);

        unsafe {
            $f.write_str(str::from_utf8_unchecked(slice::from_raw_parts(
                buf.as_mut_ptr().add(i).cast(),
                $N - i,
            )))
        }
    }};
}

fn hex(mut n: usize, buf: &mut [MaybeUninit<u8>]) -> usize {
    let ptr = buf.as_mut_ptr().cast::<u8>();
    let len = buf.len();
    let mut i = len - 1;

    loop {
        let d = (n % 16) as u8;
        unsafe {
            ptr.add(i)
                .write(if d < 10 { d + b'0' } else { (d - 10) + b'a' });
        }
        n /= 16;

        i -= 1;
        if n == 0 {
            break;
        }
    }

    unsafe { ptr.add(i).write(b'x') }
    i -= 1;

    unsafe { ptr.add(i).write(b'0') }

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
