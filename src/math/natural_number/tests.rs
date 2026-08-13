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


