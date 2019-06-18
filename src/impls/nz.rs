use core::num::{
    NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU16, NonZeroU32,
    NonZeroU64, NonZeroU8, NonZeroUsize,
};

use crate::{uDebug, uDisplay, uWrite, Formatter};

macro_rules! nz {
    ($($NZ:ident : $inner:ident,)*) => {
        $(
            impl uDebug for $NZ {
                #[inline(always)]
                fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
                where
                    W: uWrite + ?Sized,
                {
                    <$inner as uDebug>::fmt(&self.get(), f)
                }
            }

            impl uDisplay for $NZ {
                #[inline(always)]
                fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
                where
                    W: uWrite + ?Sized,
                {
                    <$inner as uDisplay>::fmt(&self.get(), f)
                }
            }
        )*
    }
}

nz!(
    NonZeroI16: i16,
    NonZeroI32: i32,
    NonZeroI64: i64,
    NonZeroI8: i8,
    NonZeroIsize: isize,
    NonZeroU16: u16,
    NonZeroU32: u32,
    NonZeroU64: u64,
    NonZeroU8: u8,
    NonZeroUsize: usize,
);
