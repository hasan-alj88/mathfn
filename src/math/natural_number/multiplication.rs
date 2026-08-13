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

use crate::math::natural_number::addition::nat_add_schoolbook;

/// Helper to add a digit at a specific index with carry propagation.
fn add_assign_digit<const BASE: u128>(
    digits: &mut Vec<Digit<BASE>>,
    index: usize,
    mut addend: Digit<BASE>,
) -> Result<(), MathError> {
    let mut i = index;
    while addend.value() > 0 {
        if i >= digits.len() {
            digits.resize(i + 1, Digit::new(0).unwrap());
        }
        let outcome = digits[i].add_digit(addend, Digit::new(0).unwrap())?;
        match outcome {
            DigitOutcome::NoOverflow(sum) => {
                digits[i] = sum;
                break;
            }
            DigitOutcome::Overflow(sum, carry) => {
                digits[i] = sum;
                addend = carry;
                i += 1;
            }
        }
    }
    Ok(())
}

/// Schoolbook multiplication.
pub fn nat_mul_schoolbook<const BASE: u128>(
    a: &NaturalNumber<BASE>,
    b: &NaturalNumber<BASE>,
) -> Result<NaturalNumber<BASE>, MathError> {
    if a.is_zero() || b.is_zero() {
        return Ok(NaturalNumber::new(Vec::new()));
    }

    let mut result_digits = Vec::new();

    for i in 0..a.digits().len() {
        let digit_a = a.digits()[i];
        let mut carry = Digit::new(0).unwrap();

        for j in 0..b.digits().len() {
            let digit_b = b.digits()[j];
            let outcome = digit_a.mul_digit(digit_b, carry)?;
            let (low, high) = match outcome {
                DigitOutcome::NoOverflow(l) => (l, Digit::new(0).unwrap()),
                DigitOutcome::Overflow(l, h) => (l, h),
            };

            add_assign_digit(&mut result_digits, i + j, low)?;
            carry = high;
        }

        if carry.value() > 0 {
            add_assign_digit(&mut result_digits, i + b.digits().len(), carry)?;
        }
    }

    Ok(NaturalNumber::new(result_digits))
}

/// Helper to shift a natural number left by `m` positions (multiplying by BASE^m).
fn shift_left<const BASE: u128>(num: &NaturalNumber<BASE>, m: usize) -> NaturalNumber<BASE> {
    if num.is_zero() || m == 0 {
        return num.clone();
    }
    let mut new_digits = vec![Digit::new(0).unwrap(); m];
    new_digits.extend_from_slice(num.digits());
    NaturalNumber::new(new_digits)
}

/// Helper to split a natural number at index `m`.
fn split_at<const BASE: u128>(
    num: &NaturalNumber<BASE>,
    m: usize,
) -> (NaturalNumber<BASE>, NaturalNumber<BASE>) {
    if m >= num.digits().len() {
        return (num.clone(), NaturalNumber::new(Vec::new()));
    }
    let lo_digits = num.digits()[..m].to_vec();
    let hi_digits = num.digits()[m..].to_vec();
    (NaturalNumber::new(lo_digits), NaturalNumber::new(hi_digits))
}

/// Karatsuba multiplication algorithm.
pub fn nat_mul_karatsuba<const BASE: u128>(
    a: &NaturalNumber<BASE>,
    b: &NaturalNumber<BASE>,
) -> Result<NaturalNumber<BASE>, MathError> {
    let n = std::cmp::max(a.digits().len(), b.digits().len());

    // Threshold under which we use schoolbook multiplication. Set to 4 for testing and production.
    if n < 4 {
        return nat_mul_schoolbook(a, b);
    }

    let m = n / 2;
    let (a_lo, a_hi) = split_at(a, m);
    let (b_lo, b_hi) = split_at(b, m);

    let p1 = nat_mul_karatsuba(&a_hi, &b_hi)?;
    let p2 = nat_mul_karatsuba(&a_lo, &b_lo)?;

    let a_sum = nat_add_schoolbook(&a_hi, &a_lo)?;
    let b_sum = nat_add_schoolbook(&b_hi, &b_lo)?;
    let p3 = nat_mul_karatsuba(&a_sum, &b_sum)?;

    // Middle term: p3 - p1 - p2
    let p1_plus_p2 = nat_add_schoolbook(&p1, &p2)?;
    let middle = nat_sub_schoolbook(&p3, &p1_plus_p2)?;

    // Result: p1 * BASE^(2m) + middle * BASE^m + p2
    let p1_shifted = shift_left(&p1, 2 * m);
    let middle_shifted = shift_left(&middle, m);

    let sum1 = nat_add_schoolbook(&p1_shifted, &middle_shifted)?;
    let result = nat_add_schoolbook(&sum1, &p2)?;

    Ok(result)
}

