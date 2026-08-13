use super::NaturalNumber;
use crate::math::base_digit::{Digit, DigitOperations, DigitOutcome};
use crate::math::math_error::MathError;

pub fn nat_sub_schoolbook<const BASE: u128>(
    a: &NaturalNumber<BASE>,
    b: &NaturalNumber<BASE>,
) -> Result<NaturalNumber<BASE>, MathError> {
    let mut result_digits = Vec::new();
    let mut borrow = Digit::new(0).unwrap();

    for i in 0..a.digits().len() {
        let digit_a = a.digits()[i];
        let digit_b = b.digits().get(i).cloned().unwrap_or_else(|| Digit::new(0).unwrap());

        let outcome = digit_a.sub_digit(digit_b, borrow)?;
        match outcome {
            DigitOutcome::NoOverflow(diff) => {
                result_digits.push(diff);
                borrow = Digit::new(0).unwrap();
            }
            DigitOutcome::Overflow(diff, next_borrow) => {
                result_digits.push(diff);
                borrow = next_borrow;
            }
        }
    }

    if borrow.value() > 0 {
        return Err(MathError::ResultNotInDomain {
            this_domain: "NaturalNumbers".to_string(),
            result_domain: "Integers (negative)".to_string(),
        });
    }

    Ok(NaturalNumber::new(result_digits))
}
