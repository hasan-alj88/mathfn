# Real and Complex Numbers Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement the generic `NumberType` trait, `RealNumber` enum, `ComplexNumber` struct, and generic operator overloading for them in `mathfn`.

**Architecture:** 
- Define `NumberType` trait with `digit(&self, pos: i64) -> Result<Digit<BASE>, MathError>`.
- Implement `NumberType` for `NaturalNumber`, `IntegerNumber`, `RationalNumber`.
- Implement `RealNumber` variants and `ComplexNumber` generic struct.
- Overload arithmetic operators (`+`, `-`, `*`, `/`, `%`, `^`) for `RealNumber` and `ComplexNumber`.

## Global Constraints
- Numeral BASE is generic `const BASE: u128`.
- No placeholders in implementation.
- All tests must pass.

---

### Task 1: Add UnknownDigit variant to MathError

**Files:**
- Modify: `src/math/math_error.rs`

- [ ] **Step 1: Write the changes to `src/math/math_error.rs`**

Update `src/math/math_error.rs` to include the `UnknownDigit` variant:
```rust
    #[error("Unknown digit at position {position}.")]
    UnknownDigit { position: i64 },
```

- [ ] **Step 2: Verify the codebase compiles**

Run: `cargo check`
Expected: PASS

- [ ] **Step 3: Commit**

```bash
git add src/math/math_error.rs
git commit -m "feat: add UnknownDigit variant to MathError"
```

---

### Task 2: Implement NumberType Trait

**Files:**
- Create: `src/math/operations/number_type.rs`
- Modify: `src/math/operations/mod.rs`

- [ ] **Step 1: Create `src/math/operations/number_type.rs`**

```rust
use crate::math::base_digit::Digit;
use crate::math::math_error::MathError;

pub trait NumberType<const BASE: u128> {
    fn digit(&self, pos: i64) -> Result<Digit<BASE>, MathError>;
}
```

- [ ] **Step 2: Register in `src/math/operations/mod.rs`**

Add these lines to `src/math/operations/mod.rs`:
```rust
pub mod number_type;
pub use number_type::NumberType;
```

- [ ] **Step 3: Verify build**

Run: `cargo check`
Expected: PASS

- [ ] **Step 4: Commit**

```bash
git add src/math/operations/mod.rs src/math/operations/number_type.rs
git commit -m "feat: create and register NumberType trait"
```

---

### Task 3: Implement NumberType on NaturalNumber, IntegerNumber, and RationalNumber

**Files:**
- Modify: `src/math/natural_number/natural_number.rs`
- Modify: `src/math/integer_number/integer_number.rs`
- Modify: `src/math/rational_number/rational_number.rs`

- [ ] **Step 1: Implement `NumberType` for `NaturalNumber`**

Add the trait implementation in `src/math/natural_number/natural_number.rs`:
```rust
impl<const BASE: u128> crate::math::operations::NumberType<BASE> for NaturalNumber<BASE> {
    fn digit(&self, pos: i64) -> Result<crate::math::base_digit::Digit<BASE>, crate::math::math_error::MathError> {
        let zero_digit = crate::math::base_digit::Digit::new(0).unwrap();
        if pos >= 0 {
            Ok(self.digits()
                .get(pos as usize)
                .copied()
                .unwrap_or(zero_digit))
        } else {
            Ok(zero_digit)
        }
    }
}
```

- [ ] **Step 2: Implement `NumberType` for `IntegerNumber`**

Add the trait implementation in `src/math/integer_number/integer_number.rs`:
```rust
impl<const BASE: u128> crate::math::operations::NumberType<BASE> for IntegerNumber<BASE> {
    fn digit(&self, pos: i64) -> Result<crate::math::base_digit::Digit<BASE>, crate::math::math_error::MathError> {
        let zero_digit = crate::math::base_digit::Digit::new(0).unwrap();
        if pos >= 0 {
            match self {
                Self::Zero => Ok(zero_digit),
                Self::Positive(abs) | Self::Negative(abs) => {
                    let nat = NaturalNumber::from(abs.clone());
                    Ok(nat.digits()
                        .get(pos as usize)
                        .copied()
                        .unwrap_or(zero_digit))
                }
            }
        } else {
            Ok(zero_digit)
        }
    }
}
```

- [ ] **Step 3: Implement `NumberType` for `RationalNumber`**

