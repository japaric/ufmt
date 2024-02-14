use crate::{uDisplay, uDebug, uDisplayFloat, uWrite, Formatter};
use core::{str::from_utf8_unchecked, slice::from_raw_parts};

// Implementiert den Trait für f32
impl uDisplayFloat for f32 {
    fn fmt_float<W>(
        &self, 
        fmt: &mut Formatter<'_, W>, 
        decimal_places: usize, 
        pad_length: usize
    ) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        // General checks for validity and overflow
        if self.is_nan() {
            fmt.write_str("NaN")?;
            return Ok(());
        }

        if *self > 8388608.0 { // 2**23
            fmt.write_str("ovfl")?;
            return Ok(());
        }

        if *self < -8388608.0 { // 2**23
            fmt.write_str("uflw")?;
            return Ok(());
        }

        const MUL_TAB: [f32; 7] = [1.0, 10.0, 100.0, 1_000.0, 10_000.0, 100_000.0, 1_000_000.0];
        const ADD_TAB: [f32; 7] = [0.5, 0.05, 0.005, 0.000_5, 0.000_05, 0.000_005, 0.000_000_5];

        let (f, is_neg) = if self.is_sign_negative() {
            ((-*self) + ADD_TAB[decimal_places as usize], true)
        } else {
            (*self + ADD_TAB[decimal_places as usize], false)
        };

        let left = f as u32;
        let right =  ((f - (left as f32)) * MUL_TAB[decimal_places as usize]) as u32;

        write_as_float_str::<W>(fmt, left, right, decimal_places, is_neg, pad_length)
    }
}

impl uDebug for f32 {
    fn fmt<W>(&self, fmt: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        self.fmt_float(fmt, 3, 0)
    }
}

impl uDisplay for f32 {
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        <f32 as uDebug>::fmt(self, f)
    }
}

impl uDisplayFloat for f64 {
    fn fmt_float<W>(
        &self, 
        fmt: &mut Formatter<'_, W>, 
        decimal_places: usize, 
        pad_length: usize
    ) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        // General checks for validity and overflow
        if self.is_nan() {
            fmt.write_str("NaN")?;
            return Ok(());
        }

        if *self > 4_294_967_295.0 { // u32::MAX
            fmt.write_str("ovfl")?;
            return Ok(());
        }

        if *self < -4_294_967_295.0 { // u32::MAX
            fmt.write_str("uflw")?;
            return Ok(());
        }

        const MUL_TAB: [f64; 7] = [1.0, 10.0, 100.0, 1_000.0, 10_000.0, 100_000.0, 1_000_000.0];
        const ADD_TAB: [f64; 7] = [0.5, 0.05, 0.005, 0.000_5, 0.000_05, 0.000_005, 0.000_000_5];

        let (f, is_neg) = if self.is_sign_negative() {
            ((-*self) + ADD_TAB[decimal_places as usize], true)
        } else {
            (*self + ADD_TAB[decimal_places as usize], false)
        };

        let left = f as u32;
        let right =  ((f - (left as f64)) * MUL_TAB[decimal_places as usize]) as u32;
    
        write_as_float_str::<W>(fmt, left, right, decimal_places, is_neg, pad_length)
    }
}

impl uDebug for f64 {
    fn fmt<W>(&self, fmt: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        self.fmt_float(fmt, 3, 0)
    }
}

impl uDisplay for f64 {
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        <f64 as uDebug>::fmt(self, f)
    }
}


// Internally, the floating point number is displayed as two integers, whereby the location of 
// the decimal point is shown separately.
//
// This routine then writes the floating point number correctly in the formatter
fn write_as_float_str<W>(
    fmt: &mut Formatter<'_, W>, 
    mut left: u32, 
    mut right: u32, 
    decimal_places: usize, 
    is_neg: bool,
    pad_length: usize) -> Result<(), W::Error>
where
    W: uWrite + ?Sized,
{
    // max 2**32 4_294_967_296 (10 digits) + 6 digits right dp + '.' + '-' => 18 digits max
    let mut buf = [core::mem::MaybeUninit::<u8>::uninit(); 18];
    let p_buf = buf.as_mut_ptr().cast::<u8>();

    let len = buf.len();
    let mut idx = len;
    let dp_idx = if decimal_places == 0 {
        None
    } else {
        Some(len - decimal_places as usize)
    };

    // Safety: This is necessary to avoid getting a panic branch
    // The algorithm ensures that the buf array range limits are not exceeded
    let write_buf = |idx: usize, c: u8| {
        unsafe { p_buf.add(idx).write(c) };
    };

    // write digits to the right of the dp
    if let Some(dp_idx) = dp_idx {
        while idx > dp_idx {
            idx -= 1;
            let m = (right % 10) as u8;
            right = right / 10;
            write_buf(idx, m + b'0');
        }
        idx -= 1;
        write_buf(idx, b'.');
    }

    // write digits to the left of the dp
    if left == 0 {
        idx -= 1;
        write_buf(idx, b'0');
    } else {
        while left > 0 {
            idx -= 1;
            let m = (left % 10) as u8;
            left = left / 10;
            write_buf(idx, m + b'0');
        }
    }
    
    // Add negativ sign if necessary
    if is_neg {
        idx -= 1;
        write_buf(idx, b'-');
    }
    
    let length = len - idx;

    // Padding, if necessary
    if length > 0 && pad_length > length {
        for _ in 0..(pad_length - length) {
            fmt.write_char(' ')?;
        }
    }

    // Safety: This is necessary to avoid getting a panic branch
    // Since we know what has been added, we also know that they are correct utf8 characters.
    let s = unsafe {
        let slice = from_raw_parts(p_buf.add(idx), length);
        from_utf8_unchecked(slice)
    };
    fmt.write_str(s)

}


#[cfg(feature = "std")]
#[cfg(test)]
mod tests {
    use crate::Formatter;

    fn f_str(left: u32, right: u32, dp: usize, is_neg: bool, pad_length: usize) -> String {
        let mut s = String::new();
        let mut fmt = Formatter::new(&mut s);
        super::write_as_float_str(&mut fmt, left, right, dp, is_neg, pad_length).unwrap();
        s
    }
    
    #[test]
    fn test_u64_to_str() {
        assert_eq!(f_str(0, 0, 0, false, 0), "0");
        assert_eq!(f_str(0, 0, 1, false, 0), "0.0");
        assert_eq!(f_str(0, 0, 6, false, 0), "0.000000");
        assert_eq!(f_str(0, 2, 6, false, 0), "0.000002");
        assert_eq!(f_str(123, 457, 3, false, 0), "123.457");
        assert_eq!(f_str(123, 457, 3, false, 10), "   123.457");
    }

}