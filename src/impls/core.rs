use crate::{uDebug, uDisplay, uWrite, Formatter};

impl uDebug for bool {
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite,
    {
        if *self {
            f.write_str("true")
        } else {
            f.write_str("false")
        }
    }
}

impl uDisplay for bool {
    #[inline(always)]
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite,
    {
        <bool as uDebug>::fmt(self, f)
    }
}

impl uDebug for char {
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite,
    {
        f.write_str("'")?;
        for c in self.escape_debug() {
            f.write_char(c)?
        }
        f.write_str("'")
    }
}

impl uDisplay for char {
    #[inline(always)]
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite,
    {
        f.write_char(*self)
    }
}

impl<T> uDebug for [T]
where
    T: uDebug,
{
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite,
    {
        f.debug_list()?.entries(self)?.finish()
    }
}

impl uDebug for str {
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite,
    {
        f.write_str("\"")?;

        let mut from = 0;
        for (i, c) in self.char_indices() {
            let esc = c.escape_debug();

            // If char needs escaping, flush backlog so far and write, else skip
            if esc.len() != 1 {
                f.write_str(&self[from..i])?;
                for c in esc {
                    f.write_char(c)?;
                }
                from = i + c.len_utf8();
            }
        }

        f.write_str(&self[from..])?;
        f.write_str("\"")
    }
}

impl uDisplay for str {
    #[inline(always)]
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite,
    {
        f.write_str(self)
    }
}

impl<T> uDebug for &'_ T
where
    T: uDebug,
{
    #[inline(always)]
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite,
    {
        <T as uDebug>::fmt(self, f)
    }
}

impl<T> uDisplay for &'_ T
where
    T: uDisplay,
{
    #[inline(always)]
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite,
    {
        <T as uDisplay>::fmt(self, f)
    }
}

impl<T> uDebug for &'_ mut T
where
    T: uDebug,
{
    #[inline(always)]
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite,
    {
        <T as uDebug>::fmt(self, f)
    }
}

impl<T> uDisplay for &'_ mut T
where
    T: uDisplay,
{
    #[inline(always)]
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite,
    {
        <T as uDisplay>::fmt(self, f)
    }
}
