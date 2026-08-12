pub mod traits;

pub use traits::{DigitAdd, DigitSub, DigitMul, DigitDivRem, DigitFromWide};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BaseDigit<const BASE: u128 = 0> {
    Binary(u8),            // Base 2
    Quaternary(u8),        // Base 4
    Octal(u8),             // Base 8
    Decimal(u8),           // Base 10
    Dezonal(u8),           // Base 12
    Hexadecimal(u8),       // Base 16
    Octet(u8),             // Base 256
    Doublet(u16),          // Base 65536
    Quadlet(u32),          // Base 2^32
    Octlet(u64),           // Base 2^64
    DoubleOctlet(u128),    // Base 2^128 (default)
    Other(u128),           // Any other base (including Unary Base 1)
}

impl<const BASE: u128> BaseDigit<BASE> {
    /// Creates a new BaseDigit. Returns Err if value >= BASE (for non-2^128 bases).
    pub fn new(value: u128) -> Result<Self, &'static str> {
        if BASE == 1 {
            return Err("Base cannot be 1");
        }
        if BASE > 1 && value >= BASE {
            return Err("Digit value too high for the base");
        }

        Ok(match BASE {
            2 => Self::Binary(value as u8),
            4 => Self::Quaternary(value as u8),
            8 => Self::Octal(value as u8),
            10 => Self::Decimal(value as u8),
            12 => Self::Dezonal(value as u8),
            16 => Self::Hexadecimal(value as u8),
            256 => Self::Octet(value as u8),
            65536 => Self::Doublet(value as u16),
            4294967296 => Self::Quadlet(value as u32),
            18446744073709551616 => Self::Octlet(value as u64),
            0 => Self::DoubleOctlet(value),
            _ => Self::Other(value),
        })
    }

    /// Returns the inner value of the digit as a u128.
    pub fn value(self) -> u128 {
        match self {
            Self::Binary(v) => v as u128,
            Self::Quaternary(v) => v as u128,
            Self::Octal(v) => v as u128,
            Self::Decimal(v) => v as u128,
            Self::Dezonal(v) => v as u128,
            Self::Hexadecimal(v) => v as u128,
            Self::Octet(v) => v as u128,
            Self::Doublet(v) => v as u128,
            Self::Quadlet(v) => v as u128,
            Self::Octlet(v) => v as u128,
            Self::DoubleOctlet(v) => v,
            Self::Other(v) => v,
        }
    }

    pub fn mul_u128_add_u128(a: u128, b: u128, carry: u128) -> (u128, u128) {
        let a_lo = a as u64 as u128;
        let a_hi = (a >> 64) as u64 as u128;
        let b_lo = b as u64 as u128;
        let b_hi = (b >> 64) as u64 as u128;

        let ll = a_lo * b_lo;
        let lh = a_lo * b_hi;
        let hl = a_hi * b_lo;
        let hh = a_hi * b_hi;

        let (ll_sum, carry_ll) = ll.overflowing_add(carry);
        let carry_ll_val = if carry_ll { 1u128 } else { 0u128 };

        let (sum_lh_hl, overflow_lh_hl) = lh.overflowing_add(hl);
        let carry_lh_hl = if overflow_lh_hl { 1u128 << 64 } else { 0u128 };

        let (sum_mid, overflow_mid) = sum_lh_hl.overflowing_add(ll_sum >> 64);
        let carry_mid = if overflow_mid { 1u128 << 64 } else { 0u128 };

        let final_low = (sum_mid << 64) | (ll_sum & 0xFFFF_FFFF_FFFF_FFFF);
        let final_high = hh + (sum_mid >> 64) + carry_lh_hl + carry_mid + carry_ll_val;
        (final_low, final_high)
    }

    pub fn div_rem_u256_by_u128(high: u128, low: u128, divisor: u128) -> (u128, u128) {
        if divisor == 0 {
            panic!("Division by zero");
        }
        if high >= divisor {
            panic!("Quotient overflow");
        }
        
        let mut q = 0u128;
        let mut r = 0u128;
        
        for i in (0..256).rev() {
            let r_overflow = (r >> 127) & 1;
            r <<= 1;
            let bit = if i >= 128 {
                (high >> (i - 128)) & 1
            } else {
                (low >> i) & 1
            };
            r |= bit;
            
            if r_overflow == 1 || r >= divisor {
                r = r.wrapping_sub(divisor);
                if i < 128 {
                    q |= 1u128 << i;
                }
            }
        }
        (q, r)
    }

    pub fn into_digit<const NEW_BASE: u128>(self) -> Result<BaseDigit<NEW_BASE>, &'static str> {
        BaseDigit::<NEW_BASE>::new(self.value())
    }

    pub fn convert_overflow<const NEW_BASE: u128>(self) -> Result<Vec<BaseDigit<NEW_BASE>>, &'static str> {
        if NEW_BASE == 1 {
            return Err("Base cannot be 1");
        }
        let mut val = self.value();
        let mut result = Vec::new();

        if NEW_BASE == 0 {
            result.push(BaseDigit::<NEW_BASE>::new(val)?);
            result.push(BaseDigit::<NEW_BASE>::new(0)?);
            return Ok(result);
        }

        result.push(BaseDigit::<NEW_BASE>::new(val % NEW_BASE)?);
        val /= NEW_BASE;

        while val > 0 {
            result.push(BaseDigit::<NEW_BASE>::new(val % NEW_BASE)?);
            val /= NEW_BASE;
        }

        if result.len() == 1 {
            result.push(BaseDigit::<NEW_BASE>::new(0)?);
        }

        Ok(result)
    }
}