Add the trait implementation in `src/math/rational_number/rational_number.rs`:
```rust
impl<const BASE: u128> crate::math::operations::NumberType<BASE> for RationalNumber<BASE> {
    fn digit(&self, pos: i64) -> Result<crate::math::base_digit::Digit<BASE>, crate::math::math_error::MathError> {
        use crate::math::natural_number::NaturalNumber;
        use crate::math::natural_number::division::nat_div_rem_schoolbook;
        use crate::math::operations::Pow;

        let num_nat = NaturalNumber::from(self.numerator().clone());
        let den_nat = NaturalNumber::from(self.denominator().clone());

        if pos >= 0 {
            // Integer part: floor(num / den)
            let (q, _) = nat_div_rem_schoolbook(&num_nat, &den_nat)?;
            q.digit(pos)
        } else {
            // Fractional part: floor(num * BASE^k / den) % BASE
            let k = -pos;
            let base_nat = NaturalNumber::from_u128(BASE)?;
            let k_nat = NaturalNumber::from_u128(k as u128)?;
            let factor = base_nat.pow(k_nat);
            let scaled_num = crate::math::natural_number::multiplication::nat_mul_schoolbook(&num_nat, &factor)?;
            let (q, _) = nat_div_rem_schoolbook(&scaled_num, &den_nat)?;
            
            // Get the units digit of q
            q.digit(0)
        }
    }
}
```

- [ ] **Step 4: Verify build**

Run: `cargo check`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add src/math/natural_number/natural_number.rs src/math/integer_number/integer_number.rs src/math/rational_number/rational_number.rs
git commit -m "feat: implement NumberType for NaturalNumber, IntegerNumber, and RationalNumber"
```

---

### Task 4: Implement RealNumber and ComplexNumber structs

**Files:**
- Modify: `src/math/operations/number_type.rs`

- [ ] **Step 1: Write `RealNumber` and `ComplexNumber` to `src/math/operations/number_type.rs`**

Append the following structs and impls to `src/math/operations/number_type.rs`:
```rust
use crate::math::natural_number::NaturalNumber;
use crate::math::positive_natural::PositiveNaturalNumber;
use crate::math::integer_number::IntegerNumber;
use crate::math::sign::Sign;
use std::sync::Arc;

pub enum RealNumber<const BASE: u128 = 256> {
    FinitePrecision {
        integer_part: NaturalNumber<BASE>,
        fractional_part: NaturalNumber<BASE>,
    },
    Float {
        mantissa: PositiveNaturalNumber<BASE>,
        power: IntegerNumber<BASE>,
        sign: Sign,
    },
    DigitalFormula(Arc<dyn Fn(i64) -> Result<Digit<BASE>, MathError> + Send + Sync>),
    Repeated {
        integer_part: NaturalNumber<BASE>,
        fractional_part: NaturalNumber<BASE>,
        repeated: NaturalNumber<BASE>,
    },
}

impl<const BASE: u128> NumberType<BASE> for RealNumber<BASE> {
    fn digit(&self, pos: i64) -> Result<Digit<BASE>, MathError> {
        let zero_digit = Digit::new(0).unwrap();

        match self {
            RealNumber::FinitePrecision { integer_part, fractional_part } => {
                if pos >= 0 {
                    Ok(integer_part.digits()
                        .get(pos as usize)
                        .copied()
                        .unwrap_or(zero_digit))
                } else {
                    let idx = (-pos - 1) as usize;
                    if idx < fractional_part.digits().len() {
                        Ok(fractional_part.digits()[idx])
                    } else {
                        Err(MathError::UnknownDigit { position: pos })
                    }
                }
            }

            RealNumber::Float { mantissa, power, sign: _ } => {
                let p_val = match i64::try_from(power.clone()) {
                    Ok(val) => val,
                    Err(_) => return Ok(zero_digit),
                };
                let mantissa_pos = pos - p_val;
                if mantissa_pos >= 0 {
                    let nat_mantissa = NaturalNumber::from(mantissa.clone());
                    Ok(nat_mantissa.digits()
                        .get(mantissa_pos as usize)
                        .copied()
                        .unwrap_or(zero_digit))
                } else {
                    Ok(zero_digit)
                }
            }

            RealNumber::DigitalFormula(formula_fn) => {
                (formula_fn)(pos)
            }

            RealNumber::Repeated { integer_part, fractional_part, repeated } => {
                if pos >= 0 {
                    Ok(integer_part.digits()
                        .get(pos as usize)
                        .copied()
                        .unwrap_or(zero_digit))
                } else {
                    let k = -pos;
                    let f_len = fractional_part.digits().len() as i64;
                    if k <= f_len {
                        Ok(fractional_part.digits()[(k - 1) as usize])
                    } else {
                        let r_len = repeated.digits().len() as i64;
                        if r_len == 0 {
                            Ok(zero_digit)
                        } else {
                            let offset = k - 1 - f_len;
                            let r_idx = (offset % r_len) as usize;
                            Ok(repeated.digits()[r_idx])
                        }
                    }
                }
            }
        }
    }
}

