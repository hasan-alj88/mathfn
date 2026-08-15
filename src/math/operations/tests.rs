use crate::math::natural_number::NaturalNumber;
use crate::math::positive_natural::PositiveNaturalNumber;
use crate::math::integer_number::IntegerNumber;
use crate::math::operations::NumberType;
use crate::math::operations::number_type::{RealNumber, ComplexNumber, FinitePrecision, FiniteContinuedFractionNumber};

#[test]
fn test_real_and_complex_basics() {
    let nat = NaturalNumber::<256>::from_u128(123).unwrap();
    assert_eq!(nat.digit(0).unwrap().value(), 123);
    assert_eq!(nat.digit(1).unwrap().value(), 0);

    let fp = FinitePrecision::<256> {
        integer_part: NaturalNumber::<256>::from_u128(12).unwrap(),
        fractional_part: NaturalNumber::<256>::from_u128(34).unwrap(),
    };

    let real_finite = RealNumber::<256>::Approximate(fp.clone());

    assert_eq!(real_finite.digit(0).unwrap().value(), 12);
    assert!(real_finite.digit(-2).is_err()); // Out of bounds for fractional length

    let complex = ComplexNumber::new(real_finite, RealNumber::ExactFinite(fp));

    assert_eq!(complex.re.digit(0).unwrap().value(), 12);
}

#[test]
fn test_finite_precision_exact() {
    let fp = FinitePrecision::<256> {
        integer_part: NaturalNumber::from_u128(5).unwrap(),
        fractional_part: NaturalNumber::from_u128(20).unwrap(),
    };
    // Exact finite precision returns Ok(0) out of bounds
    assert_eq!(fp.digit(0).unwrap().value(), 5);
    assert_eq!(fp.digit(-1).unwrap().value(), 20);
    assert_eq!(fp.digit(-2).unwrap().value(), 0);
}

#[test]
fn test_continued_fraction_conversions() {
    // [1; 2] = 1 + 1/2 = 3/2 = 1.5
    let fcf = FiniteContinuedFractionNumber::<256> {
        integer_part: IntegerNumber::Positive(PositiveNaturalNumber::try_from(1u128).unwrap()),
        coefficients: vec![PositiveNaturalNumber::try_from(2u128).unwrap()],
    };

    let rat = fcf.to_rational().unwrap();
    assert_eq!(u128::try_from(rat.numerator().clone()).unwrap(), 3);
    assert_eq!(u128::try_from(rat.denominator().clone()).unwrap(), 2);
}

#[test]
fn test_continued_fraction_operators() {
    // [1; 2] = 3/2
    let fcf1 = FiniteContinuedFractionNumber::<256> {
        integer_part: IntegerNumber::Positive(PositiveNaturalNumber::try_from(1u128).unwrap()),
        coefficients: vec![PositiveNaturalNumber::try_from(2u128).unwrap()],
    };
    // [0; 3] = 1/3
    let fcf2 = FiniteContinuedFractionNumber::<256> {
        integer_part: IntegerNumber::Zero,
        coefficients: vec![PositiveNaturalNumber::try_from(3u128).unwrap()],
    };

    // 3/2 + 1/3 = 11/6 = [1; 1, 5]
    let sum = (fcf1.clone() + fcf2.clone()).unwrap();
    assert_eq!(i32::try_from(sum.integer_part.clone()).unwrap(), 1);
    assert_eq!(sum.coefficients.len(), 2);
    assert_eq!(u128::try_from(sum.coefficients[0].clone()).unwrap(), 1);
    assert_eq!(u128::try_from(sum.coefficients[1].clone()).unwrap(), 5);

    // 3/2 * 1/3 = 1/2 = [0; 2]
    let prod = (fcf1 * fcf2).unwrap();
    assert_eq!(i32::try_from(prod.integer_part.clone()).unwrap(), 0);
    assert_eq!(prod.coefficients.len(), 1);
    assert_eq!(u128::try_from(prod.coefficients[0].clone()).unwrap(), 2);
}

#[test]
fn test_cross_type_operators() {
    use std::convert::TryFrom;
    use crate::math::sign::Sign;
    use crate::math::rational_number::RationalNumber;

    let nat_5 = NaturalNumber::<256>::from_u128(5).unwrap();
    let pos_3 = PositiveNaturalNumber::<256>::try_from(3u128).unwrap();

    // 1. NaturalNumber + PositiveNaturalNumber -> PositiveNaturalNumber
    let sum_np = (nat_5.clone() + pos_3.clone()).unwrap();
    assert_eq!(u128::try_from(sum_np).unwrap(), 8);

    // 2. NaturalNumber * PositiveNaturalNumber -> NaturalNumber
    let prod_np = (nat_5.clone() * pos_3.clone()).unwrap();
    assert_eq!(prod_np.to_u128().unwrap(), 15);

    // 3. IntegerNumber + NaturalNumber -> IntegerNumber
    let int_neg10 = IntegerNumber::<256>::try_from(-10i32).unwrap();
    let sum_in = (int_neg10.clone() + nat_5.clone()).unwrap();
    assert_eq!(i32::try_from(sum_in).unwrap(), -5);

    // 4. RationalNumber * IntegerNumber -> RationalNumber
    let rat_two_thirds = RationalNumber::<256>::new(
        Sign::Positive,
        PositiveNaturalNumber::try_from(2u128).unwrap(),
        PositiveNaturalNumber::try_from(3u128).unwrap(),
    ).unwrap();
    let int_neg3 = IntegerNumber::<256>::try_from(-3i32).unwrap();
    let prod_qi = (rat_two_thirds * int_neg3).unwrap();
    assert_eq!(prod_qi.sign(), Sign::Negative);
    assert_eq!(u128::try_from(prod_qi.numerator().clone()).unwrap(), 2);
    assert_eq!(u128::try_from(prod_qi.denominator().clone()).unwrap(), 1);

    // 5. FiniteContinuedFractionNumber + PositiveNaturalNumber -> FiniteContinuedFractionNumber
    // fcf1 = [1; 2] = 3/2
    let fcf1 = FiniteContinuedFractionNumber::<256> {
        integer_part: IntegerNumber::Positive(PositiveNaturalNumber::try_from(1u128).unwrap()),
        coefficients: vec![PositiveNaturalNumber::try_from(2u128).unwrap()],
    };
    let pos_2 = PositiveNaturalNumber::<256>::try_from(2u128).unwrap();
    // 3/2 + 2 = 7/2 = [3; 2]
    let sum_fcf_p = (fcf1 + pos_2).unwrap();
    assert_eq!(i32::try_from(sum_fcf_p.integer_part.clone()).unwrap(), 3);
    assert_eq!(sum_fcf_p.coefficients.len(), 1);
    assert_eq!(u128::try_from(sum_fcf_p.coefficients[0].clone()).unwrap(), 2);
}


