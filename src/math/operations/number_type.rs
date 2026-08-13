use crate::math::base_digit::Digit;
use crate::math::math_error::MathError;
use crate::math::natural_number::NaturalNumber;
use crate::math::positive_natural::PositiveNaturalNumber;
use crate::math::integer_number::IntegerNumber;
use crate::math::sign::Sign;
use std::sync::Arc;

pub trait NumberType<const BASE: u128> {
    fn digit(&self, pos: i64) -> Result<Digit<BASE>, MathError>;
}

pub enum RealNumber<const BASE: u128 = 256> {
    FinitePrecision {
        integer_part: NaturalNumber<BASE>,
        fractional_part: NaturalNumber<BASE>,
    },
    Float {
        mantissa: PositiveNaturalNumber<BASE>,
        power: IntegerNumber<BASE>,
        sign: Sign,
    },
    DigitalFormula(Arc<dyn Fn(i64) -> Result<Digit<BASE>, MathError> + Send + Sync>),
    Repeated {
        integer_part: NaturalNumber<BASE>,
        fractional_part: NaturalNumber<BASE>,
        repeated: NaturalNumber<BASE>,
    },
}

impl<const BASE: u128> NumberType<BASE> for RealNumber<BASE> {
    fn digit(&self, pos: i64) -> Result<Digit<BASE>, MathError> {
        let zero_digit = Digit::new(0).unwrap();

        match self {
            RealNumber::FinitePrecision { integer_part, fractional_part } => {
                if pos >= 0 {
                    Ok(integer_part.digits()
                        .get(pos as usize)
                        .copied()
                        .unwrap_or(zero_digit))
                } else {
                    let idx = (-pos - 1) as usize;
                    if idx < fractional_part.digits().len() {
                        Ok(fractional_part.digits()[idx])
                    } else {
                        Err(MathError::UnknownDigit { position: pos })
                    }
                }
            }

            RealNumber::Float { mantissa, power, sign: _ } => {
                let p_val = match i64::try_from(power.clone()) {
                    Ok(val) => val,
                    Err(_) => return Ok(zero_digit),
                };
                let mantissa_pos = pos - p_val;
                if mantissa_pos >= 0 {
                    let nat_mantissa = NaturalNumber::from(mantissa.clone());
                    Ok(nat_mantissa.digits()
                        .get(mantissa_pos as usize)
                        .copied()
                        .unwrap_or(zero_digit))
                } else {
                    Ok(zero_digit)
                }
            }

            RealNumber::DigitalFormula(formula_fn) => {
                (formula_fn)(pos)
            }

            RealNumber::Repeated { integer_part, fractional_part, repeated } => {
                if pos >= 0 {
                    Ok(integer_part.digits()
                        .get(pos as usize)
                        .copied()
                        .unwrap_or(zero_digit))
                } else {
                    let k = -pos;
                    let f_len = fractional_part.digits().len() as i64;
                    if k <= f_len {
                        Ok(fractional_part.digits()[(k - 1) as usize])
                    } else {
                        let r_len = repeated.digits().len() as i64;
                        if r_len == 0 {
                            Ok(zero_digit)
                        } else {
                            let offset = k - 1 - f_len;
                            let r_idx = (offset % r_len) as usize;
                            Ok(repeated.digits()[r_idx])
                        }
                    }
                }
            }
        }
    }
}

pub struct ComplexNumber<
    const BASE: u128 = 256,
    R: NumberType<BASE> = RealNumber<BASE>,
    I: NumberType<BASE> = RealNumber<BASE>,
> {
    pub re: R,
    pub im: I,
}

impl<const BASE: u128, R, I> ComplexNumber<BASE, R, I>
where
    R: NumberType<BASE>,
    I: NumberType<BASE>,
{
    pub fn new(re: R, im: I) -> Self {
        Self { re, im }
    }
}
