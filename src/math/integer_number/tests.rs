use crate::math::integer_number::IntegerNumber;
use crate::math::natural_number::NaturalNumber;
use crate::math::sign::Sign;
use std::convert::TryFrom;

#[test]
fn test_integer_conversions() {
    // 0
    let zero = IntegerNumber::<256>::try_from(0i32).unwrap();
    assert_eq!(zero.sign(), Sign::Zero);
    assert_eq!(i32::try_from(zero).unwrap(), 0);

    // Positive
    let pos = IntegerNumber::<256>::try_from(123i64).unwrap();
    assert_eq!(pos.sign(), Sign::Positive);
    assert_eq!(i64::try_from(pos.clone()).unwrap(), 123);
    assert_eq!(u64::try_from(pos).unwrap(), 123);

    // Negative
    let neg = IntegerNumber::<256>::try_from(-456i128).unwrap();
    assert_eq!(neg.sign(), Sign::Negative);
    assert_eq!(i128::try_from(neg.clone()).unwrap(), -456);
    assert!(u128::try_from(neg).is_err()); // Negative cannot convert to unsigned

    // NaturalNumber conversions
    let nat = NaturalNumber::<256>::from_u128(10).unwrap();
    let int_from_nat = IntegerNumber::from(nat);
    assert_eq!(i32::try_from(int_from_nat).unwrap(), 10);
}

#[test]
fn test_integer_operators() {
    let zero = IntegerNumber::<256>::Zero;
    let a = IntegerNumber::<256>::try_from(100i32).unwrap();
    let b = IntegerNumber::<256>::try_from(-40i32).unwrap();

    // 100 + (-40) = 60
    let sum = (a.clone() + b.clone()).unwrap();
    assert_eq!(i32::try_from(sum).unwrap(), 60);

    // 100 * (-40) = -4000
    let prod = (a.clone() * b.clone()).unwrap();
    assert_eq!(i32::try_from(prod).unwrap(), -4000);

    // a + 0 = a
    let sum_zero = (a.clone() + zero.clone()).unwrap();
    assert_eq!(i32::try_from(sum_zero).unwrap(), 100);

    // a * 0 = 0
    let prod_zero = (a.clone() * zero.clone()).unwrap();
    assert_eq!(i32::try_from(prod_zero).unwrap(), 0);
}

