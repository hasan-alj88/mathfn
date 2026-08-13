use super::NaturalNumber;
use crate::math::base_digit::Digit;
use crate::math::math_error::MathError;
use crate::math::natural_number::multiplication::nat_mul_karatsuba;

/// Divides a NaturalNumber by 2, returning the quotient and the remainder (0 or 1).
fn div_by_2<const BASE: u128>(num: &NaturalNumber<BASE>) -> (NaturalNumber<BASE>, u128) {
    let mut result_digits = vec![Digit::new(0).unwrap(); num.digits().len()];
    let mut carry = 0;

    (0..num.digits().len()).rev().for_each(|i| {
        let val = carry * BASE + num.digits()[i].value();
        result_digits[i] = Digit::new(val / 2).unwrap();
        carry = val % 2;
    });

    (NaturalNumber::new(result_digits), carry)
}

/// Performs arbitrary precision exponentiation by squaring.
pub fn nat_pow_binary<const BASE: u128>(
    base: &NaturalNumber<BASE>,
    exponent: &NaturalNumber<BASE>,
) -> Result<NaturalNumber<BASE>, MathError> {
    match (base.is_zero(), exponent.is_zero()) {
        (_, true) => return NaturalNumber::from_u128(1),
        (true, false) => return Ok(NaturalNumber::new(Vec::new())),
        _ => {}
    }

    let mut result = NaturalNumber::from_u128(1)?;
    let mut temp_base = base.clone();
    let mut temp_exp = exponent.clone();

    while !temp_exp.is_zero() {
        let (next_exp, rem) = div_by_2(&temp_exp);
        match rem {
            1 => {
                result = nat_mul_karatsuba(&result, &temp_base)?;
            }
            _ => {}
        }
        match next_exp.digits().len() {
            0 => {}
            _ => {
                temp_base = nat_mul_karatsuba(&temp_base, &temp_base)?;
            }
        }
        temp_exp = next_exp;
    }

    Ok(result)
}
