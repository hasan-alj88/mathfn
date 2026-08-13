use crate::math::rational_number::RationalNumber;
use crate::math::sign::Sign;
use std::convert::TryFrom;

#[test]
fn test_rational_basics() {
    // Zero normalizes to 1/1
    let zero = RationalNumber::<256>::try_from(0i32).unwrap();
    assert_eq!(zero.sign(), Sign::Zero);
    assert_eq!(u128::try_from(zero.numerator().clone()).unwrap(), 1);
    assert_eq!(u128::try_from(zero.denominator().clone()).unwrap(), 1);

    // Reduction: 4/6 -> 2/3
    let four = crate::math::positive_natural::PositiveNaturalNumber::<256>::try_from(4u128).unwrap();
    let six = crate::math::positive_natural::PositiveNaturalNumber::<256>::try_from(6u128).unwrap();
    let r = RationalNumber::new(Sign::Positive, four, six).unwrap();
    assert_eq!(u128::try_from(r.numerator().clone()).unwrap(), 2);
    assert_eq!(u128::try_from(r.denominator().clone()).unwrap(), 3);

    // Primitive conversion
    let r_from_prim = RationalNumber::<256>::try_from(-5i64).unwrap();
    assert_eq!(r_from_prim.sign(), Sign::Negative);
    assert_eq!(u128::try_from(r_from_prim.numerator().clone()).unwrap(), 5);
    assert_eq!(u128::try_from(r_from_prim.denominator().clone()).unwrap(), 1);
}
