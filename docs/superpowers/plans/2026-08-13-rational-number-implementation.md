# RationalNumber Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement arbitrary-precision integer comparison, Euclidean division, GCD, and the `RationalNumber` type.

**Architecture:** 
- `Ord`/`PartialOrd` on `NaturalNumber` using reverse-lexicographical digit comparison.
- `nat_div_rem_schoolbook` using binary Restoring Division.
- `nat_gcd` using the Euclidean Algorithm.
- `RationalNumber` struct normalized to lowest terms via GCD.

---

## Proposed Changes

### [mathfn]

#### [MODIFY] [natural_number.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/natural_number/natural_number.rs)
Implement `Ord` and `PartialOrd` for `NaturalNumber`.

#### [MODIFY] [mod.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/natural_number/mod.rs)
Register `division` submodule.

#### [NEW] [division.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/natural_number/division.rs)
Implement division, remainder, and GCD functions.

#### [MODIFY] [mod.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/mod.rs)
Register `rational_number` module.

#### [NEW] [mod.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/rational_number/mod.rs)
`rational_number` module declaration.

#### [NEW] [rational_number.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/rational_number/rational_number.rs)
`RationalNumber` struct definition and conversions.

#### [NEW] [tests.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/rational_number/tests.rs)
Unit tests for division, GCD, and `RationalNumber`.

---

### Task 1: Comparison on NaturalNumber

**Files:**
- Modify: `src/math/natural_number/natural_number.rs`
- Modify: `src/math/natural_number/tests.rs`

- [ ] **Step 1: Write failing test in `src/math/natural_number/tests.rs`**

```rust
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
```

- [ ] **Step 2: Run test to verify it fails**

Expected: FAIL (compilation error, operators `<` and `>` not defined for `NaturalNumber`).

- [ ] **Step 3: Implement `Ord` and `PartialOrd` in `src/math/natural_number/natural_number.rs`**

```rust
impl<const BASE: u128> Ord for NaturalNumber<BASE> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.digits.len().cmp(&other.digits.len()) {
            std::cmp::Ordering::Equal => {
                self.digits.iter().zip(other.digits.iter()).rev()
                    .map(|(a, b)| a.value().cmp(&b.value()))
                    .find(|&ord| match ord {
                        std::cmp::Ordering::Equal => false,
                        _ => true,
                    })
                    .unwrap_or(std::cmp::Ordering::Equal)
            }
            non_eq => non_eq,
        }
    }
}

impl<const BASE: u128> PartialOrd for NaturalNumber<BASE> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
```

- [ ] **Step 4: Run test to verify it passes**

Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add src/math/natural_number/natural_number.rs src/math/natural_number/tests.rs
git commit -m "feat: implement Ord and PartialOrd for NaturalNumber"
```

---

### Task 2: Division and GCD on NaturalNumber

**Files:**
- Modify: `src/math/natural_number/mod.rs`
- Create: `src/math/natural_number/division.rs`
- Modify: `src/math/natural_number/tests.rs`

- [ ] **Step 1: Register module in `src/math/natural_number/mod.rs`**

Add `pub mod division;` to `src/math/natural_number/mod.rs`.

- [ ] **Step 2: Write failing test in `src/math/natural_number/tests.rs`**

```rust
#[test]
fn test_natural_division_and_gcd() {
    use crate::math::natural_number::division::{nat_div_rem_schoolbook, nat_gcd};

    // Division by zero fails
    let zero = NaturalNumber::<256>::from_u128(0).unwrap();
    let ten = NaturalNumber::<256>::from_u128(10).unwrap();
    assert!(nat_div_rem_schoolbook(&ten, &zero).is_err());

    // 10 / 3 = 3 rem 1
    let three = NaturalNumber::<256>::from_u128(3).unwrap();
    let (q, r) = nat_div_rem_schoolbook(&ten, &three).unwrap();
    assert_eq!(q.to_u128().unwrap(), 3);
    assert_eq!(r.to_u128().unwrap(), 1);

    // GCD(12, 18) = 6
    let twelve = NaturalNumber::<256>::from_u128(12).unwrap();
    let eighteen = NaturalNumber::<256>::from_u128(18).unwrap();
    let g = nat_gcd(twelve, eighteen).unwrap();
    assert_eq!(g.to_u128().unwrap(), 6);
}
```

- [ ] **Step 3: Run test to verify it fails**

Expected: FAIL (compilation error, division module not found).

- [ ] **Step 4: Implement division/GCD in `src/math/natural_number/division.rs`**

```rust
use crate::math::natural_number::NaturalNumber;
use crate::math::natural_number::addition::nat_add_schoolbook;
use crate::math::natural_number::multiplication::nat_sub_schoolbook;
use crate::math::math_error::MathError;

