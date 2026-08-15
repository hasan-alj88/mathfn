use crate::math::base_digit::Digit;
use crate::math::math_error::MathError;

/// An arbitrary-precision, non-negative integer represented in a positional numeral system with base `BASE`.
///
/// ### Mathematical Representation
/// Any non-negative integer $N$ can be uniquely represented in base $B \ge 2$ as a sequence of coefficients (digits):
/// $$N = \sum_{i=0}^{k} d_i B^i$$
/// where each digit $d_i$ satisfies $0 \le d_i < B$.
///
/// In this type, the number is stored as a vector of `Digit<BASE>` in **Least Significant Digit (LSD) first** order:
/// $$\text{digits} = [d_0, d_1, \dots, d_k]$$
///
/// ### Normalization Invariant
/// To ensure a unique representation for each integer, the digit sequence is normalized.
/// Except for the number zero itself, the most significant digit (the last element in our LSD vector) must be non-zero:
/// $$d_k \ne 0$$
///
/// Under this invariant, the value zero is represented by an empty vector of digits:
/// $$\text{zero} \equiv []$$
///
/// Whenever a `NaturalNumber` is constructed or undergoes arithmetic operations, it is normalized automatically
/// by removing trailing zero digits from the internal vector.
///
/// ### Arithmetic Algorithms
/// - **Addition**: Carried out digit-wise using the schoolbook carrying algorithm.
/// - **Subtraction**: Performed using schoolbook borrow propagation (only defined when $A \ge B$ to preserve non-negativity).
/// - **Multiplication**: Supports both standard schoolbook $O(n^2)$ multiplication and the Karatsuba algorithm.
///   The Karatsuba algorithm splits $n$-digit numbers into high and low parts:
///   $$A = A_{hi} B^m + A_{lo}, \quad B = B_{hi} B^m + B_{lo}$$
///   where $m = \lfloor n/2 \rfloor$. It computes the product using only 3 multiplications instead of 4:
///   $$A \cdot B = P_1 B^{2m} + (P_3 - P_1 - P_2) B^m + P_2$$
///   where $P_1 = A_{hi} B_{hi}$, $P_2 = A_{lo} B_{lo}$, and $P_3 = (A_{hi} + A_{lo})(B_{hi} + B_{lo})$.
///   This reduces complexity to $O(n^{\log_2 3}) \approx O(n^{1.585})$.
/// - **Exponentiation**: Implemented using binary exponentiation by squaring in $O(\log e)$ multiplications.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NaturalNumber<const BASE: u128 = 256> {
    digits: Vec<Digit<BASE>>,
}

impl<const BASE: u128> NaturalNumber<BASE> {
    pub fn new(digits: Vec<Digit<BASE>>) -> Self {
        let mut num = Self { digits };
        num.remove_tailing_zero_digits();
        num
    }

    pub fn from_u128(mut value: u128) -> Result<Self, MathError> {
        let mut digits = Vec::new();
        while value > 0 {
            let digit_val = value % BASE;
            digits.push(Digit::new(digit_val).map_err(|_| MathError::BaseMismatch)?);
            value /= BASE;
        }
        Ok(Self::new(digits))
    }

    pub fn to_u128(&self) -> Result<u128, MathError> {
        let (value, _) = self.digits.iter().try_fold((0u128, 1u128), |(acc_val, acc_pow), digit| {
            let term = digit.value().checked_mul(acc_pow).ok_or(MathError::QuotientOverflow)?;
            let next_val = acc_val.checked_add(term).ok_or(MathError::QuotientOverflow)?;
            let next_pow = acc_pow.checked_mul(BASE).ok_or(MathError::QuotientOverflow)?;
            Ok((next_val, next_pow))
        })?;
        Ok(value)
    }

    fn remove_tailing_zero_digits(&mut self) {
        while let Some(last) = self.digits.last() {
            match last.value() {
                0 => {
                    self.digits.pop();
                }
                _ => break,
            }
        }
    }

    pub fn is_zero(&self) -> bool {
        self.digits.is_empty()
    }

    pub fn digits(&self) -> &[Digit<BASE>] {
        &self.digits
    }
}

impl<const BASE: u128> TryFrom<Digit<BASE>> for NaturalNumber<BASE> {
    type Error = MathError;
    fn try_from(digit: Digit<BASE>) -> Result<Self, Self::Error> {
        Ok(Self::new(vec![digit]))
    }
}

impl<const BASE: u128> TryFrom<NaturalNumber<BASE>> for Digit<BASE> {
    type Error = MathError;
    fn try_from(num: NaturalNumber<BASE>) -> Result<Self, Self::Error> {
        match num.digits.len() {
            0 => Digit::new(0).map_err(|_| MathError::BaseMismatch),
            1 => Ok(num.digits[0]),
            _ => Err(MathError::QuotientOverflow),
        }
    }
}

