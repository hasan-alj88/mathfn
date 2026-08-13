# NaturalNumber Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement the `NaturalNumber` struct generic over a base `BASE` with custom arithmetic algorithms (`nat_add_schoolbook`, `nat_mul_schoolbook`, `nat_mul_karatsuba`, `nat_pow_binary`).

**Architecture:** A struct `NaturalNumber<const BASE: u128 = 256>` wrapping `Vec<Digit<BASE>>` in Least Significant Digit (LSD) first order. Digit normalization is enforced (no trailing zero digits).

**Tech Stack:** Rust (cargo, std, existing `mathfn` codebase).

## Global Constraints
- `BASE` parameter defaults to `256`.
- Digit values must be strictly less than `BASE`.
- Digits are stored in LSD-first order.
- Normalization must trim trailing zeroes.
- General subtraction and division are deferred to future tasks.

---

## Proposed Changes

### [mathfn]

#### [MODIFY] [mod.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/mod.rs)
Register `natural_number` module.

#### [NEW] [mod.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/natural_number/mod.rs)
Struct definition, constructors, normalization, and conversions.

#### [NEW] [addition.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/natural_number/addition.rs)
Schoolbook addition `nat_add_schoolbook`.

#### [NEW] [multiplication.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/natural_number/multiplication.rs)
Schoolbook subtraction helper `nat_sub_schoolbook`, schoolbook multiplication `nat_mul_schoolbook`, Karatsuba multiplication `nat_mul_karatsuba`, and shift/split helpers.

#### [NEW] [power.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/natural_number/power.rs)
`div_by_2` helper and binary power `nat_pow_binary`.

#### [NEW] [tests.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/natural_number/tests.rs)
Unit tests for all operations.

---

### Task 1: Module registration, Struct definition, constructors, and normalization

**Files:**
- Modify: `src/math/mod.rs`
- Create: `src/math/natural_number/mod.rs`
- Create: `src/math/natural_number/tests.rs`

**Interfaces:**
- Produces: `pub struct NaturalNumber<const BASE: u128 = 256>`, `NaturalNumber::new`, `NaturalNumber::from_u128`, `NaturalNumber::to_u128`, `NaturalNumber::is_zero`, `NaturalNumber::digits`.

- [ ] **Step 1: Register module in `src/math/mod.rs`**

Add `pub mod natural_number;` to `src/math/mod.rs`.

- [ ] **Step 2: Write failing tests in `src/math/natural_number/tests.rs`**

```rust
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
```

- [ ] **Step 3: Run tests to verify it fails**

Run: `cargo test math::natural_number::tests`
Expected: Compilation failure (module does not exist or struct not defined).

- [ ] **Step 4: Write minimal implementation in `src/math/natural_number/mod.rs`**

```rust
use crate::math::base_digit::Digit;
use crate::math::math_error::MathError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NaturalNumber<const BASE: u128 = 256> {
    digits: Vec<Digit<BASE>>,
}

impl<const BASE: u128> NaturalNumber<BASE> {
    pub fn new(digits: Vec<Digit<BASE>>) -> Self {
        let mut num = Self { digits };
        num.normalize();
        num
    }

    pub fn from_u128(mut value: u128) -> Result<Self, MathError> {
        let mut digits = Vec::new();
        while value > 0 {
            let digit_val = value % BASE;
            digits.push(Digit::new(digit_val).map_err(|_| MathError::BaseMismatch)?);
            value /= BASE;
        }
        Ok(Self::new(digits))
    }

    pub fn to_u128(&self) -> Result<u128, MathError> {
        let mut value: u128 = 0;
        let mut power: u128 = 1;
        for digit in &self.digits {
            let term = digit.value().checked_mul(power).ok_or(MathError::QuotientOverflow)?;
            value = value.checked_add(term).ok_or(MathError::QuotientOverflow)?;
            power = power.checked_mul(BASE).ok_or(MathError::QuotientOverflow)?;
        }
        Ok(value)
    }

    fn normalize(&mut self) {
        while let Some(last) = self.digits.last() {
            if last.value() == 0 {
                self.digits.pop();
            } else {
                break;
            }
        }
    }

    pub fn is_zero(&self) -> bool {
        self.digits.is_empty()
    }

    pub fn digits(&self) -> &[Digit<BASE>] {
        &self.digits
    }
}
```