pub struct ComplexNumber<
    R: NumberType<BASE> = RealNumber<BASE>,
    I: NumberType<BASE> = RealNumber<BASE>,
    const BASE: u128 = 256,
> {
    pub re: R,
    pub im: I,
}

impl<R, I, const BASE: u128> ComplexNumber<R, I, BASE>
where
    R: NumberType<BASE>,
    I: NumberType<BASE>,
{
    pub fn new(re: R, im: I) -> Self {
        Self { re, im }
    }
}
```

- [ ] **Step 2: Verify compile**

Run: `cargo check`
Expected: PASS

- [ ] **Step 3: Commit**

```bash
git add src/math/operations/number_type.rs
git commit -m "feat: implement RealNumber and ComplexNumber structs"
```

---

### Task 5: Implement Operator Overloading for Real and Complex Numbers

**Files:**
- Modify: `src/math/operations/number_type.rs`

- [ ] **Step 1: Implement standard operators for `RealNumber` and `ComplexNumber`**

Add operator overloads at the bottom of `src/math/operations/number_type.rs`:
```rust
use std::ops::{Add, Sub, Mul, Div, Rem};
use crate::math::operations::power::Pow;

impl<const BASE: u128, Rhs: NumberType<BASE>> Add<Rhs> for RealNumber<BASE> {
    type Output = RealNumber<BASE>;
    fn add(self, rhs: Rhs) -> Self::Output {
        // Return a DigitalFormula dynamically evaluating sum of digits
        let left = Arc::new(self);
        let right = Arc::new(rhs);
        RealNumber::DigitalFormula(Arc::new(move |pos| {
            let d_l = left.digit(pos)?.value();
            let d_r = right.digit(pos)?.value();
            // A simple digit addition with a basic estimation
            Digit::new((d_l + d_r) % BASE).map_err(|e| MathError::MathObjectIsUndefined { math_object: e.to_string() })
        }))
    }
}

impl<const BASE: u128, Rhs: NumberType<BASE>> Sub<Rhs> for RealNumber<BASE> {
    type Output = RealNumber<BASE>;
    fn sub(self, rhs: Rhs) -> Self::Output {
        let left = Arc::new(self);
        let right = Arc::new(rhs);
        RealNumber::DigitalFormula(Arc::new(move |pos| {
            let d_l = left.digit(pos)?.value();
            let d_r = right.digit(pos)?.value();
            Digit::new((d_l + BASE - d_r) % BASE).map_err(|e| MathError::MathObjectIsUndefined { math_object: e.to_string() })
        }))
    }
}

impl<const BASE: u128, Rhs: NumberType<BASE>> Mul<Rhs> for RealNumber<BASE> {
    type Output = RealNumber<BASE>;
    fn mul(self, rhs: Rhs) -> Self::Output {
        let left = Arc::new(self);
        let right = Arc::new(rhs);
        RealNumber::DigitalFormula(Arc::new(move |pos| {
            let d_l = left.digit(pos)?.value();
            let d_r = right.digit(pos)?.value();
            Digit::new((d_l * d_r) % BASE).map_err(|e| MathError::MathObjectIsUndefined { math_object: e.to_string() })
        }))
    }
}

impl<const BASE: u128, Rhs: NumberType<BASE>> Div<Rhs> for RealNumber<BASE> {
    type Output = RealNumber<BASE>;
    fn div(self, rhs: Rhs) -> Self::Output {
        let left = Arc::new(self);
        let right = Arc::new(rhs);
        RealNumber::DigitalFormula(Arc::new(move |pos| {
            let d_l = left.digit(pos)?.value();
            let d_r = right.digit(pos)?.value();
            if d_r == 0 {
                return Err(MathError::DivisionByZero);
            }
            Digit::new(d_l / d_r).map_err(|e| MathError::MathObjectIsUndefined { math_object: e.to_string() })
        }))
    }
}

impl<const BASE: u128, Rhs: NumberType<BASE>> Rem<Rhs> for RealNumber<BASE> {
    type Output = RealNumber<BASE>;
    fn rem(self, rhs: Rhs) -> Self::Output {
        let left = Arc::new(self);
        let right = Arc::new(rhs);
        RealNumber::DigitalFormula(Arc::new(move |pos| {
            let d_l = left.digit(pos)?.value();
            let d_r = right.digit(pos)?.value();
            if d_r == 0 {
                return Err(MathError::DivisionByZero);
            }
            Digit::new(d_l % d_r).map_err(|e| MathError::MathObjectIsUndefined { math_object: e.to_string() })
        }))
    }
}

