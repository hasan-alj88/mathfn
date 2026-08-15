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

#[test]
fn test_rational_operators_and_cf() {
    let half = RationalNumber::<256>::new(
        Sign::Positive,
        crate::math::positive_natural::PositiveNaturalNumber::<256>::try_from(1u128).unwrap(),
        crate::math::positive_natural::PositiveNaturalNumber::<256>::try_from(2u128).unwrap(),
    ).unwrap();
    let third = RationalNumber::<256>::new(
        Sign::Positive,
        crate::math::positive_natural::PositiveNaturalNumber::<256>::try_from(1u128).unwrap(),
        crate::math::positive_natural::PositiveNaturalNumber::<256>::try_from(3u128).unwrap(),
    ).unwrap();

    // 1/2 + 1/3 = 5/6
    let sum = (half.clone() + third.clone()).unwrap();
    assert_eq!(u128::try_from(sum.numerator().clone()).unwrap(), 5);
    assert_eq!(u128::try_from(sum.denominator().clone()).unwrap(), 6);
    assert_eq!(sum.sign(), Sign::Positive);

    // 1/2 * 1/3 = 1/6
    let prod = (half.clone() * third.clone()).unwrap();
    assert_eq!(u128::try_from(prod.numerator().clone()).unwrap(), 1);
    assert_eq!(u128::try_from(prod.denominator().clone()).unwrap(), 6);
    assert_eq!(prod.sign(), Sign::Positive);

    // Continued fraction of 45/16 -> [2; 1, 4, 3]
    let r_cf = RationalNumber::<256>::new(
        Sign::Positive,
        crate::math::positive_natural::PositiveNaturalNumber::<256>::try_from(45u128).unwrap(),
        crate::math::positive_natural::PositiveNaturalNumber::<256>::try_from(16u128).unwrap(),
    ).unwrap();
    let cf = r_cf.to_continued_fraction().unwrap();
    assert_eq!(i32::try_from(cf.integer_part.clone()).unwrap(), 2);
    assert_eq!(cf.coefficients.len(), 3);
    assert_eq!(u128::try_from(cf.coefficients[0].clone()).unwrap(), 1);
    assert_eq!(u128::try_from(cf.coefficients[1].clone()).unwrap(), 4);
    assert_eq!(u128::try_from(cf.coefficients[2].clone()).unwrap(), 3);

    // Continued fraction of -45/16 -> [-3; 5, 3]
    let r_cf_neg = RationalNumber::<256>::new(
        Sign::Negative,
        crate::math::positive_natural::PositiveNaturalNumber::<256>::try_from(45u128).unwrap(),
        crate::math::positive_natural::PositiveNaturalNumber::<256>::try_from(16u128).unwrap(),
    ).unwrap();
    let cf_neg = r_cf_neg.to_continued_fraction().unwrap();
    assert_eq!(i32::try_from(cf_neg.integer_part.clone()).unwrap(), -3);
    assert_eq!(cf_neg.coefficients.len(), 2);
    assert_eq!(u128::try_from(cf_neg.coefficients[0].clone()).unwrap(), 5);
    assert_eq!(u128::try_from(cf_neg.coefficients[1].clone()).unwrap(), 3);
}

