use crate::math::sign::Sign;
use crate::math::positive_natural::PositiveNaturalNumber;
use crate::math::natural_number::NaturalNumber;
use crate::math::natural_number::division::{nat_div_rem_schoolbook, nat_gcd};
use crate::math::integer_number::IntegerNumber;
use crate::math::math_error::{MathError, IntoMathError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RationalNumber<const BASE: u128 = 256> {
    sign: Sign,
    numerator: PositiveNaturalNumber<BASE>,
    denominator: PositiveNaturalNumber<BASE>,
}

impl<const BASE: u128> RationalNumber<BASE> {
    pub fn new<N, D>(
        sign: Sign,
        numerator: N,
        denominator: D,
    ) -> Result<Self, MathError>
    where
        N: TryInto<PositiveNaturalNumber<BASE>>,
        D: TryInto<PositiveNaturalNumber<BASE>>,
        N::Error: crate::math::math_error::IntoMathError,
        D::Error: crate::math::math_error::IntoMathError,
    {
        let num = numerator.try_into().map_err(|e| e.into_math_error())?;
        let den = denominator.try_into().map_err(|e| e.into_math_error())?;
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
                let num_nat = NaturalNumber::from(num);
                let den_nat = NaturalNumber::from(den);
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

impl<const BASE: u128> crate::math::operations::NumberType<BASE> for RationalNumber<BASE> {
    fn digit(&self, pos: i64) -> Result<crate::math::base_digit::Digit<BASE>, crate::math::math_error::MathError> {
        use crate::math::natural_number::NaturalNumber;
        use crate::math::natural_number::division::nat_div_rem_schoolbook;
        use crate::math::operations::NumberType;

        let num_nat = NaturalNumber::from(self.numerator().clone());
        let den_nat = NaturalNumber::from(self.denominator().clone());

        if pos >= 0 {
            // Integer part: floor(num / den)
            let (q, _) = nat_div_rem_schoolbook(&num_nat, &den_nat)?;
            q.digit(pos)
        } else {
            // Fractional part: floor(num * BASE^k / den) % BASE
            let k = -pos;
            let base_nat = NaturalNumber::from_u128(BASE)?;
            let k_nat = NaturalNumber::from_u128(k as u128)?;
            let factor = crate::math::natural_number::power::nat_pow_binary(&base_nat, &k_nat)?;
            let scaled_num = crate::math::natural_number::multiplication::nat_mul_schoolbook(&num_nat, &factor)?;
            let (q, _) = nat_div_rem_schoolbook(&scaled_num, &den_nat)?;
            
            // Get the units digit of q
            q.digit(0)
        }
    }
}

impl<const BASE: u128> RationalNumber<BASE> {
    pub fn to_continued_fraction(&self) -> Result<crate::math::operations::number_type::FiniteContinuedFractionNumber<BASE>, MathError> {
        if self.sign == Sign::Zero {
            return Ok(crate::math::operations::number_type::FiniteContinuedFractionNumber {
                integer_part: IntegerNumber::Zero,
                coefficients: Vec::new(),
            });
        }

        let p_mag = NaturalNumber::from(self.numerator.clone());
        let q_mag = NaturalNumber::from(self.denominator.clone());

        // Euclidean division of P by Q to get a_0 and remainder r
        let (q, r) = nat_div_rem_schoolbook(&p_mag, &q_mag)?;

        let (a_0, mut current_num) = match self.sign {
            Sign::Positive => {
                let a_0_int = match q.is_zero() {
                    true => IntegerNumber::Zero,
                    false => IntegerNumber::Positive(PositiveNaturalNumber::try_from(q)?),
                };
                (a_0_int, r)
            }
            Sign::Negative => {
                if r.is_zero() {
                    let a_0_int = IntegerNumber::Negative(PositiveNaturalNumber::try_from(q)?);
                    (a_0_int, r)
                } else {
                    let q_plus_1 = crate::math::natural_number::addition::nat_add_schoolbook(&q, &NaturalNumber::from_u128(1)?)?;
                    let a_0_int = IntegerNumber::Negative(PositiveNaturalNumber::try_from(q_plus_1)?);
                    let new_r = crate::math::natural_number::multiplication::nat_sub_schoolbook(&q_mag, &r)?;
                    (a_0_int, new_r)
                }
            }
            Sign::Zero => unreachable!(),
        };

        let mut coefficients = Vec::new();
        let mut current_den = q_mag;

        while !current_num.is_zero() {
            let (q_k, r_k) = nat_div_rem_schoolbook(&current_den, &current_num)?;
            coefficients.push(PositiveNaturalNumber::try_from(q_k)?);
            current_den = current_num;
            current_num = r_k;
        }

        Ok(crate::math::operations::number_type::FiniteContinuedFractionNumber {
            integer_part: a_0,
            coefficients,
        })
    }
}

impl<const BASE: u128> std::ops::Add for RationalNumber<BASE> {
    type Output = Result<Self, MathError>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.sign == Sign::Zero {
            return Ok(rhs);
        }
        if rhs.sign == Sign::Zero {
            return Ok(self);
        }

        let n1 = NaturalNumber::from(self.numerator);
        let d1 = NaturalNumber::from(self.denominator);
        let n2 = NaturalNumber::from(rhs.numerator);
        let d2 = NaturalNumber::from(rhs.denominator);

        // common denominator D = d1 * d2
        let common_den = crate::math::natural_number::multiplication::nat_mul_karatsuba(&d1, &d2)?;
        let common_den_pos = PositiveNaturalNumber::try_from(common_den)?;

        // scaled numerators N1 = n1 * d2, N2 = n2 * d1
        let term1 = crate::math::natural_number::multiplication::nat_mul_karatsuba(&n1, &d2)?;
        let term2 = crate::math::natural_number::multiplication::nat_mul_karatsuba(&n2, &d1)?;

        if self.sign == rhs.sign {
            let num_sum = crate::math::natural_number::addition::nat_add_schoolbook(&term1, &term2)?;
            let num_sum_pos = PositiveNaturalNumber::try_from(num_sum)?;
            RationalNumber::new(self.sign, num_sum_pos, common_den_pos)
        } else {
            match term1.cmp(&term2) {
                std::cmp::Ordering::Equal => {
                    let one = PositiveNaturalNumber::try_from(1u128)?;
                    Ok(RationalNumber {
                        sign: Sign::Zero,
                        numerator: one.clone(),
                        denominator: one,
                    })
                }
                std::cmp::Ordering::Greater => {
                    let num_diff = crate::math::natural_number::multiplication::nat_sub_schoolbook(&term1, &term2)?;
                    let num_diff_pos = PositiveNaturalNumber::try_from(num_diff)?;
                    RationalNumber::new(self.sign, num_diff_pos, common_den_pos)
                }
                std::cmp::Ordering::Less => {
                    let num_diff = crate::math::natural_number::multiplication::nat_sub_schoolbook(&term2, &term1)?;
                    let num_diff_pos = PositiveNaturalNumber::try_from(num_diff)?;
                    RationalNumber::new(rhs.sign, num_diff_pos, common_den_pos)
                }
            }
        }
    }
}

impl<const BASE: u128> std::ops::Mul for RationalNumber<BASE> {
    type Output = Result<Self, MathError>;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.sign == Sign::Zero || rhs.sign == Sign::Zero {
            let one = PositiveNaturalNumber::try_from(1u128)?;
            return Ok(RationalNumber {
                sign: Sign::Zero,
                numerator: one.clone(),
                denominator: one,
            });
        }

        let n1 = NaturalNumber::from(self.numerator);
        let d1 = NaturalNumber::from(self.denominator);
        let n2 = NaturalNumber::from(rhs.numerator);
        let d2 = NaturalNumber::from(rhs.denominator);

        let num_prod = crate::math::natural_number::multiplication::nat_mul_karatsuba(&n1, &n2)?;
        let den_prod = crate::math::natural_number::multiplication::nat_mul_karatsuba(&d1, &d2)?;

        let num_prod_pos = PositiveNaturalNumber::try_from(num_prod)?;
        let den_prod_pos = PositiveNaturalNumber::try_from(den_prod)?;

        let res_sign = match self.sign == rhs.sign {
            true => Sign::Positive,
            false => Sign::Negative,
        };

        RationalNumber::new(res_sign, num_prod_pos, den_prod_pos)
    }
}

impl<const BASE: u128> TryFrom<RationalNumber<BASE>> for NaturalNumber<BASE> {
    type Error = MathError;

    fn try_from(num: RationalNumber<BASE>) -> Result<Self, Self::Error> {
        let int_num = IntegerNumber::try_from(num)?;
        NaturalNumber::try_from(int_num)
    }
}

impl<const BASE: u128> TryFrom<RationalNumber<BASE>> for PositiveNaturalNumber<BASE> {
    type Error = MathError;

    fn try_from(num: RationalNumber<BASE>) -> Result<Self, Self::Error> {
        let int_num = IntegerNumber::try_from(num)?;
        PositiveNaturalNumber::try_from(int_num)
    }
}

impl<const BASE: u128> TryFrom<RationalNumber<BASE>> for crate::math::operations::number_type::FiniteContinuedFractionNumber<BASE> {
    type Error = MathError;

    fn try_from(num: RationalNumber<BASE>) -> Result<Self, Self::Error> {
        num.to_continued_fraction()
    }
}



