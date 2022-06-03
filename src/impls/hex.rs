use crate::{uDisplayHex, uWrite, Formatter, HexOptions};

macro_rules! hex_format {
    ($buf:expr, $val:expr, $options:expr) => {{
        let mut cursor = $buf.len();
        let mut val = $val;
        if val <= 0 {
            cursor -= 1;
            $buf[cursor] = b'0';
        } else {
            while val != 0 && cursor > 0 {
                let rem = val & 0xf;
                cursor -= 1;
                $buf[cursor] = hex_digit(rem as u8, $options.upper_case);
                val >>= 4;
            }
        }
        unsafe { core::str::from_utf8_unchecked(&$buf[cursor..]) }
    }};
}

macro_rules! hex_pattern {
    ($itype: ty, $utype:ty) => {
        impl uDisplayHex for $itype {
            fn fmt_hex<W>(
                &self,
                fmt: &mut Formatter<'_, W>,
                options: HexOptions,
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
                <$utype as uDisplayHex>::fmt_hex(&positive, fmt, options)
            }
        }

        impl uDisplayHex for $utype {
            fn fmt_hex<W>(
                &self,
                fmt: &mut Formatter<'_, W>,
                options: HexOptions,
            ) -> Result<(), W::Error>
            where
                W: uWrite + ?Sized,
            {
                let mut buffer = [b'0'; 2 * core::mem::size_of::<$utype>()];
                let hex_string = hex_format!(buffer, *self, options);
                options.with_stuff(fmt, hex_string)
            }
        }
    };
}

hex_pattern! {i8, u8}
hex_pattern! {i16, u16}
hex_pattern! {i32, u32}
hex_pattern! {i64, u64}
hex_pattern! {i128, u128}
hex_pattern! {isize, usize}

fn hex_digit(val: u8, upper_case: bool) -> u8 {
    if val < 10 {
        b'0' + val
    } else {
        (if upper_case { b'A' } else { b'a' }) + (val - 10)
    }
}
