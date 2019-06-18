use crate::{uDebug, uDisplay, uWrite, Formatter};

impl uDebug for bool {
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
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
        W: uWrite + ?Sized,
    {
        <bool as uDebug>::fmt(self, f)
    }
}

// FIXME this (`escape_debug`) contains a panicking branch
// impl uDebug for char {
//     fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
//     where
//         W: uWrite + ?Sized,
//     {
//         f.write_str("'")?;
//         for c in self.escape_debug() {
//             f.write_char(c)?
//         }
//         f.write_str("'")
//     }
// }

impl uDisplay for char {
    #[inline(always)]
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
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
        W: uWrite + ?Sized,
    {
        f.debug_list()?.entries(self)?.finish()
    }
}

// FIXME this (`escape_debug`) contains a panicking branch
// impl uDebug for str {
//     fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
//     where
//         W: uWrite + ?Sized,
//     {
//         f.write_str("\"")?;

//         let mut from = 0;
//         for (i, c) in self.char_indices() {
//             let esc = c.escape_debug();

//             // If char needs escaping, flush backlog so far and write, else skip
//             if esc.len() != 1 {
//                 f.write_str(
//                     self.get(from..i)
//                         .unwrap_or_else(|| unsafe { assume_unreachable!() }),
//                 )?;
//                 for c in esc {
//                     f.write_char(c)?;
//                 }
//                 from = i + c.len_utf8();
//             }
//         }

//         f.write_str(
//             self.get(from..)
//                 .unwrap_or_else(|| unsafe { assume_unreachable!() }),
//         )?;
//         f.write_str("\"")
//     }
// }

impl uDisplay for str {
    #[inline(always)]
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        f.write_str(self)
    }
}

impl<T> uDebug for &'_ T
where
    T: uDebug + ?Sized,
{
    #[inline(always)]
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        <T as uDebug>::fmt(self, f)
    }
}

impl<T> uDisplay for &'_ T
where
    T: uDisplay + ?Sized,
{
    #[inline(always)]
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        <T as uDisplay>::fmt(self, f)
    }
}

impl<T> uDebug for &'_ mut T
where
    T: uDebug + ?Sized,
{
    #[inline(always)]
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        <T as uDebug>::fmt(self, f)
    }
}

impl<T> uDisplay for &'_ mut T
where
    T: uDisplay + ?Sized,
{
    #[inline(always)]
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        <T as uDisplay>::fmt(self, f)
    }
}

impl<T> uDebug for Option<T>
where
    T: uDebug,
{
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        match self {
            None => f.write_str("None"),
            Some(x) => f.debug_tuple("Some")?.field(x)?.finish(),
        }
    }
}

impl<T, E> uDebug for Result<T, E>
where
    T: uDebug,
    E: uDebug,
{
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        match self {
            Err(e) => f.debug_tuple("Err")?.field(e)?.finish(),
            Ok(x) => f.debug_tuple("Ok")?.field(x)?.finish(),
        }
    }
}
