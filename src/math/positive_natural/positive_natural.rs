use crate::math::natural_number::NaturalNumber;
use crate::math::natural_number::addition::nat_add_schoolbook;
use crate::math::natural_number::multiplication::nat_sub_schoolbook;
use crate::math::math_error::MathError;

/// A positive natural number (Z^+ = {1, 2, 3, ...}).
/// Zero is not representable in this type.
///
/// ### Positional Representation
/// To store values starting from 1, the internal representation is offset by -1.
/// The value of `PositiveNaturalNumber(N)` is mathematically equal to `N + 1`.
/// - The value 1 is represented by an internal `NaturalNumber` of 0 (empty digits).
/// - The value 2 is represented by an internal `NaturalNumber` of 1.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PositiveNaturalNumber<const BASE: u128 = 256>(NaturalNumber<BASE>);

impl<const BASE: u128> PositiveNaturalNumber<BASE> {
    pub fn new_raw(internal_val: NaturalNumber<BASE>) -> Self {
        Self(internal_val)
    }

    pub fn offset_val(&self) -> &NaturalNumber<BASE> {
        &self.0
    }
}

impl<const BASE: u128> TryFrom<u128> for PositiveNaturalNumber<BASE> {
    type Error = MathError;
    fn try_from(value: u128) -> Result<Self, Self::Error> {
        match value {
            0 => Err(MathError::ResultNotInDomain {
                this_domain: "PositiveNaturalNumbers".to_string(),
                result_domain: "Zero".to_string(),
            }),
            _ => {
                let internal_num = NaturalNumber::from_u128(value - 1)?;
                Ok(Self(internal_num))
            }
        }
    }
}

impl<const BASE: u128> TryFrom<PositiveNaturalNumber<BASE>> for u128 {
    type Error = MathError;
    fn try_from(num: PositiveNaturalNumber<BASE>) -> Result<Self, Self::Error> {
        let internal_val = num.0.to_u128()?;
        internal_val.checked_add(1).ok_or(MathError::QuotientOverflow)
    }
}

impl<const BASE: u128> TryFrom<NaturalNumber<BASE>> for PositiveNaturalNumber<BASE> {
    type Error = MathError;
    fn try_from(num: NaturalNumber<BASE>) -> Result<Self, Self::Error> {
        match num.is_zero() {
            true => Err(MathError::ResultNotInDomain {
                this_domain: "PositiveNaturalNumbers".to_string(),
                result_domain: "Zero".to_string(),
            }),
            false => {
                let one = NaturalNumber::from_u128(1)?;
                let offset_num = nat_sub_schoolbook(&num, &one)?;
                Ok(Self(offset_num))
            }
        }
    }
}

impl<const BASE: u128> From<PositiveNaturalNumber<BASE>> for NaturalNumber<BASE> {
    fn from(num: PositiveNaturalNumber<BASE>) -> Self {
        let one = NaturalNumber::from_u128(1).unwrap();
        nat_add_schoolbook(&num.0, &one).unwrap()
    }
}

impl<const BASE: u128> std::ops::Add for PositiveNaturalNumber<BASE> {
    type Output = Result<Self, MathError>;

    fn add(self, rhs: Self) -> Self::Output {
        let sum_offset = nat_add_schoolbook(&self.0, &rhs.0)?;
        let one = NaturalNumber::from_u128(1)?;
        let res_offset = nat_add_schoolbook(&sum_offset, &one)?;
        Ok(Self(res_offset))
    }
}

impl<const BASE: u128> std::ops::Mul for PositiveNaturalNumber<BASE> {
    type Output = Result<Self, MathError>;

    fn mul(self, rhs: Self) -> Self::Output {
        let p1 = crate::math::natural_number::multiplication::nat_mul_karatsuba(&self.0, &rhs.0)?;
        let s1 = nat_add_schoolbook(&p1, &self.0)?;
        let res_offset = nat_add_schoolbook(&s1, &rhs.0)?;
        Ok(Self(res_offset))
    }
}

impl<const BASE: u128> TryFrom<crate::math::integer_number::IntegerNumber<BASE>> for PositiveNaturalNumber<BASE> {
    type Error = MathError;

    fn try_from(num: crate::math::integer_number::IntegerNumber<BASE>) -> Result<Self, Self::Error> {
        match num {
            crate::math::integer_number::IntegerNumber::Positive(pos) => Ok(pos),
            _ => Err(MathError::ResultNotInDomain {
                this_domain: "PositiveNaturalNumbers".to_string(),
                result_domain: "NonPositiveIntegers".to_string(),
            }),
        }
    }
}

macro_rules! impl_try_from_primitive_positive {
    ($($t:ty),*) => {
        $(
            impl<const BASE: u128> TryFrom<$t> for PositiveNaturalNumber<BASE> {
                type Error = MathError;
                fn try_from(value: $t) -> Result<Self, Self::Error> {
                    Self::try_from(value as u128)
                }
            }
        )*
    };
}
impl_try_from_primitive_positive!(u64, u32, u16, u8, usize);

macro_rules! impl_try_from_signed_primitive_positive {
    ($($t:ty),*) => {
        $(
            impl<const BASE: u128> TryFrom<$t> for PositiveNaturalNumber<BASE> {
                type Error = MathError;
                fn try_from(value: $t) -> Result<Self, Self::Error> {
                    match value.cmp(&0) {
                        std::cmp::Ordering::Less | std::cmp::Ordering::Equal => Err(MathError::ResultNotInDomain {
                            this_domain: "PositiveNaturalNumbers".to_string(),
                            result_domain: "NonPositive".to_string(),
                        }),
                        _ => Self::try_from(value as u128),
                    }
                }
            }
        )*
    };
}
impl_try_from_signed_primitive_positive!(i128, i64, i32, i16, i8, isize);



