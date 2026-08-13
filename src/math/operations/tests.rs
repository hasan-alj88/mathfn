use crate::math::natural_number::NaturalNumber;
use crate::math::operations::NumberType;
use crate::math::operations::number_type::{RealNumber, ComplexNumber};

#[test]
fn test_real_and_complex_basics() {
    let nat = NaturalNumber::<256>::from_u128(123).unwrap();
    assert_eq!(nat.digit(0).unwrap().value(), 123);
    assert_eq!(nat.digit(1).unwrap().value(), 0);

    let real_finite = RealNumber::<256>::FinitePrecision {
        integer_part: NaturalNumber::<256>::from_u128(12).unwrap(),
        fractional_part: NaturalNumber::<256>::from_u128(34).unwrap(),
    };

    assert_eq!(real_finite.digit(0).unwrap().value(), 12);
    assert!(real_finite.digit(-2).is_err()); // Out of bounds for fractional length of 1 digit (since 34 normalized to 1 digit in base 256)

    let complex = ComplexNumber::new(real_finite, RealNumber::FinitePrecision {
        integer_part: NaturalNumber::<256>::from_u128(0).unwrap(),
        fractional_part: NaturalNumber::<256>::from_u128(0).unwrap(),
    });

    assert_eq!(complex.re.digit(0).unwrap().value(), 12);
}