fn div_by_2_local<const BASE: u128>(num: &NaturalNumber<BASE>) -> (NaturalNumber<BASE>, u128) {
    let mut result_digits = vec![crate::math::base_digit::Digit::new(0).unwrap(); num.digits().len()];
    let mut carry = 0;

    (0..num.digits().len()).rev().for_each(|i| {
        let val = carry * BASE + num.digits()[i].value();
        result_digits[i] = crate::math::base_digit::Digit::new(val / 2).unwrap();
        carry = val % 2;
    });

    (NaturalNumber::new(result_digits), carry)
}

pub fn nat_div_rem_schoolbook<const BASE: u128>(
    a: &NaturalNumber<BASE>,
    b: &NaturalNumber<BASE>,
) -> Result<(NaturalNumber<BASE>, NaturalNumber<BASE>), MathError> {
    match b.is_zero() {
        true => return Err(MathError::DivisionByZero),
        false => {}
    }
    match a.is_zero() {
        true => return Ok((NaturalNumber::new(Vec::new()), NaturalNumber::new(Vec::new()))),
        false => {}
    }
    match a.cmp(b) {
        std::cmp::Ordering::Less => return Ok((NaturalNumber::new(Vec::new()), a.clone())),
        _ => {}
    }

    let mut temp = a.clone();
    let mut bits = Vec::new();
    while !temp.is_zero() {
        let (next_temp, rem) = div_by_2_local(&temp);
        bits.push(match rem {
            1 => true,
            _ => false,
        });
        temp = next_temp;
    }
    bits.reverse();

    let mut q = NaturalNumber::from_u128(0)?;
    let mut r = NaturalNumber::from_u128(0)?;
    let one = NaturalNumber::from_u128(1)?;

    (0..bits.len()).try_for_each(|i| {
        let bit = bits[i];
        r = nat_add_schoolbook(&r, &r)?;
        match bit {
            true => r = nat_add_schoolbook(&r, &one)?,
            false => {}
        }
        match r.cmp(b) {
            std::cmp::Ordering::Greater | std::cmp::Ordering::Equal => {
                r = nat_sub_schoolbook(&r, b)?;
                q = nat_add_schoolbook(&q, &q)?;
                q = nat_add_schoolbook(&q, &one)?;
            }
            std::cmp::Ordering::Less => {
                q = nat_add_schoolbook(&q, &q)?;
            }
        }
        Ok::<(), MathError>(())
    })?;

    Ok((q, r))
}

pub fn nat_gcd<const BASE: u128>(
    mut a: NaturalNumber<BASE>,
    mut b: NaturalNumber<BASE>,
) -> Result<NaturalNumber<BASE>, MathError> {
    while !b.is_zero() {
        let (_, rem) = nat_div_rem_schoolbook(&a, &b)?;
        a = b;
        b = rem;
    }
    Ok(a)
}
```

- [ ] **Step 5: Run test to verify it passes**

Expected: PASS.

- [ ] **Step 6: Commit**

```bash
git add src/math/natural_number/mod.rs src/math/natural_number/division.rs src/math/natural_number/tests.rs
git commit -m "feat: implement division and gcd on NaturalNumber"
```

---

### Task 3: RationalNumber Implementation & Tests

**Files:**
- Modify: `src/math/mod.rs`
- Create: `src/math/rational_number/mod.rs`
- Create: `src/math/rational_number/rational_number.rs`
- Create: `src/math/rational_number/tests.rs`

- [ ] **Step 1: Register module in `src/math/mod.rs`**

Add `pub mod rational_number;` to `src/math/mod.rs`.

- [ ] **Step 2: Create `src/math/rational_number/mod.rs`**

```rust
#[cfg(test)]
mod tests;

