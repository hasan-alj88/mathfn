# IntegerNumber Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement the `PositiveNaturalNumber` and `IntegerNumber` types with failable conversion traits.

**Architecture:** 
- `PositiveNaturalNumber` wraps `NaturalNumber` with a -1 value offset.
- `IntegerNumber` is an enum with `Positive(PositiveNaturalNumber)`, `Negative(PositiveNaturalNumber)`, and `Zero` variants.
- The `Sign` enum is updated to include `Zero`.

**Tech Stack:** Rust (cargo, std, existing `mathfn` codebase).

## Global Constraints
- `BASE` parameter defaults to `256`.
- Value offset for `PositiveNaturalNumber` is -1 (mathematical value is `internal_val + 1`).
- `Sign` enum must include `Positive`, `Negative`, and `Zero`.

---

## Proposed Changes

### [mathfn]

#### [MODIFY] [sign.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/sign.rs)
Update `Sign` enum to include `Zero`.

#### [MODIFY] [mod.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/mod.rs)
Register `positive_natural` and `integer_number` modules.

#### [NEW] [mod.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/positive_natural/mod.rs)
`positive_natural` module declaration.

#### [NEW] [positive_natural.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/positive_natural/positive_natural.rs)
`PositiveNaturalNumber` struct definition and conversion implementations.

#### [NEW] [mod.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/integer_number/mod.rs)
`integer_number` module declaration.

#### [NEW] [integer_number.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/integer_number/integer_number.rs)
`IntegerNumber` enum definition and conversion implementations.

#### [NEW] [tests.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/integer_number/tests.rs)
Unit tests for `PositiveNaturalNumber` and `IntegerNumber`.

---

### Task 1: Update Sign Enum

**Files:**
- Modify: `src/math/sign.rs`

**Interfaces:**
- Produces: `pub enum Sign` with `Positive`, `Negative`, `Zero` variants.

- [ ] **Step 1: Write failing test in `src/math/natural_number/tests.rs`**

```rust
#[test]
fn test_sign_enum_zero() {
    use crate::math::sign::Sign;
    let s = Sign::Zero;
    assert_eq!(s, Sign::Zero);
}
```

Wait, let's append this test to the end of `src/math/natural_number/tests.rs`.

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test math::natural_number::tests::test_sign_enum_zero`
Expected: FAIL (no variant `Zero` in `Sign`).

- [ ] **Step 3: Update `src/math/sign.rs`**

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sign {
    Positive,
    Negative,
    Zero,
}
```

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test math::natural_number::tests::test_sign_enum_zero`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add src/math/sign.rs src/math/natural_number/tests.rs
git commit -m "feat: add Zero variant to Sign enum"
```

---

### Task 2: PositiveNaturalNumber Implementation

**Files:**
- Modify: `src/math/mod.rs`
- Create: `src/math/positive_natural/mod.rs`
- Create: `src/math/positive_natural/positive_natural.rs`

**Interfaces:**
- Produces: `pub struct PositiveNaturalNumber<const BASE: u128 = 256>`, constructors, and `TryFrom` / `From` conversions.

- [ ] **Step 1: Register module in `src/math/mod.rs`**

Add `pub mod positive_natural;` to `src/math/mod.rs`.

- [ ] **Step 2: Create `src/math/positive_natural/mod.rs`**

```rust
pub mod positive_natural;
pub use positive_natural::PositiveNaturalNumber;
```

- [ ] **Step 3: Write failing test in `src/math/natural_number/tests.rs`**

```rust
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
```

- [ ] **Step 4: Run test to verify it fails**

Run: `cargo test math::natural_number::tests::test_positive_natural_basics`
Expected: FAIL (compilation error, module or struct not found).

- [ ] **Step 5: Implement `PositiveNaturalNumber` in `src/math/positive_natural/positive_natural.rs`**

