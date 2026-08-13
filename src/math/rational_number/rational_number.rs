use crate::math::sign::Sign;
use crate::math::positive_natural::PositiveNaturalNumber;
use crate::math::natural_number::NaturalNumber;
use crate::math::natural_number::division::{nat_div_rem_schoolbook, nat_gcd};
use crate::math::integer_number::IntegerNumber;
use crate::math::math_error::MathError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RationalNumber<const BASE: u128 = 256> {
    sign: Sign,
    numerator: PositiveNaturalNumber<BASE>,
    denominator: PositiveNaturalNumber<BASE>,
}

impl<const BASE: u128> RationalNumber<BASE> {
    pub fn new(
        sign: Sign,
        numerator: PositiveNaturalNumber<BASE>,
        denominator: PositiveNaturalNumber<BASE>,
    ) -> Result<Self, MathError> {
        match sign {
            Sign::Zero => {
                let one = PositiveNaturalNumber::try_from(1u128)?;
                Ok(Self {
                    sign: Sign::Zero,
                    numerator: one.clone(),
                    denominator: one,
                })
            }
            _ => {
                let num_nat = NaturalNumber::from(numerator);
                let den_nat = NaturalNumber::from(denominator);
                let g = nat_gcd(num_nat.clone(), den_nat.clone())?;

                let (num_red, _) = nat_div_rem_schoolbook(&num_nat, &g)?;
                let (den_red, _) = nat_div_rem_schoolbook(&den_nat, &g)?;

                Ok(Self {
                    sign,
                    numerator: PositiveNaturalNumber::try_from(num_red)?,
                    denominator: PositiveNaturalNumber::try_from(den_red)?,
                })
            }
        }
    }

    pub fn sign(&self) -> Sign {
        self.sign
    }

    pub fn numerator(&self) -> &PositiveNaturalNumber<BASE> {
        &self.numerator
    }

    pub fn denominator(&self) -> &PositiveNaturalNumber<BASE> {
        &self.denominator
    }
}

impl<const BASE: u128> From<PositiveNaturalNumber<BASE>> for RationalNumber<BASE> {
    fn from(num: PositiveNaturalNumber<BASE>) -> Self {
        let one = PositiveNaturalNumber::try_from(1u128).unwrap();
        Self {
            sign: Sign::Positive,
            numerator: num,
            denominator: one,
        }
    }
}

impl<const BASE: u128> From<NaturalNumber<BASE>> for RationalNumber<BASE> {
    fn from(num: NaturalNumber<BASE>) -> Self {
        match num.is_zero() {
            true => {
                let one = PositiveNaturalNumber::try_from(1u128).unwrap();
                Self {
                    sign: Sign::Zero,
                    numerator: one.clone(),
                    denominator: one,
                }
            }
            false => {
                let pos = PositiveNaturalNumber::try_from(num).unwrap();
                Self::from(pos)
            }
        }
    }
}

impl<const BASE: u128> From<IntegerNumber<BASE>> for RationalNumber<BASE> {
    fn from(num: IntegerNumber<BASE>) -> Self {
        match num {
            IntegerNumber::Zero => {
                let one = PositiveNaturalNumber::try_from(1u128).unwrap();
                Self {
                    sign: Sign::Zero,
                    numerator: one.clone(),
                    denominator: one,
                }
            }
            IntegerNumber::Positive(abs) => Self::from(abs),
            IntegerNumber::Negative(abs) => {
                let one = PositiveNaturalNumber::try_from(1u128).unwrap();
                Self {
                    sign: Sign::Negative,
                    numerator: abs,
                    denominator: one,
                }
            }
        }
    }
}

impl<const BASE: u128> TryFrom<RationalNumber<BASE>> for IntegerNumber<BASE> {
    type Error = MathError;
    fn try_from(num: RationalNumber<BASE>) -> Result<Self, Self::Error> {
        match num.sign {
            Sign::Zero => Ok(Self::Zero),
            _ => {
                let den_val = u128::try_from(num.denominator.clone())?;
                match den_val {
                    1 => match num.sign {
                        Sign::Positive => Ok(Self::Positive(num.numerator)),
                        Sign::Negative => Ok(Self::Negative(num.numerator)),
                        Sign::Zero => unreachable!(),
                    },
                    _ => Err(MathError::ResultNotInDomain {
                        this_domain: "RationalNumbers".to_string(),
                        result_domain: "Integers".to_string(),
                    }),
                }
            }
        }
    }
}

macro_rules! impl_try_from_signed_primitive {
    ($($t:ty),*) => {
        $(
            impl<const BASE: u128> TryFrom<$t> for RationalNumber<BASE> {
                type Error = MathError;
                fn try_from(value: $t) -> Result<Self, Self::Error> {
                    let int_num = IntegerNumber::try_from(value)?;
                    Ok(Self::from(int_num))
                }
            }
        )*
    };
}
impl_try_from_signed_primitive!(i128, i64, i32, i16, i8, isize);

macro_rules! impl_try_from_unsigned_primitive {
    ($($t:ty),*) => {
        $(
            impl<const BASE: u128> TryFrom<$t> for RationalNumber<BASE> {
                type Error = MathError;
                fn try_from(value: $t) -> Result<Self, Self::Error> {
                    let int_num = IntegerNumber::try_from(value)?;
                    Ok(Self::from(int_num))
                }
            }
        )*
    };
}
impl_try_from_unsigned_primitive!(u128, u64, u32, u16, u8, usize);