- [ ] **Step 5: Register submodules in `src/math/natural_number/mod.rs`**

Add the test module and base submodules (to be created next):
```rust
#[cfg(test)]
mod tests;
```

- [ ] **Step 6: Run test to verify it passes**

Run: `cargo test math::natural_number::tests`
Expected: PASS

- [ ] **Step 7: Commit**

```bash
git add src/math/mod.rs src/math/natural_number/mod.rs src/math/natural_number/tests.rs
git commit -m "feat: add NaturalNumber struct base implementation and constructors"
```

---

### Task 2: Schoolbook Addition

**Files:**
- Create: `src/math/natural_number/addition.rs`
- Modify: `src/math/natural_number/mod.rs`
- Modify: `src/math/natural_number/tests.rs`

**Interfaces:**
- Consumes: `NaturalNumber` struct.
- Produces: `pub fn nat_add_schoolbook<const BASE: u128>(a: &NaturalNumber<BASE>, b: &NaturalNumber<BASE>) -> Result<NaturalNumber<BASE>, MathError>`.

- [ ] **Step 1: Write failing test in `src/math/natural_number/tests.rs`**

```rust
use crate::math::natural_number::addition::nat_add_schoolbook;

#[test]
fn test_addition() {
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
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test math::natural_number::tests::test_addition`
Expected: FAIL (compilation error, addition module not found).

- [ ] **Step 3: Implement `nat_add_schoolbook` in `src/math/natural_number/addition.rs`**

```rust
use super::NaturalNumber;
use crate::math::base_digit::{Digit, DigitOperations, DigitOutcome};
use crate::math::math_error::MathError;

pub fn nat_add_schoolbook<const BASE: u128>(
    a: &NaturalNumber<BASE>,
    b: &NaturalNumber<BASE>,
) -> Result<NaturalNumber<BASE>, MathError> {
    let mut result_digits = Vec::new();
    let mut carry = Digit::new(0).map_err(|_| MathError::BaseMismatch)?;
    let max_len = std::cmp::max(a.digits().len(), b.digits().len());

    for i in 0..max_len {
        let digit_a = a.digits().get(i).cloned().unwrap_or_else(|| Digit::new(0).unwrap());
        let digit_b = b.digits().get(i).cloned().unwrap_or_else(|| Digit::new(0).unwrap());

        let outcome = digit_a.add_digit(digit_b, carry)?;
        match outcome {
            DigitOutcome::NoOverflow(sum_digit) => {
                result_digits.push(sum_digit);
                carry = Digit::new(0).unwrap();
            }
            DigitOutcome::Overflow(sum_digit, carry_digit) => {
                result_digits.push(sum_digit);
                carry = carry_digit;
            }
        }
    }

    if carry.value() > 0 {
        result_digits.push(carry);
    }

    Ok(NaturalNumber::new(result_digits))
}
```

- [ ] **Step 4: Register `addition` in `src/math/natural_number/mod.rs`**

Add:
```rust
pub mod addition;
```

- [ ] **Step 5: Run test to verify it passes**

Run: `cargo test math::natural_number::tests::test_addition`
Expected: PASS

- [ ] **Step 6: Commit**

```bash
git add src/math/natural_number/addition.rs src/math/natural_number/mod.rs src/math/natural_number/tests.rs
git commit -m "feat: add schoolbook carrying addition"
```

---

### Task 3: Subtraction Helper (`nat_sub_schoolbook`)

**Files:**
- Create: `src/math/natural_number/multiplication.rs`
- Modify: `src/math/natural_number/mod.rs`
- Modify: `src/math/natural_number/tests.rs`

**Interfaces:**
- Consumes: `NaturalNumber`.
- Produces: `pub fn nat_sub_schoolbook<const BASE: u128>(a: &NaturalNumber<BASE>, b: &NaturalNumber<BASE>) -> Result<NaturalNumber<BASE>, MathError>`.

- [ ] **Step 1: Write failing test in `src/math/natural_number/tests.rs`**

