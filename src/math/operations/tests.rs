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