pub mod rational_number;
pub use rational_number::RationalNumber;
```

- [ ] **Step 3: Create tests file `src/math/rational_number/tests.rs` with failing tests**

```rust
use crate::math::rational_number::RationalNumber;
use crate::math::integer_number::IntegerNumber;
use crate::math::natural_number::NaturalNumber;
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
    let four = crate::math::positive_natural::PositiveNaturalNumber::try_from(4u128).unwrap();
    let six = crate::math::positive_natural::PositiveNaturalNumber::try_from(6u128).unwrap();
    let r = RationalNumber::new(Sign::Positive, four, six).unwrap();
    assert_eq!(u128::try_from(r.numerator().clone()).unwrap(), 2);
    assert_eq!(u128::try_from(r.denominator().clone()).unwrap(), 3);

    // Primitive conversion
    let r_from_prim = RationalNumber::<256>::try_from(-5i64).unwrap();
    assert_eq!(r_from_prim.sign(), Sign::Negative);
    assert_eq!(u128::try_from(r_from_prim.numerator().clone()).unwrap(), 5);
    assert_eq!(u128::try_from(r_from_prim.denominator().clone()).unwrap(), 1);
}
```

- [ ] **Step 4: Run test to verify it fails**

Expected: FAIL (compilation error, `RationalNumber` not defined).

- [ ] **Step 5: Implement `RationalNumber` in `src/math/rational_number/rational_number.rs`**

```rust
use crate::math::sign::Sign;
use crate::math::positive_natural::PositiveNaturalNumber;
use crate::math::natural_number::NaturalNumber;
use crate::math::natural_number::division::{nat_div_rem_schoolbook, nat_gcd};
use crate::math::integer_number::IntegerNumber;
use crate::math::math_error::MathError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RationalNumber<const BASE: u128 = 256> {
    sign: Sign,
    numerator: PositiveNaturalNumber<BASE>,
    denominator: PositiveNaturalNumber<BASE>,
}

impl<const BASE: u128> RationalNumber<BASE> {
    pub fn new(
        sign: Sign,
        numerator: PositiveNaturalNumber<BASE>,
        denominator: PositiveNaturalNumber<BASE>,
    ) -> Result<Self, MathError> {
        match sign {
            Sign::Zero => {
                let one = PositiveNaturalNumber::try_from(1u128)?;
                Ok(Self {
                    sign: Sign::Zero,
                    numerator: one.clone(),
                    denominator: one,
                })
            }
            _ => {
                let num_nat = NaturalNumber::from(numerator);
                let den_nat = NaturalNumber::from(denominator);
                let g = nat_gcd(num_nat.clone(), den_nat.clone())?;

                let (num_red, _) = nat_div_rem_schoolbook(&num_nat, &g)?;
                let (den_red, _) = nat_div_rem_schoolbook(&den_nat, &g)?;

                Ok(Self {
                    sign,
                    numerator: PositiveNaturalNumber::try_from(num_red)?,
                    denominator: PositiveNaturalNumber::try_from(den_red)?,
                })
            }
        }
    }

    pub fn sign(&self) -> Sign {
        self.sign
    }

    pub fn numerator(&self) -> &PositiveNaturalNumber<BASE> {
        &self.numerator
    }

    pub fn denominator(&self) -> &PositiveNaturalNumber<BASE> {
        &self.denominator
    }
}

