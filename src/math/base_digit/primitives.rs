use super::{DigitOperations, DigitFromDoubleWide, DigitOutcome};

impl DigitOperations for u8 {
    type Output = Self;
    fn add_digit(self, other: Self, carry_in: Self) -> Result<DigitOutcome<Self>, crate::math::math_error::MathError> {
        let sum = (self as u16) + (other as u16) + (carry_in as u16);
        let sum_val = sum as u8;
        let carry_out = (sum >> 8) as u8;
        match carry_out == 0 {
            true => Ok(DigitOutcome::NoOverflow(sum_val)),
            false => Ok(DigitOutcome::Overflow(sum_val, carry_out)),
        }
    }
    fn sub_digit(self, other: Self, borrow_in: Self) -> Result<DigitOutcome<Self>, crate::math::math_error::MathError> {
        let diff = (self as i16) - (other as i16) - (borrow_in as i16);
        let (diff_val, borrow_out) = match diff >= 0 {
            true => (diff as u8, 0),
            false => ((diff + 256) as u8, 1),
        };
        match borrow_out == 0 {
            true => Ok(DigitOutcome::NoOverflow(diff_val)),
            false => Ok(DigitOutcome::Overflow(diff_val, borrow_out)),
        }
    }
    fn mul_digit(self, other: Self, carry_in: Self) -> Result<DigitOutcome<Self>, crate::math::math_error::MathError> {
        let prod = (self as u16) * (other as u16) + (carry_in as u16);
        let low = prod as u8;
        let high = (prod >> 8) as u8;
        match high == 0 {
            true => Ok(DigitOutcome::NoOverflow(low)),
            false => Ok(DigitOutcome::Overflow(low, high)),
        }
    }
    fn div_rem_digit(high: Self, low: Self, divisor: Self) -> Result<DigitOutcome<Self>, crate::math::math_error::MathError> {
        use crate::math::math_error::MathError;
        match divisor == 0 {
            true => return Err(MathError::DivisionByZero),
            false => {}
        }
        match high >= divisor {
            true => return Err(MathError::QuotientOverflow),
            false => {}
        }
        let dividend = ((high as u16) << 8) | (low as u16);
        let q = (dividend / (divisor as u16)) as u8;
        let r = (dividend % (divisor as u16)) as u8;
        match r == 0 {
            true => Ok(DigitOutcome::NoOverflow(q)),
            false => Ok(DigitOutcome::Overflow(q, r)),
        }
    }
}

impl DigitFromDoubleWide<u16> for u8 {
    fn from_double_wide(double_wide_value: u16) -> (Self, Self) {
        ((double_wide_value >> 8) as u8, double_wide_value as u8)
    }
}

impl DigitOperations for u16 {
    type Output = Self;
    fn add_digit(self, other: Self, carry_in: Self) -> Result<DigitOutcome<Self>, crate::math::math_error::MathError> {
        let sum = (self as u32) + (other as u32) + (carry_in as u32);
        let sum_val = sum as u16;
        let carry_out = (sum >> 16) as u16;
        match carry_out == 0 {
            true => Ok(DigitOutcome::NoOverflow(sum_val)),
            false => Ok(DigitOutcome::Overflow(sum_val, carry_out)),
        }
    }
    fn sub_digit(self, other: Self, borrow_in: Self) -> Result<DigitOutcome<Self>, crate::math::math_error::MathError> {
        let diff = (self as i32) - (other as i32) - (borrow_in as i32);
        let (diff_val, borrow_out) = match diff >= 0 {
            true => (diff as u16, 0),
            false => ((diff + 65536) as u16, 1),
        };
        match borrow_out == 0 {
            true => Ok(DigitOutcome::NoOverflow(diff_val)),
            false => Ok(DigitOutcome::Overflow(diff_val, borrow_out)),
        }
    }
    fn mul_digit(self, other: Self, carry_in: Self) -> Result<DigitOutcome<Self>, crate::math::math_error::MathError> {
        let prod = (self as u32) * (other as u32) + (carry_in as u32);
        let low = prod as u16;
        let high = (prod >> 16) as u16;
        match high == 0 {
            true => Ok(DigitOutcome::NoOverflow(low)),
            false => Ok(DigitOutcome::Overflow(low, high)),
        }
    }
    fn div_rem_digit(high: Self, low: Self, divisor: Self) -> Result<DigitOutcome<Self>, crate::math::math_error::MathError> {
        use crate::math::math_error::MathError;
        match divisor == 0 {
            true => return Err(MathError::DivisionByZero),
            false => {}
        }
        match high >= divisor {
            true => return Err(MathError::QuotientOverflow),
            false => {}
        }
        let dividend = ((high as u32) << 16) | (low as u32);
        let q = (dividend / (divisor as u32)) as u16;
        let r = (dividend % (divisor as u32)) as u16;
        match r == 0 {
            true => Ok(DigitOutcome::NoOverflow(q)),
            false => Ok(DigitOutcome::Overflow(q, r)),
        }
    }
}

