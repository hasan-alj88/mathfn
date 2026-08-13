use crate::math::sign::Sign;
use crate::math::positive_natural::PositiveNaturalNumber;
use crate::math::natural_number::NaturalNumber;
use crate::math::math_error::MathError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IntegerNumber<const BASE: u128 = 256> {
    Positive(PositiveNaturalNumber<BASE>),
    Negative(PositiveNaturalNumber<BASE>),
    Zero,
}

impl<const BASE: u128> IntegerNumber<BASE> {
    pub fn sign(&self) -> Sign {
        match self {
            Self::Positive(_) => Sign::Positive,
            Self::Negative(_) => Sign::Negative,
            Self::Zero => Sign::Zero,
        }
    }

    pub fn abs(&self) -> Option<PositiveNaturalNumber<BASE>> {
        match self {
            Self::Positive(abs) | Self::Negative(abs) => Some(abs.clone()),
            Self::Zero => None,
        }
    }
}

impl<const BASE: u128> From<NaturalNumber<BASE>> for IntegerNumber<BASE> {
    fn from(num: NaturalNumber<BASE>) -> Self {
        match num.is_zero() {
            true => Self::Zero,
            false => Self::Positive(PositiveNaturalNumber::try_from(num).unwrap()),
        }
    }
}

impl<const BASE: u128> TryFrom<IntegerNumber<BASE>> for NaturalNumber<BASE> {
    type Error = MathError;
    fn try_from(num: IntegerNumber<BASE>) -> Result<Self, Self::Error> {
        match num {
            IntegerNumber::Zero => Ok(NaturalNumber::new(Vec::new())),
            IntegerNumber::Positive(abs) => Ok(NaturalNumber::from(abs)),
            IntegerNumber::Negative(_) => Err(MathError::ResultNotInDomain {
                this_domain: "NaturalNumbers".to_string(),
                result_domain: "NegativeIntegers".to_string(),
            }),
        }
    }
}

macro_rules! impl_try_from_signed_primitive {
    ($($t:ty),*) => {
        $(
            impl<const BASE: u128> TryFrom<$t> for IntegerNumber<BASE> {
                type Error = MathError;
                fn try_from(value: $t) -> Result<Self, Self::Error> {
                    match value.cmp(&0) {
                        std::cmp::Ordering::Equal => Ok(Self::Zero),
                        std::cmp::Ordering::Greater => {
                            let abs = PositiveNaturalNumber::try_from(value as u128)?;
                            Ok(Self::Positive(abs))
                        }
                        std::cmp::Ordering::Less => {
                            let abs = PositiveNaturalNumber::try_from(value.unsigned_abs() as u128)?;
                            Ok(Self::Negative(abs))
                        }
                    }
                }
            }
        )*
    };
}
impl_try_from_signed_primitive!(i128, i64, i32, i16, i8, isize);

macro_rules! impl_try_from_unsigned_primitive {
    ($($t:ty),*) => {
        $(
            impl<const BASE: u128> TryFrom<$t> for IntegerNumber<BASE> {
                type Error = MathError;
                fn try_from(value: $t) -> Result<Self, Self::Error> {
                    match value {
                        0 => Ok(Self::Zero),
                        _ => {
                            let abs = PositiveNaturalNumber::try_from(value as u128)?;
                            Ok(Self::Positive(abs))
                        }
                    }
                }
            }
        )*
    };
}
impl_try_from_unsigned_primitive!(u128, u64, u32, u16, u8, usize);

macro_rules! impl_try_into_signed_primitive {
    ($($t:ty),*) => {
        $(
            impl<const BASE: u128> TryFrom<IntegerNumber<BASE>> for $t {
                type Error = MathError;
                fn try_from(num: IntegerNumber<BASE>) -> Result<Self, Self::Error> {
                    match num {
                        IntegerNumber::Zero => Ok(0),
                        IntegerNumber::Positive(abs) => {
                            let val_u128 = u128::try_from(abs)?;
                            <$t>::try_from(val_u128).map_err(|_| MathError::QuotientOverflow)
                        }
                        IntegerNumber::Negative(abs) => {
                            let val_u128 = u128::try_from(abs)?;
                            let val_signed = <$t>::try_from(val_u128).map_err(|_| MathError::QuotientOverflow)?;
                            val_signed.checked_neg().ok_or(MathError::QuotientOverflow)
                        }
                    }
                }
            }
        )*
    };
}
impl_try_into_signed_primitive!(i128, i64, i32, i16, i8, isize);

macro_rules! impl_try_into_unsigned_primitive {
    ($($t:ty),*) => {
        $(
            impl<const BASE: u128> TryFrom<IntegerNumber<BASE>> for $t {
                type Error = MathError;
                fn try_from(num: IntegerNumber<BASE>) -> Result<Self, Self::Error> {
                    match num {
                        IntegerNumber::Zero => Ok(0),
                        IntegerNumber::Positive(abs) => {
                            let val_u128 = u128::try_from(abs)?;
                            <$t>::try_from(val_u128).map_err(|_| MathError::QuotientOverflow)
                        }
                        IntegerNumber::Negative(_) => Err(MathError::ResultNotInDomain {
                            this_domain: "UnsignedPrimitives".to_string(),
                            result_domain: "NegativeIntegers".to_string(),
                        }),
                    }
                }
            }
        )*
    };
}
impl_try_into_unsigned_primitive!(u128, u64, u32, u16, u8, usize);

impl<const BASE: u128> crate::math::operations::NumberType<BASE> for IntegerNumber<BASE> {
    fn digit(&self, pos: i64) -> Result<crate::math::base_digit::Digit<BASE>, crate::math::math_error::MathError> {
        let zero_digit = crate::math::base_digit::Digit::new(0).unwrap();
        if pos >= 0 {
            match self {
                Self::Zero => Ok(zero_digit),
                Self::Positive(abs) | Self::Negative(abs) => {
                    let nat = NaturalNumber::from(abs.clone());
                    Ok(nat.digits()
                        .get(pos as usize)
                        .copied()
                        .unwrap_or(zero_digit))
                }
            }
        } else {
            Ok(zero_digit)
        }
    }
}

