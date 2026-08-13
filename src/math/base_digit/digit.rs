use super::traits::{DigitOperations, DigitFromDoubleWide, DigitOutcome};

/// Represents a single digit in a positional numeral system with base `BASE`.
///
/// ### Mathematical Background
/// A positional numeral system with base $B \ge 2$ represents numbers using a set of $B$ unique symbols (digits).
/// Each digit $d$ in the representation of a number represents a coefficient for a power of the base $B$.
///
/// In this type, a digit $d$ is represented by the `Digit<BASE>` enum and satisfies the constraint:
/// $$0 \le d < \text{BASE}$$
///
/// For example, if `BASE` is 10, the digit values range from 0 to 9. If `BASE` is 256, the digit values range from 0 to 255.
/// Special variants are provided for common bases (like `Binary`, `Decimal`, `Hexadecimal`, `Octet`) for optimized backing storage representations.
///
/// ### Base Conversion
/// A value $V$ represented in a source base can be converted into a target base $B_{\text{new}}$ by repeated division:
/// $$V_0 = V$$
/// $$V_{i} = q_{i} \cdot B_{\text{new}} + d_{i}$$
/// where each remainder $d_{i} = V_{i} \bmod B_{\text{new}}$ forms the next digit (from least to most significant), and the quotient $q_{i} = \lfloor V_{i} / B_{\text{new}} \rfloor$ becomes the dividend for the next step. The process terminates when $q_{i} = 0$.
///
/// This repeated division algorithm is implemented in [`Digit::convert_overflow`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Digit<const BASE: u128 = 0> {
    Binary(u8),         // Base 2
    Quaternary(u8),     // Base 4
    Octal(u8),          // Base 8
    Decimal(u8),        // Base 10
    Dezonal(u8),        // Base 12
    Hexadecimal(u8),    // Base 16
    Octet(u8),          // Base 256
    Doublet(u16),       // Base 65536
    Quadlet(u32),       // Base 2^32
    Octlet(u64),        // Base 2^64
    DoubleOctlet(u128), // Base 2^128 (default)
    Other(u128),        // Any other base (including Unary Base 1)
}

impl<const BASE: u128> Digit<BASE> {
    /// Creates a new Digit. Returns Err if value >= BASE (for non-2^128 bases).
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



    pub fn into_digit<const NEW_BASE: u128>(self) -> Result<Digit<NEW_BASE>, &'static str> {
        Digit::<NEW_BASE>::new(self.value())
    }

    pub fn convert_overflow<const NEW_BASE: u128>(
        self,
    ) -> Result<Vec<Digit<NEW_BASE>>, &'static str> {
        match NEW_BASE {
            1 => return Err("Base cannot be 1"),
            0 => {
                let mut result = Vec::new();
                result.push(Digit::<NEW_BASE>::new(self.value())?);
                result.push(Digit::<NEW_BASE>::new(0)?);
                return Ok(result);
            }
            _ => {}
        }
        let mut val = self.value();
        let mut result = Vec::new();

        result.push(Digit::<NEW_BASE>::new(val % NEW_BASE)?);
        val /= NEW_BASE;

        while val > 0 {
            result.push(Digit::<NEW_BASE>::new(val % NEW_BASE)?);
            val /= NEW_BASE;
        }

        match result.len() {
            1 => result.push(Digit::<NEW_BASE>::new(0)?),
            _ => {}
        }

        Ok(result)
    }

    pub fn check_same_base(&self, other: &Self) -> Result<(), crate::math::math_error::MathError> {
        use crate::math::math_error::MathError;
        match (self, other) {
            (Self::Binary(_), Self::Binary(_)) => Ok(()),
            (Self::Quaternary(_), Self::Quaternary(_)) => Ok(()),
            (Self::Octal(_), Self::Octal(_)) => Ok(()),
            (Self::Decimal(_), Self::Decimal(_)) => Ok(()),
            (Self::Dezonal(_), Self::Dezonal(_)) => Ok(()),
            (Self::Hexadecimal(_), Self::Hexadecimal(_)) => Ok(()),
            (Self::Octet(_), Self::Octet(_)) => Ok(()),
            (Self::Doublet(_), Self::Doublet(_)) => Ok(()),
            (Self::Quadlet(_), Self::Quadlet(_)) => Ok(()),
            (Self::Octlet(_), Self::Octlet(_)) => Ok(()),
            (Self::DoubleOctlet(_), Self::DoubleOctlet(_)) => Ok(()),
            (Self::Other(_), Self::Other(_)) => Ok(()),
            _ => Err(MathError::BaseMismatch),
        }
    }
}

impl<const BASE: u128> DigitOperations for Digit<BASE> {
    type Output = Self;

