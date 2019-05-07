use core::str;

use crate::{uDebug, uDisplay, uWrite, Formatter};

macro_rules! uxx {
    ($expr:expr, $f:expr, $N:expr) => {{
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
            $f.write_str(str::from_utf8_unchecked(
                buf.get(i..).unwrap_or_else(|| debug_unreachable!()),
            ))
        }
    }};
}

impl uDebug for u8 {
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite,
    {
        uxx!(*self, f, 3)
    }
}

impl uDisplay for u8 {
    #[inline(always)]
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite,
    {
        <u8 as uDebug>::fmt(self, f)
    }
}

impl uDebug for u16 {
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite,
    {
        uxx!(*self, f, 5)
    }
}

impl uDisplay for u16 {
    #[inline(always)]
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite,
    {
        <u16 as uDebug>::fmt(self, f)
    }
}

impl uDebug for u32 {
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite,
    {
        uxx!(*self, f, 10)
    }
}

impl uDisplay for u32 {
    #[inline(always)]
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite,
    {
        <u32 as uDebug>::fmt(self, f)
    }
}

impl uDebug for u64 {
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite,
    {
        uxx!(*self, f, 20)
    }
}

impl uDisplay for u64 {
    #[inline(always)]
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite,
    {
        <u64 as uDebug>::fmt(self, f)
    }
}

impl uDebug for u128 {
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite,
    {
        uxx!(*self, f, 39)
    }
}

impl uDisplay for u128 {
    #[inline(always)]
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite,
    {
        <u128 as uDebug>::fmt(self, f)
    }
}
