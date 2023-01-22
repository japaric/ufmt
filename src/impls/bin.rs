use crate::{uDisplayBin, uWrite, BinOptions, Formatter};

macro_rules! bin_format {
    ($buf:expr, $val:expr, $options:expr) => {{
        let mut cursor = $buf.len();
        let mut val = $val;
        if val <= 0 {
            cursor -= 1;
            $buf[cursor] = b'0';
        } else {
            while val != 0 && cursor > 0 {
                let rem = val & 0b1;
                cursor -= 1;
                $buf[cursor] = bin_digit(rem as u8, $options.upper_case);
                val >>= 1;
            }
        }
        unsafe { core::str::from_utf8_unchecked(&$buf[cursor..]) }
    }};
}

macro_rules! bin_pattern {
    ($itype: ty, $utype:ty) => {
        impl uDisplayBin for $itype {
            fn fmt_bin<W>(
                &self,
                fmt: &mut Formatter<'_, W>,
                options: BinOptions,
            ) -> Result<(), W::Error>
            where
                W: uWrite + ?Sized,
            {
                let positive = if false && // the standard rust library doesn't format negative numbers with a minus sign
                *self < 0 {
                    fmt.write_char('-')?;
                    ((!*self) as $utype).wrapping_add(1)
                } else {
                    *self as $utype
                };
                <$utype as uDisplayBin>::fmt_bin(&positive, fmt, options)
            }
        }

        impl uDisplayBin for $utype {
            fn fmt_bin<W>(
                &self,
                fmt: &mut Formatter<'_, W>,
                options: BinOptions,
            ) -> Result<(), W::Error>
            where
                W: uWrite + ?Sized,
            {
                let mut buffer = [b'0'; 2 * core::mem::size_of::<$utype>()];
                let bin_string = bin_format!(buffer, *self, options);
                options.with_stuff(fmt, bin_string)
            }
        }
    };
}

bin_pattern! {i8, u8}
bin_pattern! {i16, u16}
bin_pattern! {i32, u32}
bin_pattern! {i64, u64}
bin_pattern! {i128, u128}
bin_pattern! {isize, usize}

fn bin_digit(val: u8, _upper_case: bool) -> u8 {
    b'0' + val
}
