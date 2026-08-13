use crate::math::base_digit::Digit;
use crate::math::math_error::MathError;

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