impl<const BASE: u128> TryFrom<Vec<Digit<BASE>>> for NaturalNumber<BASE> {
    type Error = MathError;
    fn try_from(digits: Vec<Digit<BASE>>) -> Result<Self, Self::Error> {
        Ok(Self::new(digits))
    }
}

impl<const BASE: u128> TryFrom<NaturalNumber<BASE>> for Vec<Digit<BASE>> {
    type Error = MathError;
    fn try_from(num: NaturalNumber<BASE>) -> Result<Self, Self::Error> {
        Ok(num.digits)
    }
}

impl<const BASE: u128> TryFrom<Vec<u128>> for NaturalNumber<BASE> {
    type Error = MathError;
    fn try_from(raw_digits: Vec<u128>) -> Result<Self, Self::Error> {
        let mut digits = Vec::new();
        for val in raw_digits {
            digits.push(Digit::new(val).map_err(|_| MathError::BaseMismatch)?);
        }
        Ok(Self::new(digits))
    }
}

macro_rules! impl_try_from_unsigned {
    ($($t:ty),*) => {
        $(
            impl<const BASE: u128> TryFrom<$t> for NaturalNumber<BASE> {
                type Error = MathError;
                fn try_from(value: $t) -> Result<Self, Self::Error> {
                    Self::from_u128(value as u128)
                }
            }
        )*
    };
}
impl_try_from_unsigned!(u128, u64, u32, u16, u8, usize);

macro_rules! impl_try_from_signed {
    ($($t:ty),*) => {
        $(
            impl<const BASE: u128> TryFrom<$t> for NaturalNumber<BASE> {
                type Error = MathError;
                fn try_from(value: $t) -> Result<Self, Self::Error> {
                    match value.cmp(&0) {
                        std::cmp::Ordering::Less => Err(MathError::ResultNotInDomain {
                            this_domain: "NaturalNumbers".to_string(),
                            result_domain: "Signed negative".to_string(),
                        }),
                        _ => Self::from_u128(value as u128),
                    }
                }
            }
        )*
    };
}
impl_try_from_signed!(i128, i64, i32, i16, i8, isize);

macro_rules! impl_try_into_unsigned {
    ($($t:ty),*) => {
        $(
            impl<const BASE: u128> TryFrom<NaturalNumber<BASE>> for $t {
                type Error = MathError;
                fn try_from(num: NaturalNumber<BASE>) -> Result<Self, Self::Error> {
                    let val_u128 = num.to_u128()?;
                    <$t>::try_from(val_u128).map_err(|_| MathError::QuotientOverflow)
                }
            }
        )*
    };
}
impl_try_into_unsigned!(u128, u64, u32, u16, u8, usize);

macro_rules! impl_try_into_signed {
    ($($t:ty),*) => {
        $(
            impl<const BASE: u128> TryFrom<NaturalNumber<BASE>> for $t {
                type Error = MathError;
                fn try_from(num: NaturalNumber<BASE>) -> Result<Self, Self::Error> {
                    let val_u128 = num.to_u128()?;
                    <$t>::try_from(val_u128).map_err(|_| MathError::QuotientOverflow)
                }
            }
        )*
    };
}
impl_try_into_signed!(i128, i64, i32, i16, i8, isize);

impl<const BASE: u128> Ord for NaturalNumber<BASE> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.digits.len().cmp(&other.digits.len()) {
            std::cmp::Ordering::Equal => {
                self.digits.iter().zip(other.digits.iter()).rev()
                    .map(|(a, b)| a.value().cmp(&b.value()))
                    .find(|&ord| match ord {
                        std::cmp::Ordering::Equal => false,
                        _ => true,
                    })
                    .unwrap_or(std::cmp::Ordering::Equal)
            }
            non_eq => non_eq,
        }
    }
}

impl<const BASE: u128> PartialOrd for NaturalNumber<BASE> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<const BASE: u128> crate::math::operations::NumberType<BASE> for NaturalNumber<BASE> {
    fn digit(&self, pos: i64) -> Result<crate::math::base_digit::Digit<BASE>, crate::math::math_error::MathError> {
        let zero_digit = crate::math::base_digit::Digit::new(0).unwrap();
        if pos >= 0 {
            Ok(self.digits()
                .get(pos as usize)
                .copied()
                .unwrap_or(zero_digit))
        } else {
            Ok(zero_digit)
        }
    }
}

impl<const BASE: u128> std::ops::Add for NaturalNumber<BASE> {
    type Output = Result<Self, MathError>;

    fn add(self, rhs: Self) -> Self::Output {
        crate::math::natural_number::addition::nat_add_schoolbook(&self, &rhs)
    }
}

impl<const BASE: u128> std::ops::Mul for NaturalNumber<BASE> {
    type Output = Result<Self, MathError>;

    fn mul(self, rhs: Self) -> Self::Output {
        crate::math::natural_number::multiplication::nat_mul_karatsuba(&self, &rhs)
    }
}