```rust
use crate::math::natural_number::NaturalNumber;
use crate::math::natural_number::addition::nat_add_schoolbook;
use crate::math::natural_number::multiplication::nat_sub_schoolbook;
use crate::math::math_error::MathError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PositiveNaturalNumber<const BASE: u128 = 256>(NaturalNumber<BASE>);

impl<const BASE: u128> PositiveNaturalNumber<BASE> {
    pub fn new_raw(internal_val: NaturalNumber<BASE>) -> Self {
        Self(internal_val)
    }

    pub fn offset_val(&self) -> &NaturalNumber<BASE> {
        &self.0
    }
}

impl<const BASE: u128> TryFrom<u128> for PositiveNaturalNumber<BASE> {
    type Error = MathError;
    fn try_from(value: u128) -> Result<Self, Self::Error> {
        match value {
            0 => Err(MathError::ResultNotInDomain {
                this_domain: "PositiveNaturalNumbers".to_string(),
                result_domain: "Zero".to_string(),
            }),
            _ => {
                let internal_num = NaturalNumber::from_u128(value - 1)?;
                Ok(Self(internal_num))
            }
        }
    }
}

impl<const BASE: u128> TryFrom<PositiveNaturalNumber<BASE>> for u128 {
    type Error = MathError;
    fn try_from(num: PositiveNaturalNumber<BASE>) -> Result<Self, Self::Error> {
        let internal_val = num.0.to_u128()?;
        internal_val.checked_add(1).ok_or(MathError::QuotientOverflow)
    }
}

impl<const BASE: u128> TryFrom<NaturalNumber<BASE>> for PositiveNaturalNumber<BASE> {
    type Error = MathError;
    fn try_from(num: NaturalNumber<BASE>) -> Result<Self, Self::Error> {
        match num.is_zero() {
            true => Err(MathError::ResultNotInDomain {
                this_domain: "PositiveNaturalNumbers".to_string(),
                result_domain: "Zero".to_string(),
            }),
            false => {
                let one = NaturalNumber::from_u128(1)?;
                let offset_num = nat_sub_schoolbook(&num, &one)?;
                Ok(Self(offset_num))
            }
        }
    }
}

impl<const BASE: u128> From<PositiveNaturalNumber<BASE>> for NaturalNumber<BASE> {
    fn from(num: PositiveNaturalNumber<BASE>) -> Self {
        let one = NaturalNumber::from_u128(1).unwrap();
        nat_add_schoolbook(&num.0, &one).unwrap()
    }
}
```

- [ ] **Step 6: Run test to verify it passes**

Run: `cargo test math::natural_number::tests::test_positive_natural_basics`
Expected: PASS

- [ ] **Step 7: Commit**

```bash
git add src/math/mod.rs src/math/positive_natural/ src/math/natural_number/tests.rs
git commit -m "feat: implement PositiveNaturalNumber struct and conversions"
```

---

### Task 3: IntegerNumber Implementation & Tests

**Files:**
- Modify: `src/math/mod.rs`
- Create: `src/math/integer_number/mod.rs`
- Create: `src/math/integer_number/integer_number.rs`
- Create: `src/math/integer_number/tests.rs`

**Interfaces:**
- Produces: `pub enum IntegerNumber<const BASE: u128 = 256>`, sign & abs accessors, From/TryFrom conversions.

- [ ] **Step 1: Register module in `src/math/mod.rs`**

Add `pub mod integer_number;` to `src/math/mod.rs`.

- [ ] **Step 2: Create `src/math/integer_number/mod.rs`**

```rust
#[cfg(test)]
mod tests;

pub mod integer_number;
pub use integer_number::IntegerNumber;
```

- [ ] **Step 3: Create tests file `src/math/integer_number/tests.rs` with failing tests**

```rust
use crate::math::integer_number::IntegerNumber;
use crate::math::positive_natural::PositiveNaturalNumber;
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
```

- [ ] **Step 4: Run test to verify it fails**

Run: `cargo test math::integer_number::tests`
Expected: FAIL (compilation error, module or enum not found).

- [ ] **Step 5: Implement `IntegerNumber` in `src/math/integer_number/integer_number.rs`**