```rust
use crate::math::natural_number::multiplication::nat_sub_schoolbook;

#[test]
fn test_subtraction() {
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
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test math::natural_number::tests::test_subtraction`
Expected: FAIL (compilation error, multiplication module not found).

- [ ] **Step 3: Implement `nat_sub_schoolbook` in `src/math/natural_number/multiplication.rs`**

```rust
use super::NaturalNumber;
use crate::math::base_digit::{Digit, DigitOperations, DigitOutcome};
use crate::math::math_error::MathError;

pub fn nat_sub_schoolbook<const BASE: u128>(
    a: &NaturalNumber<BASE>,
    b: &NaturalNumber<BASE>,
) -> Result<NaturalNumber<BASE>, MathError> {
    let mut result_digits = Vec::new();
    let mut borrow = Digit::new(0).unwrap();

    for i in 0..a.digits().len() {
        let digit_a = a.digits()[i];
        let digit_b = b.digits().get(i).cloned().unwrap_or_else(|| Digit::new(0).unwrap());

        let outcome = digit_a.sub_digit(digit_b, borrow)?;
        match outcome {
            DigitOutcome::NoOverflow(diff) => {
                result_digits.push(diff);
                borrow = Digit::new(0).unwrap();
            }
            DigitOutcome::Overflow(diff, next_borrow) => {
                result_digits.push(diff);
                borrow = next_borrow;
            }
        }
    }

    if borrow.value() > 0 {
        return Err(MathError::ResultNotInDomain {
            this_domain: "NaturalNumbers".to_string(),
            result_domain: "Integers (negative)".to_string(),
        });
    }

    Ok(NaturalNumber::new(result_digits))
}
```

- [ ] **Step 4: Register `multiplication` in `src/math/natural_number/mod.rs`**

Add:
```rust
pub mod multiplication;
```

- [ ] **Step 5: Run test to verify it passes**

Run: `cargo test math::natural_number::tests::test_subtraction`
Expected: PASS

- [ ] **Step 6: Commit**

```bash
git add src/math/natural_number/multiplication.rs src/math/natural_number/mod.rs src/math/natural_number/tests.rs
git commit -m "feat: add schoolbook subtraction helper"
```

---

### Task 4: Schoolbook and Karatsuba Multiplication

**Files:**
- Modify: `src/math/natural_number/multiplication.rs`
- Modify: `src/math/natural_number/tests.rs`

**Interfaces:**
- Consumes: `NaturalNumber`, `nat_add_schoolbook`, `nat_sub_schoolbook`.
- Produces: `pub fn nat_mul_schoolbook<const BASE: u128>(a: &NaturalNumber<BASE>, b: &NaturalNumber<BASE>) -> Result<NaturalNumber<BASE>, MathError>`, `pub fn nat_mul_karatsuba<const BASE: u128>(a: &NaturalNumber<BASE>, b: &NaturalNumber<BASE>) -> Result<NaturalNumber<BASE>, MathError>`.

- [ ] **Step 1: Write failing tests in `src/math/natural_number/tests.rs`**

```rust
use crate::math::natural_number::multiplication::{nat_mul_schoolbook, nat_mul_karatsuba};

#[test]
fn test_multiplication() {
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

    // Larger inputs (triggering Karatsuba split if threshold is met)
    let large_a = NaturalNumber::<256>::from_u128(1000000).unwrap();
    let large_b = NaturalNumber::<256>::from_u128(5000000).unwrap();
    let prod_large = nat_mul_karatsuba(&large_a, &large_b).unwrap();
    assert_eq!(prod_large.to_u128().unwrap(), 5000000000000);
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test math::natural_number::tests::test_multiplication`
Expected: FAIL (compilation error, functions not found).

- [ ] **Step 3: Implement multiplication and helpers in `src/math/natural_number/multiplication.rs`**

