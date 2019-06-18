use crate::{uDebug, uWrite, Formatter};

macro_rules! tuple {
    ($($T:ident),*; $($i:tt),*) => {
        impl<$($T,)*> uDebug for ($($T,)*)
        where
            $($T: uDebug,)*
        {
            fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
            where
                W: uWrite + ?Sized,
            {
                f.debug_tuple("")?$(.field(&self.$i)?)*.finish()
            }
        }

    }
}

impl uDebug for () {
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        f.write_str("()")
    }
}

tuple!(A; 0);
tuple!(A, B; 0, 1);
tuple!(A, B, C; 0, 1, 2);
tuple!(A, B, C, D; 0, 1, 2, 3);
tuple!(A, B, C, D, E; 0, 1, 2, 3, 4);
tuple!(A, B, C, D, E, F; 0, 1, 2, 3, 4, 5);
tuple!(A, B, C, D, E, F, G; 0, 1, 2, 3, 4, 5, 6);
tuple!(A, B, C, D, E, F, G, H; 0, 1, 2, 3, 4, 5, 6, 7);
tuple!(A, B, C, D, E, F, G, H, I; 0, 1, 2, 3, 4, 5, 6, 7, 8);
tuple!(A, B, C, D, E, F, G, H, I, J; 0, 1, 2, 3, 4, 5, 6, 7, 8, 9);
tuple!(A, B, C, D, E, F, G, H, I, J, K; 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10);
tuple!(A, B, C, D, E, F, G, H, I, J, K, L; 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11);
