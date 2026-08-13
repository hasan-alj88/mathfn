use super::*;

#[test]
fn test_base_digit_creation() {
    // Default base (BASE = 0, represents 2^128)
    let default_digit = Digit::<0>::new(u128::MAX).unwrap();
    assert_eq!(default_digit.value(), u128::MAX);
    assert!(matches!(default_digit, Digit::<0>::DoubleOctlet(_)));

    // Base 10 (Decimal)
    let dec_digit = Digit::<10>::new(9).unwrap();
    assert_eq!(dec_digit.value(), 9);
    assert!(matches!(dec_digit, Digit::Decimal(_)));
    assert!(Digit::<10>::new(10).is_err());

    // Base 12 (Dezonal)
    let dez_digit = Digit::<12>::new(11).unwrap();
    assert_eq!(dez_digit.value(), 11);
    assert!(matches!(dez_digit, Digit::Dezonal(_)));
    assert!(Digit::<12>::new(12).is_err());

    // Other custom base (e.g. Base 50)
    let custom_digit = Digit::<50>::new(49).unwrap();
    assert_eq!(custom_digit.value(), 49);
    assert!(matches!(custom_digit, Digit::Other(_)));
    assert!(Digit::<50>::new(50).is_err());

    // Base 1 (invalid)
    assert!(Digit::<1>::new(0).is_err());
}

#[test]
fn test_base10_arithmetic() {
    let a = Digit::<10>::new(7).unwrap();
    let b = Digit::<10>::new(5).unwrap();
    let zero = Digit::<10>::new(0).unwrap();
    let one = Digit::<10>::new(1).unwrap();

    // 7 + 5 + 0 = 12 -> 2 overflow 1
    let sum_res = a.add_digit(b, zero).unwrap();
    assert_eq!(sum_res, DigitOutcome::Overflow(Digit::<10>::new(2).unwrap(), Digit::<10>::new(1).unwrap()));

    // 7 - 5 = 2 (NoOverflow)
    let diff_res = a.sub_digit(b, zero).unwrap();
    assert_eq!(diff_res, DigitOutcome::NoOverflow(Digit::<10>::new(2).unwrap()));

    // 5 - 7 = 8 borrow 1 (Overflow)
    let diff2_res = b.sub_digit(a, zero).unwrap();
    assert_eq!(diff2_res, DigitOutcome::Overflow(Digit::<10>::new(8).unwrap(), Digit::<10>::new(1).unwrap()));

    // 7 * 5 + 1 = 36 -> 6 carry 3 (Overflow)
    let prod_res = a.mul_digit(b, one).unwrap();
    assert_eq!(prod_res, DigitOutcome::Overflow(Digit::<10>::new(6).unwrap(), Digit::<10>::new(3).unwrap()));
}

#[test]
fn test_base_2_128_arithmetic() {
    let max = Digit::<0>::new(u128::MAX).unwrap();
    let zero = Digit::<0>::new(0).unwrap();
    let one = Digit::<0>::new(1).unwrap();

    // u128::MAX + 1 = 0 overflow 1
    let sum_res = max.add_digit(one, zero).unwrap();
    assert_eq!(sum_res, DigitOutcome::Overflow(Digit::<0>::new(0).unwrap(), Digit::<0>::new(1).unwrap()));

    // u128::MAX * u128::MAX + 0 = (1, u128::MAX - 1)
    let prod_res = max.mul_digit(max, zero).unwrap();
    assert_eq!(prod_res, DigitOutcome::Overflow(Digit::<0>::new(1).unwrap(), Digit::<0>::new(u128::MAX - 1).unwrap()));
}

#[test]
fn test_base_digit_conversion() {
    // 12_10 to base 16: [12_16, 0_16]
    // Note: 12 is valid in base 20, let's construct a base 20 digit of value 12
    let digit_12_base20 = Digit::<20>::new(12).unwrap();
    let res_16 = digit_12_base20.convert_overflow::<16>().unwrap();
    assert_eq!(res_16.len(), 2);
    assert_eq!(res_16[0].value(), 12);
    assert_eq!(res_16[1].value(), 0);

    // 10_10 to base 2: [0_2, 1_2, 0_2, 1_2]
    // Let's construct a base 20 digit of value 10
    let digit_10_base20 = Digit::<20>::new(10).unwrap();
    let res_2 = digit_10_base20.convert_overflow::<2>().unwrap();
    assert_eq!(res_2.len(), 4);
    assert_eq!(res_2[0].value(), 0);
    assert_eq!(res_2[1].value(), 1);
    assert_eq!(res_2[2].value(), 0);
    assert_eq!(res_2[3].value(), 1);

    // into_digit tests
    let digit_9_base10 = Digit::<10>::new(9).unwrap();
    let res_into_16 = digit_9_base10.into_digit::<16>().unwrap();
    assert_eq!(res_into_16.value(), 9);

    let digit_18_base20 = Digit::<20>::new(18).unwrap();
    assert!(digit_18_base20.into_digit::<10>().is_err());
}

