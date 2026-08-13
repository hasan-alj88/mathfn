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