impl DigitFromDoubleWide<u32> for u16 {
    fn from_double_wide(double_wide_value: u32) -> (Self, Self) {
        ((double_wide_value >> 16) as u16, double_wide_value as u16)
    }
}

impl DigitOperations for u32 {
    type Output = Self;
    fn add_digit(self, other: Self, carry_in: Self) -> Result<DigitOutcome<Self>, crate::math::math_error::MathError> {
        let sum = (self as u64) + (other as u64) + (carry_in as u64);
        let sum_val = sum as u32;
        let carry_out = (sum >> 32) as u32;
        match carry_out == 0 {
            true => Ok(DigitOutcome::NoOverflow(sum_val)),
            false => Ok(DigitOutcome::Overflow(sum_val, carry_out)),
        }
    }
    fn sub_digit(self, other: Self, borrow_in: Self) -> Result<DigitOutcome<Self>, crate::math::math_error::MathError> {
        let diff = (self as i64) - (other as i64) - (borrow_in as i64);
        let (diff_val, borrow_out) = match diff >= 0 {
            true => (diff as u32, 0),
            false => ((diff + 4294967296) as u32, 1),
        };
        match borrow_out == 0 {
            true => Ok(DigitOutcome::NoOverflow(diff_val)),
            false => Ok(DigitOutcome::Overflow(diff_val, borrow_out)),
        }
    }
    fn mul_digit(self, other: Self, carry_in: Self) -> Result<DigitOutcome<Self>, crate::math::math_error::MathError> {
        let prod = (self as u64) * (other as u64) + (carry_in as u64);
        let low = prod as u32;
        let high = (prod >> 32) as u32;
        match high == 0 {
            true => Ok(DigitOutcome::NoOverflow(low)),
            false => Ok(DigitOutcome::Overflow(low, high)),
        }
    }
    fn div_rem_digit(high: Self, low: Self, divisor: Self) -> Result<DigitOutcome<Self>, crate::math::math_error::MathError> {
        use crate::math::math_error::MathError;
        match divisor == 0 {
            true => return Err(MathError::DivisionByZero),
            false => {}
        }
        match high >= divisor {
            true => return Err(MathError::QuotientOverflow),
            false => {}
        }
        let dividend = ((high as u64) << 32) | (low as u64);
        let q = (dividend / (divisor as u64)) as u32;
        let r = (dividend % (divisor as u64)) as u32;
        match r == 0 {
            true => Ok(DigitOutcome::NoOverflow(q)),
            false => Ok(DigitOutcome::Overflow(q, r)),
        }
    }
}

impl DigitFromDoubleWide<u64> for u32 {
    fn from_double_wide(double_wide_value: u64) -> (Self, Self) {
        ((double_wide_value >> 32) as u32, double_wide_value as u32)
    }
}

impl DigitOperations for u64 {
    type Output = Self;
    fn add_digit(self, other: Self, carry_in: Self) -> Result<DigitOutcome<Self>, crate::math::math_error::MathError> {
        let sum = (self as u128) + (other as u128) + (carry_in as u128);
        let sum_val = sum as u64;
        let carry_out = (sum >> 64) as u64;
        match carry_out == 0 {
            true => Ok(DigitOutcome::NoOverflow(sum_val)),
            false => Ok(DigitOutcome::Overflow(sum_val, carry_out)),
        }
    }
    fn sub_digit(self, other: Self, borrow_in: Self) -> Result<DigitOutcome<Self>, crate::math::math_error::MathError> {
        let diff = (self as i128) - (other as i128) - (borrow_in as i128);
        let (diff_val, borrow_out) = match diff >= 0 {
            true => (diff as u64, 0),
            false => ((diff + 18446744073709551616) as u64, 1),
        };
        match borrow_out == 0 {
            true => Ok(DigitOutcome::NoOverflow(diff_val)),
            false => Ok(DigitOutcome::Overflow(diff_val, borrow_out)),
        }
    }
    fn mul_digit(self, other: Self, carry_in: Self) -> Result<DigitOutcome<Self>, crate::math::math_error::MathError> {
        let prod = (self as u128) * (other as u128) + (carry_in as u128);
        let low = prod as u64;
        let high = (prod >> 64) as u64;
        match high == 0 {
            true => Ok(DigitOutcome::NoOverflow(low)),
            false => Ok(DigitOutcome::Overflow(low, high)),
        }
    }
    fn div_rem_digit(high: Self, low: Self, divisor: Self) -> Result<DigitOutcome<Self>, crate::math::math_error::MathError> {
        use crate::math::math_error::MathError;
        match divisor == 0 {
            true => return Err(MathError::DivisionByZero),
            false => {}
        }
        match high >= divisor {
            true => return Err(MathError::QuotientOverflow),
            false => {}
        }
        let dividend = ((high as u128) << 64) | (low as u128);
        let q = (dividend / (divisor as u128)) as u64;
        let r = (dividend % (divisor as u128)) as u64;
        match r == 0 {
            true => Ok(DigitOutcome::NoOverflow(q)),
            false => Ok(DigitOutcome::Overflow(q, r)),
        }
    }
}