impl<const BASE: u128> From<PositiveNaturalNumber<BASE>> for RationalNumber<BASE> {
    fn from(num: PositiveNaturalNumber<BASE>) -> Self {
        let one = PositiveNaturalNumber::try_from(1u128).unwrap();
        Self {
            sign: Sign::Positive,
            numerator: num,
            denominator: one,
        }
    }
}

impl<const BASE: u128> From<NaturalNumber<BASE>> for RationalNumber<BASE> {
    fn from(num: NaturalNumber<BASE>) -> Self {
        match num.is_zero() {
            true => {
                let one = PositiveNaturalNumber::try_from(1u128).unwrap();
                Self {
                    sign: Sign::Zero,
                    numerator: one.clone(),
                    denominator: one,
                }
            }
            false => {
                let pos = PositiveNaturalNumber::try_from(num).unwrap();
                Self::from(pos)
            }
        }
    }
}

impl<const BASE: u128> From<IntegerNumber<BASE>> for RationalNumber<BASE> {
    fn from(num: IntegerNumber<BASE>) -> Self {
        match num {
            IntegerNumber::Zero => {
                let one = PositiveNaturalNumber::try_from(1u128).unwrap();
                Self {
                    sign: Sign::Zero,
                    numerator: one.clone(),
                    denominator: one,
                }
            }
            IntegerNumber::Positive(abs) => Self::from(abs),
            IntegerNumber::Negative(abs) => {
                let one = PositiveNaturalNumber::try_from(1u128).unwrap();
                Self {
                    sign: Sign::Negative,
                    numerator: abs,
                    denominator: one,
                }
            }
        }
    }
}

impl<const BASE: u128> TryFrom<RationalNumber<BASE>> for IntegerNumber<BASE> {
    type Error = MathError;
    fn try_from(num: RationalNumber<BASE>) -> Result<Self, Self::Error> {
        match num.sign {
            Sign::Zero => Ok(Self::Zero),
            _ => {
                let den_val = u128::try_from(num.denominator.clone())?;
                match den_val {
                    1 => match num.sign {
                        Sign::Positive => Ok(Self::Positive(num.numerator)),
                        Sign::Negative => Ok(Self::Negative(num.numerator)),
                        Sign::Zero => unreachable!(),
                    },
                    _ => Err(MathError::ResultNotInDomain {
                        this_domain: "RationalNumbers".to_string(),
                        result_domain: "Integers".to_string(),
                    }),
                }
            }
        }
    }
}

macro_rules! impl_try_from_signed_primitive {
    ($($t:ty),*) => {
        $(
            impl<const BASE: u128> TryFrom<$t> for RationalNumber<BASE> {
                type Error = MathError;
                fn try_from(value: $t) -> Result<Self, Self::Error> {
                    let int_num = IntegerNumber::try_from(value)?;
                    Ok(Self::from(int_num))
                }
            }
        )*
    };
}
impl_try_from_signed_primitive!(i128, i64, i32, i16, i8, isize);

macro_rules! impl_try_from_unsigned_primitive {
    ($($t:ty),*) => {
        $(
            impl<const BASE: u128> TryFrom<$t> for RationalNumber<BASE> {
                type Error = MathError;
                fn try_from(value: $t) -> Result<Self, Self::Error> {
                    let int_num = IntegerNumber::try_from(value)?;
                    Ok(Self::from(int_num))
                }
            }
        )*
    };
}
impl_try_from_unsigned_primitive!(u128, u64, u32, u16, u8, usize);
```

- [ ] **Step 6: Run test to verify it passes**

Run: `cargo test math::rational_number::tests`
Expected: PASS.

- [ ] **Step 7: Run all tests in the workspace**

Run: `cargo test`
Expected: PASS.

- [ ] **Step 8: Commit**

```bash
git add src/math/mod.rs src/math/rational_number/
git commit -m "feat: implement RationalNumber type with conversions and GCD simplification"
```

## Verification Plan

### Automated Tests
- Run `cargo test` to execute all tests under `src/math/rational_number/tests.rs` and other test files.
- Run `cargo check` to ensure clean compilation.