```rust
use crate::math::sign::Sign;
use crate::math::positive_natural::PositiveNaturalNumber;
use crate::math::natural_number::NaturalNumber;
use crate::math::math_error::MathError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IntegerNumber<const BASE: u128 = 256> {
    Positive(PositiveNaturalNumber<BASE>),
    Negative(PositiveNaturalNumber<BASE>),
    Zero,
}

impl<const BASE: u128> IntegerNumber<BASE> {
    pub fn sign(&self) -> Sign {
        match self {
            Self::Positive(_) => Sign::Positive,
            Self::Negative(_) => Sign::Negative,
            Self::Zero => Sign::Zero,
        }
    }

    pub fn abs(&self) -> Option<PositiveNaturalNumber<BASE>> {
        match self {
            Self::Positive(abs) | Self::Negative(abs) => Some(abs.clone()),
            Self::Zero => None,
        }
    }
}

impl<const BASE: u128> From<NaturalNumber<BASE>> for IntegerNumber<BASE> {
    fn from(num: NaturalNumber<BASE>) -> Self {
        match num.is_zero() {
            true => Self::Zero,
            false => Self::Positive(PositiveNaturalNumber::try_from(num).unwrap()),
        }
    }
}

impl<const BASE: u128> TryFrom<IntegerNumber<BASE>> for NaturalNumber<BASE> {
    type Error = MathError;
    fn try_from(num: IntegerNumber<BASE>) -> Result<Self, Self::Error> {
        match num {
            IntegerNumber::Zero => Ok(NaturalNumber::new(Vec::new())),
            IntegerNumber::Positive(abs) => Ok(NaturalNumber::from(abs)),
            IntegerNumber::Negative(_) => Err(MathError::ResultNotInDomain {
                this_domain: "NaturalNumbers".to_string(),
                result_domain: "NegativeIntegers".to_string(),
            }),
        }
    }
}

macro_rules! impl_try_from_signed_primitive {
    ($($t:ty),*) => {
        $(
            impl<const BASE: u128> TryFrom<$t> for IntegerNumber<BASE> {
                type Error = MathError;
                fn try_from(value: $t) -> Result<Self, Self::Error> {
                    match value.cmp(&0) {
                        std::cmp::Ordering::Equal => Ok(Self::Zero),
                        std::cmp::Ordering::Greater => {
                            let abs = PositiveNaturalNumber::try_from(value as u128)?;
                            Ok(Self::Positive(abs))
                        }
                        std::cmp::Ordering::Less => {
                            let abs = PositiveNaturalNumber::try_from(value.unsigned_abs() as u128)?;
                            Ok(Self::Negative(abs))
                        }
                    }
                }
            }
        )*
    };
}
impl_try_from_signed_primitive!(i128, i64, i32, i16, i8, isize);

macro_rules! impl_try_from_unsigned_primitive {
    ($($t:ty),*) => {
        $(
            impl<const BASE: u128> TryFrom<$t> for IntegerNumber<BASE> {
                type Error = MathError;
                fn try_from(value: $t) -> Result<Self, Self::Error> {
                    match value {
                        0 => Ok(Self::Zero),
                        _ => {
                            let abs = PositiveNaturalNumber::try_from(value as u128)?;
                            Ok(Self::Positive(abs))
                        }
                    }
                }
            }
        )*
    };
}
impl_try_from_unsigned_primitive!(u128, u64, u32, u16, u8, usize);

macro_rules! impl_try_into_signed_primitive {
    ($($t:ty),*) => {
        $(
            impl<const BASE: u128> TryFrom<IntegerNumber<BASE>> for $t {
                type Error = MathError;
                fn try_from(num: IntegerNumber<BASE>) -> Result<Self, Self::Error> {
                    match num {
                        IntegerNumber::Zero => Ok(0),
                        IntegerNumber::Positive(abs) => {
                            let val_u128 = u128::try_from(abs)?;
                            <$t>::try_from(val_u128).map_err(|_| MathError::QuotientOverflow)
                        }
                        IntegerNumber::Negative(abs) => {
                            let val_u128 = u128::try_from(abs)?;
                            let val_signed = <$t>::try_from(val_u128).map_err(|_| MathError::QuotientOverflow)?;
                            val_signed.checked_neg().ok_or(MathError::QuotientOverflow)
                        }
                    }
                }
            }
        )*
    };
}
impl_try_into_signed_primitive!(i128, i64, i32, i16, i8, isize);

macro_rules! impl_try_into_unsigned_primitive {
    ($($t:ty),*) => {
        $(
            impl<const BASE: u128> TryFrom<IntegerNumber<BASE>> for $t {
                type Error = MathError;
                fn try_from(num: IntegerNumber<BASE>) -> Result<Self, Self::Error> {
                    match num {
                        IntegerNumber::Zero => Ok(0),
                        IntegerNumber::Positive(abs) => {
                            let val_u128 = u128::try_from(abs)?;
                            <$t>::try_from(val_u128).map_err(|_| MathError::QuotientOverflow)
                        }
                        IntegerNumber::Negative(_) => Err(MathError::ResultNotInDomain {
                            this_domain: "UnsignedPrimitives".to_string(),
                            result_domain: "NegativeIntegers".to_string(),
                        }),
                    }
                }
            }
        )*
    };
}
impl_try_into_unsigned_primitive!(u128, u64, u32, u16, u8, usize);
```

- [ ] **Step 6: Run test to verify it passes**

Run: `cargo test math::integer_number::tests`
Expected: PASS

- [ ] **Step 7: Run all tests in the workspace**

Run: `cargo test`
Expected: PASS

- [ ] **Step 8: Commit**

```bash
git add src/math/mod.rs src/math/integer_number/
git commit -m "feat: implement IntegerNumber enum with failable conversions"
```

## Verification Plan

### Automated Tests
- Run `cargo test` to execute all tests under `src/math/integer_number/tests.rs` and other test files.
- Run `cargo check` to ensure clean compilation.