impl DigitFromDoubleWide<u128> for u64 {
    fn from_double_wide(double_wide_value: u128) -> (Self, Self) {
        ((double_wide_value >> 64) as u64, double_wide_value as u64)
    }
}

impl DigitOperations for u128 {
    type Output = Self;
    fn add_digit(self, other: Self, carry_in: Self) -> Result<DigitOutcome<Self>, crate::math::math_error::MathError> {
        let (sum1, overflow1) = self.overflowing_add(other);
        let (sum2, overflow2) = sum1.overflowing_add(carry_in);
        let carry = match overflow1 || overflow2 {
            true => 1,
            false => 0,
        };
        match carry == 0 {
            true => Ok(DigitOutcome::NoOverflow(sum2)),
            false => Ok(DigitOutcome::Overflow(sum2, carry)),
        }
    }
    fn sub_digit(self, other: Self, borrow_in: Self) -> Result<DigitOutcome<Self>, crate::math::math_error::MathError> {
        let (diff1, borrow1) = self.overflowing_sub(other);
        let (diff2, borrow2) = diff1.overflowing_sub(borrow_in);
        let borrow = match borrow1 || borrow2 {
            true => 1,
            false => 0,
        };
        match borrow == 0 {
            true => Ok(DigitOutcome::NoOverflow(diff2)),
            false => Ok(DigitOutcome::Overflow(diff2, borrow)),
        }
    }
    fn mul_digit(self, other: Self, carry_in: Self) -> Result<DigitOutcome<Self>, crate::math::math_error::MathError> {
        let a_lo = self as u64 as u128;
        let a_hi = (self >> 64) as u64 as u128;
        let b_lo = other as u64 as u128;
        let b_hi = (other >> 64) as u64 as u128;

        let ll = a_lo * b_lo;
        let lh = a_lo * b_hi;
        let hl = a_hi * b_lo;
        let hh = a_hi * b_hi;

        let (ll_sum, carry_ll) = ll.overflowing_add(carry_in);
        let carry_ll_val = match carry_ll {
            true => 1,
            false => 0,
        };

        let (sum_lh_hl, overflow_lh_hl) = lh.overflowing_add(hl);
        let carry_lh_hl = match overflow_lh_hl {
            true => 1 << 64,
            false => 0,
        };

        let (sum_mid, overflow_mid) = sum_lh_hl.overflowing_add(ll_sum >> 64);
        let carry_mid = match overflow_mid {
            true => 1 << 64,
            false => 0,
        };

        let final_low = (sum_mid << 64) | (ll_sum & 0xFFFF_FFFF_FFFF_FFFF);
        let final_high = hh + (sum_mid >> 64) + carry_lh_hl + carry_mid + carry_ll_val;
        match final_high == 0 {
            true => Ok(DigitOutcome::NoOverflow(final_low)),
            false => Ok(DigitOutcome::Overflow(final_low, final_high)),
        }
    }
    fn div_rem_digit(high: Self, low: Self, divisor: Self) -> Result<DigitOutcome<Self>, crate::math::math_error::MathError> {
        use crate::math::math_error::MathError;
        match divisor == 0 {
            true => return Err(MathError::DivisionByZero),
            false => {}
        }
        match high >= divisor {
            true => return Err(MathError::QuotientOverflow),
            false => {}
        }

        let mut q = 0u128;
        let mut r = 0u128;

        for i in (0..256).rev() {
            let r_overflow = (r >> 127) & 1;
            r <<= 1;
            let bit = match i >= 128 {
                true => (high >> (i - 128)) & 1,
                false => (low >> i) & 1,
            };
            r |= bit;

            match r_overflow == 1 || r >= divisor {
                true => {
                    r = r.wrapping_sub(divisor);
                    match i < 128 {
                        true => q |= 1u128 << i,
                        false => {}
                    }
                }
                false => {}
            }
        }
        match r == 0 {
            true => Ok(DigitOutcome::NoOverflow(q)),
            false => Ok(DigitOutcome::Overflow(q, r)),
        }
    }
}

impl DigitFromDoubleWide<u128> for u128 {
    fn from_double_wide(double_wide_value: u128) -> (Self, Self) {
        (0, double_wide_value)
    }
}
