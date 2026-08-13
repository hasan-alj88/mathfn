use crate::math::natural_number::NaturalNumber;
use crate::math::base_digit::Digit;

#[test]
fn test_natural_number_creation_and_conversion() {
    // Zero
    let zero = NaturalNumber::<256>::from_u128(0).unwrap();
    assert!(zero.is_zero());
    assert_eq!(zero.digits().len(), 0);
    assert_eq!(zero.to_u128().unwrap(), 0);

    // Small number
    let num = NaturalNumber::<256>::from_u128(500).unwrap();
    assert_eq!(num.digits().len(), 2); // 500 = 244 + 1 * 256
    assert_eq!(num.digits()[0].value(), 244);
    assert_eq!(num.digits()[1].value(), 1);
    assert_eq!(num.to_u128().unwrap(), 500);

    // Normalization test
    let raw_digits = vec![Digit::new(5).unwrap(), Digit::new(0).unwrap()];
    let norm_num = NaturalNumber::<256>::new(raw_digits);
    assert_eq!(norm_num.digits().len(), 1);
    assert_eq!(norm_num.digits()[0].value(), 5);
}

#[test]
fn test_addition() {
    use crate::math::natural_number::addition::nat_add_schoolbook;

    let a = NaturalNumber::<256>::from_u128(200).unwrap();
    let b = NaturalNumber::<256>::from_u128(100).unwrap();
    let sum = nat_add_schoolbook(&a, &b).unwrap();
    assert_eq!(sum.to_u128().unwrap(), 300);

    // Carry propagation test
    let large_a = NaturalNumber::<256>::from_u128(255).unwrap();
    let large_b = NaturalNumber::<256>::from_u128(1).unwrap();
    let sum2 = nat_add_schoolbook(&large_a, &large_b).unwrap();
    assert_eq!(sum2.digits().len(), 2);
    assert_eq!(sum2.digits()[0].value(), 0);
    assert_eq!(sum2.digits()[1].value(), 1);
    assert_eq!(sum2.to_u128().unwrap(), 256);
}

#[test]
fn test_subtraction() {
    use crate::math::natural_number::multiplication::nat_sub_schoolbook;

    let a = NaturalNumber::<256>::from_u128(300).unwrap();
    let b = NaturalNumber::<256>::from_u128(100).unwrap();
    let diff = nat_sub_schoolbook(&a, &b).unwrap();
    assert_eq!(diff.to_u128().unwrap(), 200);

    // Borrow test
    let large_a = NaturalNumber::<256>::from_u128(256).unwrap();
    let large_b = NaturalNumber::<256>::from_u128(1).unwrap();
    let diff2 = nat_sub_schoolbook(&large_a, &large_b).unwrap();
    assert_eq!(diff2.to_u128().unwrap(), 255);

    // Negative result (error) test
    let small_a = NaturalNumber::<256>::from_u128(100).unwrap();
    let small_b = NaturalNumber::<256>::from_u128(200).unwrap();
    assert!(nat_sub_schoolbook(&small_a, &small_b).is_err());
}

#[test]
fn test_multiplication() {
    use crate::math::natural_number::multiplication::{nat_mul_schoolbook, nat_mul_karatsuba};

    let a = NaturalNumber::<256>::from_u128(12).unwrap();
    let b = NaturalNumber::<256>::from_u128(13).unwrap();

    let prod1 = nat_mul_schoolbook(&a, &b).unwrap();
    assert_eq!(prod1.to_u128().unwrap(), 156);

    let prod2 = nat_mul_karatsuba(&a, &b).unwrap();
    assert_eq!(prod2.to_u128().unwrap(), 156);

    // Verify zero multiplication
    let zero = NaturalNumber::<256>::from_u128(0).unwrap();
    assert_eq!(nat_mul_schoolbook(&a, &zero).unwrap().to_u128().unwrap(), 0);
    assert_eq!(nat_mul_karatsuba(&a, &zero).unwrap().to_u128().unwrap(), 0);

    // Larger inputs (triggering Karatsuba split)
    let large_a = NaturalNumber::<256>::from_u128(1000000).unwrap();
    let large_b = NaturalNumber::<256>::from_u128(5000000).unwrap();
    let prod_large = nat_mul_karatsuba(&large_a, &large_b).unwrap();
    assert_eq!(prod_large.to_u128().unwrap(), 5000000000000);
}