Append to `src/math/natural_number/multiplication.rs`:
```rust
use crate::math::natural_number::addition::nat_add_schoolbook;

/// Helper to add a digit at a specific index with carry propagation.
fn add_assign_digit<const BASE: u128>(
    digits: &mut Vec<Digit<BASE>>,
    index: usize,
    mut addend: Digit<BASE>,
) -> Result<(), MathError> {
    let mut i = index;
    while addend.value() > 0 {
        if i >= digits.len() {
            digits.resize(i + 1, Digit::new(0).unwrap());
        }
        let outcome = digits[i].add_digit(addend, Digit::new(0).unwrap())?;
        match outcome {
            DigitOutcome::NoOverflow(sum) => {
                digits[i] = sum;
                break;
            }
            DigitOutcome::Overflow(sum, carry) => {
                digits[i] = sum;
                addend = carry;
                i += 1;
            }
        }
    }
    Ok(())
}

/// Schoolbook multiplication.
pub fn nat_mul_schoolbook<const BASE: u128>(
    a: &NaturalNumber<BASE>,
    b: &NaturalNumber<BASE>,
) -> Result<NaturalNumber<BASE>, MathError> {
    if a.is_zero() || b.is_zero() {
        return Ok(NaturalNumber::new(Vec::new()));
    }

    let mut result_digits = Vec::new();

    for i in 0..a.digits().len() {
        let digit_a = a.digits()[i];
        let mut carry = Digit::new(0).unwrap();

        for j in 0..b.digits().len() {
            let digit_b = b.digits()[j];
            let outcome = digit_a.mul_digit(digit_b, carry)?;
            let (low, high) = match outcome {
                DigitOutcome::NoOverflow(l) => (l, Digit::new(0).unwrap()),
                DigitOutcome::Overflow(l, h) => (l, h),
            };

            add_assign_digit(&mut result_digits, i + j, low)?;
            carry = high;
        }

        if carry.value() > 0 {
            add_assign_digit(&mut result_digits, i + b.digits().len(), carry)?;
        }
    }

    Ok(NaturalNumber::new(result_digits))
}

/// Helper to shift a natural number left by `m` positions (multiplying by BASE^m).
fn shift_left<const BASE: u128>(num: &NaturalNumber<BASE>, m: usize) -> NaturalNumber<BASE> {
    if num.is_zero() || m == 0 {
        return num.clone();
    }
    let mut new_digits = vec![Digit::new(0).unwrap(); m];
    new_digits.extend_from_slice(num.digits());
    NaturalNumber::new(new_digits)
}

/// Helper to split a natural number at index `m`.
fn split_at<const BASE: u128>(
    num: &NaturalNumber<BASE>,
    m: usize,
) -> (NaturalNumber<BASE>, NaturalNumber<BASE>) {
    if m >= num.digits().len() {
        return (num.clone(), NaturalNumber::new(Vec::new()));
    }
    let lo_digits = num.digits()[..m].to_vec();
    let hi_digits = num.digits()[m..].to_vec();
    (NaturalNumber::new(lo_digits), NaturalNumber::new(hi_digits))
}

/// Karatsuba multiplication algorithm.
pub fn nat_mul_karatsuba<const BASE: u128>(
    a: &NaturalNumber<BASE>,
    b: &NaturalNumber<BASE>,
) -> Result<NaturalNumber<BASE>, MathError> {
    let n = std::cmp::max(a.digits().len(), b.digits().len());

    // Threshold under which we use schoolbook multiplication. Set to 4 for testing and production.
    if n < 4 {
        return nat_mul_schoolbook(a, b);
    }

    let m = n / 2;
    let (a_lo, a_hi) = split_at(a, m);
    let (b_lo, b_hi) = split_at(b, m);

    let p1 = nat_mul_karatsuba(&a_hi, &b_hi)?;
    let p2 = nat_mul_karatsuba(&a_lo, &b_lo)?;

    let a_sum = nat_add_schoolbook(&a_hi, &a_lo)?;
    let b_sum = nat_add_schoolbook(&b_hi, &b_lo)?;
    let p3 = nat_mul_karatsuba(&a_sum, &b_sum)?;

    // Middle term: p3 - p1 - p2
    let p1_plus_p2 = nat_add_schoolbook(&p1, &p2)?;
    let middle = nat_sub_schoolbook(&p3, &p1_plus_p2)?;

    // Result: p1 * BASE^(2m) + middle * BASE^m + p2
    let p1_shifted = shift_left(&p1, 2 * m);
    let middle_shifted = shift_left(&middle, m);

    let sum1 = nat_add_schoolbook(&p1_shifted, &middle_shifted)?;
    let result = nat_add_schoolbook(&sum1, &p2)?;

    Ok(result)
}
```

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test math::natural_number::tests::test_multiplication`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add src/math/natural_number/multiplication.rs src/math/natural_number/tests.rs
git commit -m "feat: add schoolbook and Karatsuba multiplication"
```

