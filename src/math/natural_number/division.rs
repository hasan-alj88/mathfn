use crate::math::natural_number::NaturalNumber;
use crate::math::natural_number::addition::nat_add_schoolbook;
use crate::math::natural_number::multiplication::nat_sub_schoolbook;
use crate::math::math_error::MathError;

fn div_by_2_local<const BASE: u128>(num: &NaturalNumber<BASE>) -> (NaturalNumber<BASE>, u128) {
    let mut result_digits = vec![crate::math::base_digit::Digit::new(0).unwrap(); num.digits().len()];
    let mut carry = 0;

    (0..num.digits().len()).rev().for_each(|i| {
        let val = carry * BASE + num.digits()[i].value();
        result_digits[i] = crate::math::base_digit::Digit::new(val / 2).unwrap();
        carry = val % 2;
    });

    (NaturalNumber::new(result_digits), carry)
}

pub fn nat_div_rem_schoolbook<const BASE: u128>(
    a: &NaturalNumber<BASE>,
    b: &NaturalNumber<BASE>,
) -> Result<(NaturalNumber<BASE>, NaturalNumber<BASE>), MathError> {
    match b.is_zero() {
        true => return Err(MathError::DivisionByZero),
        false => {}
    }
    match a.is_zero() {
        true => return Ok((NaturalNumber::new(Vec::new()), NaturalNumber::new(Vec::new()))),
        false => {}
    }
    match a.cmp(b) {
        std::cmp::Ordering::Less => return Ok((NaturalNumber::new(Vec::new()), a.clone())),
        _ => {}
    }

    let mut temp = a.clone();
    let mut bits = Vec::new();
    while !temp.is_zero() {
        let (next_temp, rem) = div_by_2_local(&temp);
        bits.push(match rem {
            1 => true,
            _ => false,
        });
        temp = next_temp;
    }
    bits.reverse();

    let mut q = NaturalNumber::from_u128(0)?;
    let mut r = NaturalNumber::from_u128(0)?;
    let one = NaturalNumber::from_u128(1)?;

    (0..bits.len()).try_for_each(|i| {
        let bit = bits[i];
        r = nat_add_schoolbook(&r, &r)?;
        match bit {
            true => r = nat_add_schoolbook(&r, &one)?,
            false => {}
        }
        match r.cmp(b) {
            std::cmp::Ordering::Greater | std::cmp::Ordering::Equal => {
                r = nat_sub_schoolbook(&r, b)?;
                q = nat_add_schoolbook(&q, &q)?;
                q = nat_add_schoolbook(&q, &one)?;
            }
            std::cmp::Ordering::Less => {
                q = nat_add_schoolbook(&q, &q)?;
            }
        }
        Ok::<(), MathError>(())
    })?;

    Ok((q, r))
}

pub fn nat_gcd<const BASE: u128>(
    mut a: NaturalNumber<BASE>,
    mut b: NaturalNumber<BASE>,
) -> Result<NaturalNumber<BASE>, MathError> {
    while !b.is_zero() {
        let (_, rem) = nat_div_rem_schoolbook(&a, &b)?;
        a = b;
        b = rem;
    }
    Ok(a)
}
