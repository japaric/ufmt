use crate::{uDisplay, uDisplayFloat, uWrite, Formatter};

impl uDisplayFloat for f32 {
    fn fmt_float<W>(&self, f: &mut Formatter<'_, W>, decimal_places: u8) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        // General checks for validity and overflow
        if self.is_nan() {
            f.write_str("NaN")?;
            return Ok(());
        }

        if *self > 8388608.0 { // 2**23
            f.write_str("inf")?;
            return Ok(());
        }

        if *self < -8388608.0 { // 2**23
            f.write_str("-inf")?;
            return Ok(());
        }

        // Calculate integer auxiliary values
        let is_negative = self.is_sign_negative();
        let precision = match decimal_places {
            0 => 1.0,
            1 => 10.0,
            2 => 100.0,
            3 => 1000.0,
            4 => 10000.0,
            5 => 100000.0,
            6 => 1000000.0,
            7 => 10000000.0,
            8 => 100000000.0,
            9 => 1000000000.0,
            _ => 1.0,
        };
        let (before_dp, after_dp) = if is_negative {
            let f = (*self * precision - 0.5) / precision;
            let before_dp = f as i32;
            let after_dp = ((-f + (before_dp as f32)) * precision) as u32;
            (before_dp, after_dp)
        } else {
            let f = (*self * precision + 0.5) / precision;
            let before_dp = f as i32;
            let after_dp = ((f - (before_dp as f32)) * precision) as u32;
            (before_dp, after_dp)
        };

        // Output values
        if decimal_places > 0 && is_negative && *self > -1. {
            f.write_char('-')?;
        }
        before_dp.fmt(f)?;
        if decimal_places > 0 {
            f.write_char('.')?;
            let len = match after_dp {
                0..=9 => 1,
                10..=99 => 2,
                100..=999 => 3,
                1000..=9999 => 4,
                10000..=99999 => 5,
                100000..=999999 => 6,
                1000000..=9999999 => 7,
                10000000..=99999999 => 8,
                100000000..=999999999 => 9,
                _ => decimal_places,
            };
            for _ in len..decimal_places {
                f.write_char('0')?;
            }
            after_dp.fmt(f)?;
        }
        Ok(())
    }
}

impl uDisplayFloat for f64 {
    fn fmt_float<W>(&self, f: &mut Formatter<'_, W>, decimal_places: u8) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        // General checks for validity and overflow
        if self.is_nan() {
            f.write_str("NaN")?;
            return Ok(());
        }

        if *self > 4503599627370496.0 { // 2**52
            f.write_str("inf")?;
            return Ok(());
        }

        if *self < -4503599627370496.0 { // 2**52
            f.write_str("-inf")?;
            return Ok(());
        }

        // Calculate integer auxiliary values
        let is_negative = self.is_sign_negative();
        let precision = match decimal_places {
            0 => 1.0,
            1 => 10.0,
            2 => 100.0,
            3 => 1000.0,
            4 => 10000.0,
            5 => 100000.0,
            6 => 1000000.0,
            7 => 10000000.0,
            8 => 100000000.0,
            9 => 1000000000.0,
            _ => 1.0,
        };
        let (before_dp, after_dp) = if is_negative {
            let f = (*self * precision - 0.5) / precision;
            let before_dp = f as i64;
            let after_dp = ((-f + (before_dp as f64)) * precision) as u32;
            (before_dp, after_dp)
        } else {
            let f = (*self * precision + 0.5) / precision;
            let before_dp = f as i64;
            let after_dp = ((f - (before_dp as f64)) * precision) as u32;
            (before_dp, after_dp)
        };

        // Output values
        if decimal_places > 0 && is_negative && *self > -1. {
            f.write_char('-')?;
        }
        before_dp.fmt(f)?;
        if decimal_places > 0 {
            f.write_char('.')?;
            let len = match after_dp {
                0..=9 => 1,
                10..=99 => 2,
                100..=999 => 3,
                1000..=9999 => 4,
                10000..=99999 => 5,
                100000..=999999 => 6,
                1000000..=9999999 => 7,
                10000000..=99999999 => 8,
                100000000..=999999999 => 9,
                _ => decimal_places,
            };
            for _ in len..decimal_places {
                f.write_char('0')?;
            }
            after_dp.fmt(f)?;
        }
        Ok(())
    }
}