#[test]
fn test_digit_division_errors() {
    use crate::math::math_error::MathError;

    // Base 10 division: (high, low) / divisor
    let high_val = Digit::<10>::new(5).unwrap();
    let low_val = Digit::<10>::new(8).unwrap();
    let divisor_val = Digit::<10>::new(6).unwrap();

    // 58 / 6 = 9 remainder 4
    let res = DigitOperations::div_rem_digit(high_val, low_val, divisor_val).unwrap();
    assert_eq!(res, DigitOutcome::Overflow(Digit::<10>::new(9).unwrap(), Digit::<10>::new(4).unwrap()));

    // Division by zero in Base 10
    let zero = Digit::<10>::new(0).unwrap();
    let err_div_zero = DigitOperations::div_rem_digit(high_val, low_val, zero);
    assert!(matches!(err_div_zero, Err(MathError::DivisionByZero)));

    // Quotient overflow in Base 10: high >= divisor (6 >= 6)
    let high_overflow = Digit::<10>::new(6).unwrap();
    let err_overflow = DigitOperations::div_rem_digit(high_overflow, low_val, divisor_val);
    assert!(matches!(err_overflow, Err(MathError::QuotientOverflow)));

    // Base 2^128 (BASE = 0) division: (high, low) / divisor
    // Let's divide 2^128 by 2.
    // high = 1, low = 0, divisor = 2
    // Quotient should be 2^127 (1 << 127), remainder 0
    let high_0 = Digit::<0>::new(1).unwrap();
    let low_0 = Digit::<0>::new(0).unwrap();
    let divisor_0 = Digit::<0>::new(2).unwrap();
    let res_0 = DigitOperations::div_rem_digit(high_0, low_0, divisor_0).unwrap();
    assert_eq!(res_0, DigitOutcome::NoOverflow(Digit::<0>::new(1 << 127).unwrap()));

    // Division by zero in Base 2^128
    let zero_0 = Digit::<0>::new(0).unwrap();
    let err_zero_0 = DigitOperations::div_rem_digit(high_0, low_0, zero_0);
    assert!(matches!(err_zero_0, Err(MathError::DivisionByZero)));

    // Quotient overflow in Base 2^128: high >= divisor (2 >= 2)
    let err_overflow_0 = DigitOperations::div_rem_digit(divisor_0, low_0, divisor_0);
    assert!(matches!(err_overflow_0, Err(MathError::QuotientOverflow)));
}

#[test]
fn test_base_conversion_enum() {
    // Test the Exact variant
    let exact_result: BaseConversion<String, (), ()> = BaseConversion::Exact("1010".to_string());
    assert_eq!(exact_result, BaseConversion::Exact("1010".to_string()));

    // Test the InfiniteRepeating variant
    let repeating_result: BaseConversion<(), String, ()> = BaseConversion::InfiniteRepeating("3".to_string());
    assert_eq!(repeating_result, BaseConversion::InfiniteRepeating("3".to_string()));

    // Test the InfiniteNonRepeating variant
    let non_repeating_result: BaseConversion<(), (), String> = BaseConversion::InfiniteNonRepeating("14159...".to_string());
    assert_eq!(non_repeating_result, BaseConversion::InfiniteNonRepeating("14159...".to_string()));
}

#[test]
fn test_digit_base_mismatch_error() {
    use crate::math::math_error::MathError;

    // Both are Digit::<10>, but one has Quaternary variant and the other Decimal
    let a = Digit::<10>::Decimal(5);
    let b = Digit::<10>::Quaternary(2);
    let zero = Digit::<10>::Decimal(0);

    let res = a.add_digit(b, zero);
    assert!(matches!(res, Err(MathError::BaseMismatch)));
}
