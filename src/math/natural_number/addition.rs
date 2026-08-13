use super::NaturalNumber;
use crate::math::base_digit::{Digit, DigitOperations, DigitOutcome};
use crate::math::math_error::MathError;

pub fn nat_add_schoolbook<const BASE: u128>(
    a: &NaturalNumber<BASE>,
    b: &NaturalNumber<BASE>,
) -> Result<NaturalNumber<BASE>, MathError> {
    let mut result_digits = Vec::new();
    let mut carry = Digit::new(0).map_err(|_| MathError::BaseMismatch)?;
    let max_len = std::cmp::max(a.digits().len(), b.digits().len());

    for i in 0..max_len {
        let digit_a = a.digits().get(i).cloned().unwrap_or_else(|| Digit::new(0).unwrap());
        let digit_b = b.digits().get(i).cloned().unwrap_or_else(|| Digit::new(0).unwrap());

        let outcome = digit_a.add_digit(digit_b, carry)?;
        match outcome {
            DigitOutcome::NoOverflow(sum_digit) => {
                result_digits.push(sum_digit);
                carry = Digit::new(0).unwrap();
            }
            DigitOutcome::Overflow(sum_digit, carry_digit) => {
                result_digits.push(sum_digit);
                carry = carry_digit;
            }
        }
    }

    if carry.value() > 0 {
        result_digits.push(carry);
    }

    Ok(NaturalNumber::new(result_digits))
}