impl<const BASE: u128> DigitAdd for BaseDigit<BASE> {
    type Output = Self;
    fn add_digit(self, other: Self, carry_in: Self) -> (Self, Self) {
        let a = self.value();
        let b = other.value();
        let c = carry_in.value();

        let (sum_val, carry_out_val) = match BASE {
            0 => {
                let (sum1, overflow1) = a.overflowing_add(b);
                let (sum2, overflow2) = sum1.overflowing_add(c);
                let carry = if overflow1 || overflow2 { 1 } else { 0 };
                (sum2, carry)
            }
            _ => {
                let total = a + b + c;
                (total % BASE, total / BASE)
            }
        };

        (Self::new(sum_val).unwrap(), Self::new(carry_out_val).unwrap())
    }
}

impl<const BASE: u128> DigitSub for BaseDigit<BASE> {
    type Output = Self;
    fn sub_digit(self, other: Self, borrow_in: Self) -> (Self, Self) {
        let a = self.value();
        let b = other.value();
        let borrow = borrow_in.value();

        let (diff_val, borrow_out_val) = match BASE {
            0 => {
                let (diff1, borrow1) = a.overflowing_sub(b);
                let (diff2, borrow2) = diff1.overflowing_sub(borrow);
                let borrow_out = if borrow1 || borrow2 { 1 } else { 0 };
                (diff2, borrow_out)
            }
            _ => {
                let net = (a as i128) - (b as i128) - (borrow as i128);
                if net >= 0 {
                    (net as u128, 0)
                } else {
                    ((net + BASE as i128) as u128, 1)
                }
            }
        };

        (Self::new(diff_val).unwrap(), Self::new(borrow_out_val).unwrap())
    }
}

impl<const BASE: u128> DigitMul for BaseDigit<BASE> {
    type Output = Self;
    fn mul_digit(self, other: Self, carry_in: Self) -> (Self, Self) {
        let a = self.value();
        let b = other.value();
        let c = carry_in.value();

        let (low_val, high_val) = match BASE {
            0 => {
                Self::mul_u128_add_u128(a, b, c)
            }
            _ => {
                let total = (a * b) + c;
                (total % BASE, total / BASE)
            }
        };

        (Self::new(low_val).unwrap(), Self::new(high_val).unwrap())
    }
}

impl<const BASE: u128> DigitDivRem for BaseDigit<BASE> {
    type Output = Self;
    fn div_rem_digit(high: Self, low: Self, divisor: Self) -> (Self, Self) {
        let h = high.value();
        let l = low.value();
        let d = divisor.value();

        let (q_val, r_val) = match BASE {
            0 => {
                Self::div_rem_u256_by_u128(h, l, d)
            }
            _ => {
                let total = h * BASE + l;
                (total / d, total % d)
            }
        };

        (Self::new(q_val).unwrap(), Self::new(r_val).unwrap())
    }
}

