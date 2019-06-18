use crate::{uDebug, uWrite, Formatter};

macro_rules! array {
    ($($N:expr),+) => {
        $(
            impl<T> uDebug for [T; $N]
            where
                T: uDebug,
            {
                fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
                    where
                    W: uWrite + ?Sized,
                {
                    <[T] as uDebug>::fmt(self, f)
                }
            }
        )+
    }
}

array!(
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    26, 27, 28, 29, 30, 31, 32
);