#[test]
fn test_exponentiation() {
    use crate::math::natural_number::power::nat_pow_binary;

    let base = NaturalNumber::<256>::from_u128(2).unwrap();
    let exponent = NaturalNumber::<256>::from_u128(10).unwrap();
    let result = nat_pow_binary(&base, &exponent).unwrap();
    assert_eq!(result.to_u128().unwrap(), 1024);

    let base2 = NaturalNumber::<256>::from_u128(3).unwrap();
    let exponent2 = NaturalNumber::<256>::from_u128(5).unwrap();
    let result2 = nat_pow_binary(&base2, &exponent2).unwrap();
    assert_eq!(result2.to_u128().unwrap(), 243);

    // Pow to 0
    let exponent_zero = NaturalNumber::<256>::from_u128(0).unwrap();
    assert_eq!(nat_pow_binary(&base2, &exponent_zero).unwrap().to_u128().unwrap(), 1);
}

#[test]
fn test_conversions() {
    use std::convert::TryFrom;

    // Primitives -> NaturalNumber
    let num_unsigned = NaturalNumber::<256>::try_from(500u32).unwrap();
    assert_eq!(num_unsigned.to_u128().unwrap(), 500);

    let num_signed = NaturalNumber::<256>::try_from(123i64).unwrap();
    assert_eq!(num_signed.to_u128().unwrap(), 123);

    // Negative signed values fail
    let num_neg = NaturalNumber::<256>::try_from(-10i32);
    assert!(num_neg.is_err());

    // NaturalNumber -> Primitives
    let a = NaturalNumber::<256>::from_u128(1000).unwrap();
    let a_u16 = u16::try_from(a.clone()).unwrap();
    assert_eq!(a_u16, 1000);

    // Overflow conversion fails
    let a_u8 = u8::try_from(a.clone());
    assert!(a_u8.is_err());

    // Digit<BASE> <-> NaturalNumber
    let digit = Digit::<256>::new(5).unwrap();
    let num_from_digit = NaturalNumber::<256>::try_from(digit).unwrap();
    assert_eq!(num_from_digit.to_u128().unwrap(), 5);

    let digit_back = Digit::<256>::try_from(num_from_digit).unwrap();
    assert_eq!(digit_back.value(), 5);

    // Mismatched base for raw vector conversion
    let raw_vector_ok = NaturalNumber::<256>::try_from(vec![244u128, 1u128]).unwrap();
    assert_eq!(raw_vector_ok.to_u128().unwrap(), 500);

    let raw_vector_err = NaturalNumber::<256>::try_from(vec![300u128]);
    assert!(raw_vector_err.is_err());
}

#[test]
fn test_sign_enum_zero() {
    use crate::math::sign::Sign;
    let s = Sign::Zero;
    assert_eq!(s, Sign::Zero);
}

#[test]
fn test_positive_natural_basics() {
    use crate::math::positive_natural::PositiveNaturalNumber;
    use crate::math::natural_number::NaturalNumber;
    use std::convert::TryFrom;

    // Fails on zero
    assert!(PositiveNaturalNumber::<256>::try_from(0u128).is_err());

    // Succeeds on 1 (represented internally as 0)
    let one = PositiveNaturalNumber::<256>::try_from(1u128).unwrap();
    assert_eq!(u128::try_from(one.clone()).unwrap(), 1);
    assert!(one.offset_val().is_zero());

    // NaturalNumber conversion failable
    let nat_zero = NaturalNumber::<256>::from_u128(0).unwrap();
    assert!(PositiveNaturalNumber::try_from(nat_zero).is_err());

    let nat_five = NaturalNumber::<256>::from_u128(5).unwrap();
    let pos_five = PositiveNaturalNumber::try_from(nat_five).unwrap();
    assert_eq!(u128::try_from(pos_five).unwrap(), 5);
}

#[test]
fn test_natural_number_comparison() {
    let a = NaturalNumber::<256>::from_u128(500).unwrap();
    let b = NaturalNumber::<256>::from_u128(1000).unwrap();
    let c = NaturalNumber::<256>::from_u128(500).unwrap();

    assert!(a < b);
    assert!(b > a);
    assert_eq!(a, c);
    assert!(a <= c);
    assert!(a >= c);
}