impl<const BASE: u128> DigitFromWide<u128> for BaseDigit<BASE> {
    fn from_wide(wide_value: u128) -> (Self, Self) {
        let (q_val, r_val) = match BASE {
            0 => {
                (0, wide_value)
            }
            _ => {
                (wide_value / BASE, wide_value % BASE)
            }
        };
        (Self::new(q_val).unwrap(), Self::new(r_val).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base_digit_creation() {
        // Default base (BASE = 0, represents 2^128)
        let default_digit = BaseDigit::<0>::new(u128::MAX).unwrap();
        assert_eq!(default_digit.value(), u128::MAX);
        assert!(matches!(default_digit, BaseDigit::<0>::DoubleOctlet(_)));

        // Base 10 (Decimal)
        let dec_digit = BaseDigit::<10>::new(9).unwrap();
        assert_eq!(dec_digit.value(), 9);
        assert!(matches!(dec_digit, BaseDigit::Decimal(_)));
        assert!(BaseDigit::<10>::new(10).is_err());

        // Base 12 (Dezonal)
        let dez_digit = BaseDigit::<12>::new(11).unwrap();
        assert_eq!(dez_digit.value(), 11);
        assert!(matches!(dez_digit, BaseDigit::Dezonal(_)));
        assert!(BaseDigit::<12>::new(12).is_err());

        // Other custom base (e.g. Base 50)
        let custom_digit = BaseDigit::<50>::new(49).unwrap();
        assert_eq!(custom_digit.value(), 49);
        assert!(matches!(custom_digit, BaseDigit::Other(_)));
        assert!(BaseDigit::<50>::new(50).is_err());

        // Base 1 (invalid)
        assert!(BaseDigit::<1>::new(0).is_err());
    }

    #[test]
    fn test_base10_arithmetic() {
        let a = BaseDigit::<10>::new(7).unwrap();
        let b = BaseDigit::<10>::new(5).unwrap();
        let zero = BaseDigit::<10>::new(0).unwrap();
        let one = BaseDigit::<10>::new(1).unwrap();

        // 7 + 5 + 0 = 12 -> 2 overflow 1
        let (sum, carry) = a.add_digit(b, zero);
        assert_eq!(sum.value(), 2);
        assert_eq!(carry.value(), 1);

        // 7 - 5 = 2
        let (diff, borrow) = a.sub_digit(b, zero);
        assert_eq!(diff.value(), 2);
        assert_eq!(borrow.value(), 0);

        // 5 - 7 = 8 borrow 1
        let (diff2, borrow2) = b.sub_digit(a, zero);
        assert_eq!(diff2.value(), 8);
        assert_eq!(borrow2.value(), 1);

        // 7 * 5 + 1 = 36 -> 6 carry 3
        let (low, high) = a.mul_digit(b, one);
        assert_eq!(low.value(), 6);
        assert_eq!(high.value(), 3);
    }

    #[test]
    fn test_base_2_128_arithmetic() {
        let max = BaseDigit::<0>::new(u128::MAX).unwrap();
        let zero = BaseDigit::<0>::new(0).unwrap();
        let one = BaseDigit::<0>::new(1).unwrap();

        // u128::MAX + 1 = 0 overflow 1
        let (sum, carry) = max.add_digit(one, zero);
        assert_eq!(sum.value(), 0);
        assert_eq!(carry.value(), 1);

        // u128::MAX * u128::MAX + 0 = (1, u128::MAX - 1)
        let (low, high) = max.mul_digit(max, zero);
        assert_eq!(low.value(), 1);
        assert_eq!(high.value(), u128::MAX - 1);
    }

    #[test]
    fn test_base_digit_conversion() {
        // 12_10 to base 16: [12_16, 0_16]
        // Note: 12 is valid in base 20, let's construct a base 20 digit of value 12
        let digit_12_base20 = BaseDigit::<20>::new(12).unwrap();
        let res_16 = digit_12_base20.convert_overflow::<16>().unwrap();
        assert_eq!(res_16.len(), 2);
        assert_eq!(res_16[0].value(), 12);
        assert_eq!(res_16[1].value(), 0);

        // 10_10 to base 2: [0_2, 1_2, 0_2, 1_2]
        // Let's construct a base 20 digit of value 10
        let digit_10_base20 = BaseDigit::<20>::new(10).unwrap();
        let res_2 = digit_10_base20.convert_overflow::<2>().unwrap();
        assert_eq!(res_2.len(), 4);
        assert_eq!(res_2[0].value(), 0);
        assert_eq!(res_2[1].value(), 1);
        assert_eq!(res_2[2].value(), 0);
        assert_eq!(res_2[3].value(), 1);

        // into_digit tests
        let digit_9_base10 = BaseDigit::<10>::new(9).unwrap();
        let res_into_16 = digit_9_base10.into_digit::<16>().unwrap();
        assert_eq!(res_into_16.value(), 9);

        let digit_18_base20 = BaseDigit::<20>::new(18).unwrap();
        assert!(digit_18_base20.into_digit::<10>().is_err());
    }
}