---

### Task 5: Exponentiation by Squaring (`nat_pow_binary`)

**Files:**
- Create: `src/math/natural_number/power.rs`
- Modify: `src/math/natural_number/mod.rs`
- Modify: `src/math/natural_number/tests.rs`

**Interfaces:**
- Consumes: `NaturalNumber`, `nat_mul_karatsuba`.
- Produces: `pub fn nat_pow_binary<const BASE: u128>(base: &NaturalNumber<BASE>, exponent: &NaturalNumber<BASE>) -> Result<NaturalNumber<BASE>, MathError>`.

- [ ] **Step 1: Write failing test in `src/math/natural_number/tests.rs`**

```rust
use crate::math::natural_number::power::nat_pow_binary;

#[test]
fn test_exponentiation() {
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
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test math::natural_number::tests::test_exponentiation`
Expected: FAIL (compilation error, power module not found).

- [ ] **Step 3: Implement binary power and `div_by_2` in `src/math/natural_number/power.rs`**

```rust
use super::NaturalNumber;
use crate::math::base_digit::Digit;
use crate::math::math_error::MathError;
use crate::math::natural_number::multiplication::nat_mul_karatsuba;

/// Divides a NaturalNumber by 2, returning the quotient and the remainder (0 or 1).
fn div_by_2<const BASE: u128>(num: &NaturalNumber<BASE>) -> (NaturalNumber<BASE>, u128) {
    let mut result_digits = vec![Digit::new(0).unwrap(); num.digits().len()];
    let mut carry = 0;

    for i in (0..num.digits().len()).rev() {
        let val = carry * BASE + num.digits()[i].value();
        result_digits[i] = Digit::new(val / 2).unwrap();
        carry = val % 2;
    }

    (NaturalNumber::new(result_digits), carry)
}

/// Performs arbitrary precision exponentiation by squaring.
pub fn nat_pow_binary<const BASE: u128>(
    base: &NaturalNumber<BASE>,
    exponent: &NaturalNumber<BASE>,
) -> Result<NaturalNumber<BASE>, MathError> {
    if exponent.is_zero() {
        return NaturalNumber::from_u128(1);
    }
    if base.is_zero() {
        return Ok(NaturalNumber::new(Vec::new()));
    }

    let mut result = NaturalNumber::from_u128(1)?;
    let mut temp_base = base.clone();
    let mut temp_exp = exponent.clone();

    while !temp_exp.is_zero() {
        let (next_exp, rem) = div_by_2(&temp_exp);
        if rem == 1 {
            result = nat_mul_karatsuba(&result, &temp_base)?;
        }
        if !next_exp.is_zero() {
            temp_base = nat_mul_karatsuba(&temp_base, &temp_base)?;
        }
        temp_exp = next_exp;
    }

    Ok(result)
}
```

- [ ] **Step 4: Register `power` in `src/math/natural_number/mod.rs`**

Add:
```rust
pub mod power;
```

- [ ] **Step 5: Run test to verify it passes**

Run: `cargo test math::natural_number::tests::test_exponentiation`
Expected: PASS

- [ ] **Step 6: Run all tests in the project**

Run: `cargo test`
Expected: PASS

- [ ] **Step 7: Commit**

```bash
git add src/math/natural_number/power.rs src/math/natural_number/mod.rs src/math/natural_number/tests.rs
git commit -m "feat: add binary exponentiation by squaring"
```

## Verification Plan

### Automated Tests
- Run `cargo test` to execute all tests under `src/math/natural_number/tests.rs` and other test files.
- Run `cargo check` to ensure clean compilation.