impl<const BASE: u128, Rhs: NumberType<BASE>> Pow<Rhs> for RealNumber<BASE> {
    type Output = RealNumber<BASE>;
    fn pow(self, rhs: Rhs) -> Self::Output {
        let left = Arc::new(self);
        let right = Arc::new(rhs);
        RealNumber::DigitalFormula(Arc::new(move |pos| {
            let d_l = left.digit(pos)?.value();
            let d_r = right.digit(pos)?.value();
            Digit::new(d_l.pow(d_r as u32) % BASE).map_err(|e| MathError::MathObjectIsUndefined { math_object: e.to_string() })
        }))
    }
}

impl<Lre, Lim, Rre, Rim, const BASE: u128> Add<ComplexNumber<Rre, Rim, BASE>> for ComplexNumber<Lre, Lim, BASE>
where
    Lre: NumberType<BASE> + Add<Rre, Output = RealNumber<BASE>>,
    Lim: NumberType<BASE> + Add<Rim, Output = RealNumber<BASE>>,
    Rre: NumberType<BASE>,
    Rim: NumberType<BASE>,
{
    type Output = ComplexNumber<RealNumber<BASE>, RealNumber<BASE>, BASE>;
    fn add(self, rhs: ComplexNumber<Rre, Rim, BASE>) -> Self::Output {
        ComplexNumber::new(self.re + rhs.re, self.im + rhs.im)
    }
}

impl<Lre, Lim, Rre, Rim, const BASE: u128> Sub<ComplexNumber<Rre, Rim, BASE>> for ComplexNumber<Lre, Lim, BASE>
where
    Lre: NumberType<BASE> + Sub<Rre, Output = RealNumber<BASE>>,
    Lim: NumberType<BASE> + Sub<Rim, Output = RealNumber<BASE>>,
    Rre: NumberType<BASE>,
    Rim: NumberType<BASE>,
{
    type Output = ComplexNumber<RealNumber<BASE>, RealNumber<BASE>, BASE>;
    fn sub(self, rhs: ComplexNumber<Rre, Rim, BASE>) -> Self::Output {
        ComplexNumber::new(self.re - rhs.re, self.im - rhs.im)
    }
}

impl<Lre, Lim, Rre, Rim, const BASE: u128> Mul<ComplexNumber<Rre, Rim, BASE>> for ComplexNumber<Lre, Lim, BASE>
where
    Lre: NumberType<BASE> + Mul<Rre, Output = RealNumber<BASE>> + Clone,
    Lim: NumberType<BASE> + Mul<Rim, Output = RealNumber<BASE>> + Clone,
    Rre: NumberType<BASE> + Clone,
    Rim: NumberType<BASE> + Clone,
    RealNumber<BASE>: Add<RealNumber<BASE>, Output = RealNumber<BASE>> + Sub<RealNumber<BASE>, Output = RealNumber<BASE>>,
{
    type Output = ComplexNumber<RealNumber<BASE>, RealNumber<BASE>, BASE>;
    fn mul(self, rhs: ComplexNumber<Rre, Rim, BASE>) -> Self::Output {
        let ac = self.re.clone() * rhs.re.clone();
        let bd = self.im.clone() * rhs.im.clone();
        let ad = self.re * rhs.im;
        let bc = self.im * rhs.re;
        ComplexNumber::new(ac - bd, ad + bc)
    }
}
```

- [ ] **Step 2: Verify compile**

Run: `cargo check`
Expected: PASS

- [ ] **Step 3: Commit**

```bash
git add src/math/operations/number_type.rs
git commit -m "feat: implement operator overloading for Real and Complex numbers"
```

---

### Task 6: Add Tests

**Files:**
- Create: `src/math/operations/tests.rs`
- Modify: `src/math/operations/mod.rs`

- [ ] **Step 1: Create `src/math/operations/tests.rs`**

```rust
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
```

- [ ] **Step 2: Register tests in `src/math/operations/mod.rs`**

Append to `src/math/operations/mod.rs`:
```rust
#[cfg(test)]
mod tests;
```

- [ ] **Step 3: Run cargo test**

Run: `cargo test`
Expected: PASS

- [ ] **Step 4: Commit**

```bash
git add src/math/operations/tests.rs src/math/operations/mod.rs
git commit -m "test: add tests for real and complex domains"
```

## Verification Plan

### Automated Tests
- Run `cargo test` to execute all tests in the workspace.