    fn add_digit(self, other: Self, carry_in: Self) -> Result<DigitOutcome<Self>, crate::math::math_error::MathError> {
        use crate::math::math_error::MathError;
        self.check_same_base(&other)?;
        self.check_same_base(&carry_in)?;

        let a = self.value();
        let b = other.value();
        let c = carry_in.value();

        let (sum_val, carry_out_val) = match BASE {
            0 => {
                match a.add_digit(b, c)? {
                    DigitOutcome::NoOverflow(s) => (s, 0),
                    DigitOutcome::Overflow(s, co) => (s, co),
                }
            }
            _ => {
                let total = a + b + c;
                (total % BASE, total / BASE)
            }
        };

        let sum = Self::new(sum_val).unwrap();
        match carry_out_val == 0 {
            true => Ok(DigitOutcome::NoOverflow(sum)),
            false => Ok(DigitOutcome::Overflow(sum, Self::new(carry_out_val).unwrap())),
        }
    }

    fn sub_digit(self, other: Self, borrow_in: Self) -> Result<DigitOutcome<Self>, crate::math::math_error::MathError> {
        use crate::math::math_error::MathError;
        self.check_same_base(&other)?;
        self.check_same_base(&borrow_in)?;

        let a = self.value();
        let b = other.value();
        let borrow = borrow_in.value();

        let (diff_val, borrow_out_val) = match BASE {
            0 => {
                match a.sub_digit(b, borrow)? {
                    DigitOutcome::NoOverflow(d) => (d, 0),
                    DigitOutcome::Overflow(d, bo) => (d, bo),
                }
            }
            _ => {
                let net = (a as i128) - (b as i128) - (borrow as i128);
                match net >= 0 {
                    true => (net as u128, 0),
                    false => ((net + BASE as i128) as u128, 1),
                }
            }
        };

        let diff = Self::new(diff_val).unwrap();
        match borrow_out_val == 0 {
            true => Ok(DigitOutcome::NoOverflow(diff)),
            false => Ok(DigitOutcome::Overflow(diff, Self::new(borrow_out_val).unwrap())),
        }
    }

    fn mul_digit(self, other: Self, carry_in: Self) -> Result<DigitOutcome<Self>, crate::math::math_error::MathError> {
        use crate::math::math_error::MathError;
        self.check_same_base(&other)?;
        self.check_same_base(&carry_in)?;

        let a = self.value();
        let b = other.value();
        let c = carry_in.value();

        let (low_val, high_val) = match BASE {
            0 => {
                match a.mul_digit(b, c)? {
                    DigitOutcome::NoOverflow(l) => (l, 0),
                    DigitOutcome::Overflow(l, h) => (l, h),
                }
            }
            _ => {
                let total = (a * b) + c;
                (total % BASE, total / BASE)
            }
        };

        let low = Self::new(low_val).unwrap();
        match high_val == 0 {
            true => Ok(DigitOutcome::NoOverflow(low)),
            false => Ok(DigitOutcome::Overflow(low, Self::new(high_val).unwrap())),
        }
    }

    fn div_rem_digit(high: Self, low: Self, divisor: Self) -> Result<DigitOutcome<Self>, crate::math::math_error::MathError> {
        use crate::math::math_error::MathError;
        high.check_same_base(&low)?;
        high.check_same_base(&divisor)?;

        let h = high.value();
        let l = low.value();
        let d = divisor.value();

        let (q_val, r_val) = match BASE {
            0 => {
                match DigitOperations::div_rem_digit(h, l, d)? {
                    DigitOutcome::NoOverflow(q) => (q, 0),
                    DigitOutcome::Overflow(q, r) => (q, r),
                }
            }
            _ => {
                match d == 0 {
                    true => return Err(MathError::DivisionByZero),
                    false => {}
                }
                match h >= d {
                    true => return Err(MathError::QuotientOverflow),
                    false => {}
                }
                let total = h * BASE + l;
                (total / d, total % d)
            }
        };

        let q = Self::new(q_val).unwrap();
        match r_val == 0 {
            true => Ok(DigitOutcome::NoOverflow(q)),
            false => Ok(DigitOutcome::Overflow(q, Self::new(r_val).unwrap())),
        }
    }
}

impl<const BASE: u128> DigitFromDoubleWide<u128> for Digit<BASE> {
    fn from_double_wide(double_wide_value: u128) -> (Self, Self) {
        let (q_val, r_val) = match BASE {
            0 => <u128 as DigitFromDoubleWide<u128>>::from_double_wide(double_wide_value),
            _ => (double_wide_value / BASE, double_wide_value % BASE),
        };
        (Self::new(q_val).unwrap(), Self::new(r_val).unwrap())
    }
}


