# FinitePrecision and Continued Fractions Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement standalone `FinitePrecision` and continued fraction domains (`FiniteContinuedFractionNumber`, `RepeatedContinuedFraction`, `ApproximateContinuedFraction`) and integrate them into the `RealNumber` enum and arithmetic operations.

**Architecture:**
- Create `FinitePrecision` representing exact decimals.
- Create `FiniteContinuedFractionNumber` with exact rational conversion converters using convergence recurrence relations.
- Update `RealNumber` variants to use `ExactFinite`, `Approximate`, `FiniteContinuedFraction`, `RepeatedContinuedFraction`, and `ApproximateContinuedFraction`.
- Update operator overloads on `RealNumber`.

## Global Constraints
- Use match pattern matching instead of `if-else`.
- All operations generic over `BASE`.
- All tests must pass.

---

### Task 1: Implement FinitePrecision Struct

**Files:**
- Modify: `src/math/operations/number_type.rs`

- [ ] **Step 1: Write `FinitePrecision` definition and its `NumberType` impl**

Open `src/math/operations/number_type.rs` and add the `FinitePrecision` struct:
```rust
/// A finite positional expansion representing an exact value.
///
/// $$V = \text{integer\_part} + \text{fractional\_part} \cdot \text{BASE}^{-\text{len}}$$
/// Digits outside the stored ranges are exactly 0.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FinitePrecision<const BASE: u128 = 256> {
    pub integer_part: NaturalNumber<BASE>,
    pub fractional_part: NaturalNumber<BASE>, // MSD-first order
}

impl<const BASE: u128> NumberType<BASE> for FinitePrecision<BASE> {
    fn digit(&self, pos: i64) -> Result<Digit<BASE>, MathError> {
        let zero_digit = Digit::new(0).unwrap();
        match pos {
            p if p >= 0 => {
                Ok(self.integer_part.digits()
                    .get(p as usize)
                    .copied()
                    .unwrap_or(zero_digit))
            }
            p => {
                let idx = (-p - 1) as usize;
                Ok(self.fractional_part.digits()
                    .get(idx)
                    .copied()
                    .unwrap_or(zero_digit))
            }
        }
    }
}
```

- [ ] **Step 2: Verify build**

Run: `cargo check`
Expected: PASS

- [ ] **Step 3: Commit**

```bash
git add src/math/operations/number_type.rs
git commit -m "feat: implement FinitePrecision struct"
```

---

### Task 2: Implement FiniteContinuedFractionNumber Struct

**Files:**
- Modify: `src/math/operations/number_type.rs`

- [ ] **Step 1: Write `FiniteContinuedFractionNumber` and `to_rational` conversion**

Add `FiniteContinuedFractionNumber` definition and helpers to `src/math/operations/number_type.rs`:
```rust
/// A finite continued fraction representation.
/// Represents a rational number exactly.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FiniteContinuedFractionNumber<const BASE: u128 = 256> {
    pub integer_part: IntegerNumber<BASE>,                // a_0
    pub coefficients: Vec<PositiveNaturalNumber<BASE>>,    // a_1, a_2, ..., a_n
}

fn to_sign_magnitude<const BASE: u128>(val: &IntegerNumber<BASE>) -> (Sign, NaturalNumber<BASE>) {
    match val {
        IntegerNumber::Zero => (Sign::Zero, NaturalNumber::new(Vec::new())),
        IntegerNumber::Positive(pos) => (Sign::Positive, NaturalNumber::from(pos.clone())),
        IntegerNumber::Negative(pos) => (Sign::Negative, NaturalNumber::from(pos.clone())),
    }
}

fn from_sign_magnitude<const BASE: u128>(sign: Sign, magnitude: NaturalNumber<BASE>) -> Result<IntegerNumber<BASE>, MathError> {
    match magnitude.is_zero() {
        true => Ok(IntegerNumber::Zero),
        false => match sign {
            Sign::Zero => Ok(IntegerNumber::Zero),
            Sign::Positive => {
                let pos = PositiveNaturalNumber::try_from(magnitude)?;
                Ok(IntegerNumber::Positive(pos))
            }
            Sign::Negative => {
                let pos = PositiveNaturalNumber::try_from(magnitude)?;
                Ok(IntegerNumber::Negative(pos))
            }
        }
    }
}

fn mul_integers<const BASE: u128>(
    a: &IntegerNumber<BASE>,
    b: &IntegerNumber<BASE>,
) -> Result<IntegerNumber<BASE>, MathError> {
    let (s_a, m_a) = to_sign_magnitude(a);
    let (s_b, m_b) = to_sign_magnitude(b);
    let m_prod = crate::math::natural_number::multiplication::nat_mul_schoolbook(&m_a, &m_b)?;
    let s_prod = match (s_a, s_b) {
        (Sign::Zero, _) | (_, Sign::Zero) => Sign::Zero,
        (Sign::Positive, Sign::Positive) | (Sign::Negative, Sign::Negative) => Sign::Positive,
        _ => Sign::Negative,
    };
    from_sign_magnitude(s_prod, m_prod)
}

fn add_integers<const BASE: u128>(
    a: &IntegerNumber<BASE>,
    b: &IntegerNumber<BASE>,
) -> Result<IntegerNumber<BASE>, MathError> {
    let (s_a, m_a) = to_sign_magnitude(a);
    let (s_b, m_b) = to_sign_magnitude(b);

    match s_a == Sign::Zero {
        true => Ok(b.clone()),
        false => match s_b == Sign::Zero {
            true => Ok(a.clone()),
            false => {
                match s_a == s_b {
                    true => {
                        let m_sum = crate::math::natural_number::addition::nat_add_schoolbook(&m_a, &m_b)?;
                        from_sign_magnitude(s_a, m_sum)
                    }
                    false => {
                        match m_a.cmp(&m_b) {
                            std::cmp::Ordering::Equal => Ok(IntegerNumber::Zero),
                            std::cmp::Ordering::Greater => {
                                let m_diff = crate::math::natural_number::multiplication::nat_sub_schoolbook(&m_a, &m_b)?;
                                from_sign_magnitude(s_a, m_diff)
                            }
                            std::cmp::Ordering::Less => {
                                let m_diff = crate::math::natural_number::multiplication::nat_sub_schoolbook(&m_b, &m_a)?;
                                from_sign_magnitude(s_b, m_diff)
                            }
                        }
                    }
                }
            }
        }
    }
}

impl<const BASE: u128> FiniteContinuedFractionNumber<BASE> {
    pub fn to_rational(&self) -> Result<crate::math::rational_number::RationalNumber<BASE>, MathError> {
        let mut p_prev2 = IntegerNumber::Zero;
        let mut p_prev1 = IntegerNumber::try_from(1i128)?;
        let mut q_prev2 = IntegerNumber::try_from(1i128)?;
        let mut q_prev1 = IntegerNumber::Zero;

        let mut p = self.integer_part.clone();
        let mut q = IntegerNumber::try_from(1i128)?;

        p_prev2 = p_prev1;
        p_prev1 = p.clone();
        q_prev2 = q_prev1;
        q_prev1 = q.clone();

        for coeff in &self.coefficients {
            let a_k = IntegerNumber::Positive(coeff.clone());
            p = add_integers(&mul_integers(&a_k, &p_prev1)?, &p_prev2)?;
            q = add_integers(&mul_integers(&a_k, &q_prev1)?, &q_prev2)?;

            p_prev2 = p_prev1;
            p_prev1 = p.clone();
            q_prev2 = q_prev1;
            q_prev1 = q.clone();
        }

        let p_nat = match p {
            IntegerNumber::Zero => NaturalNumber::new(Vec::new()),
            IntegerNumber::Positive(abs) | IntegerNumber::Negative(abs) => NaturalNumber::from(abs),
        };
        let q_nat = match q {
            IntegerNumber::Zero => return Err(MathError::DivisionByZero),
            IntegerNumber::Positive(abs) | IntegerNumber::Negative(abs) => PositiveNaturalNumber::try_from(NaturalNumber::from(abs))?,
        };

        crate::math::rational_number::RationalNumber::new(p.sign(), PositiveNaturalNumber::try_from(p_nat)?, q_nat)
    }
}

impl<const BASE: u128> NumberType<BASE> for FiniteContinuedFractionNumber<BASE> {
    fn digit(&self, pos: i64) -> Result<Digit<BASE>, MathError> {
        self.to_rational()?.digit(pos)
    }
}
```

- [ ] **Step 2: Verify build**

Run: `cargo check`
Expected: PASS

- [ ] **Step 3: Commit**

```bash
git add src/math/operations/number_type.rs
git commit -m "feat: implement FiniteContinuedFractionNumber and rational conversion"
```

---

### Task 3: Update RealNumber Enum Variants and NumberType digit Implementation

**Files:**
- Modify: `src/math/operations/number_type.rs`

- [ ] **Step 1: Replace `RealNumber` definition and `NumberType` impl**

Update `RealNumber` and `NumberType` implementation to support the new variants and match-based digit retrieval:
```rust
pub enum RealNumber<const BASE: u128 = 256> {
    ExactFinite(FinitePrecision<BASE>),
    Approximate(FinitePrecision<BASE>),
    FiniteContinuedFraction(FiniteContinuedFractionNumber<BASE>),
    RepeatedContinuedFraction {
        integer_part: IntegerNumber<BASE>,
        non_repeating: Vec<PositiveNaturalNumber<BASE>>,
        repeating: Vec<PositiveNaturalNumber<BASE>>,
    },
    ApproximateContinuedFraction(FiniteContinuedFractionNumber<BASE>),
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
            RealNumber::ExactFinite(fp) => fp.digit(pos),

            RealNumber::Approximate(fp) => {
                match pos {
                    p if p >= 0 => fp.digit(p),
                    p => {
                        let idx = (-p - 1) as usize;
                        match idx < fp.fractional_part.digits().len() {
                            true => fp.digit(p),
                            false => Err(MathError::UnknownDigit { position: p }),
                        }
                    }
                }
            }

            RealNumber::FiniteContinuedFraction(fcf) => fcf.digit(pos),

            RealNumber::ApproximateContinuedFraction(fcf) => {
                // Return digit from the simplified rational if index is within the precision interval
                let rat = fcf.to_rational()?;
                let q_n = NaturalNumber::from(rat.denominator().clone());
                
                // Interval width is 1 / (q_n * q_{n-1}). As an approximation limit, we check if pos is within scope.
                // For this, we check standard bounds or allow queries down to the base length of denominator.
                let q_len = q_n.digits().len() as i64;
                match pos < -q_len {
                    true => Err(MathError::UnknownDigit { position: pos }),
                    false => rat.digit(pos),
                }
            }

            RealNumber::RepeatedContinuedFraction { integer_part, non_repeating, repeating } => {
                // Dynamically evaluate convergent p_k / q_k periodically
                // For simplicity, generate the first few terms matching the non_repeating + repeating parts
                let mut coeffs = non_repeating.clone();
                match repeating.is_empty() {
                    true => {}
                    false => {
                        for _ in 0..10 {
                            coeffs.extend(repeating.clone());
                        }
                    }
                }
                let fcf = FiniteContinuedFractionNumber {
                    integer_part: integer_part.clone(),
                    coefficients: coeffs,
                };
                fcf.digit(pos)
            }

            RealNumber::Float { mantissa, power, sign: _ } => {
                let p_val = match i64::try_from(power.clone()) {
                    Ok(val) => val,
                    Err(_) => return Ok(zero_digit),
                };
                let mantissa_pos = pos - p_val;
                match mantissa_pos >= 0 {
                    true => {
                        let nat_mantissa = NaturalNumber::from(mantissa.clone());
                        Ok(nat_mantissa.digits()
                            .get(mantissa_pos as usize)
                            .copied()
                            .unwrap_or(zero_digit))
                    }
                    false => Ok(zero_digit),
                }
            }

            RealNumber::DigitalFormula(formula_fn) => {
                (formula_fn)(pos)
            }

            RealNumber::Repeated { integer_part, fractional_part, repeated } => {
                match pos >= 0 {
                    true => {
                        Ok(integer_part.digits()
                            .get(pos as usize)
                            .copied()
                            .unwrap_or(zero_digit))
                    }
                    false => {
                        let k = -pos;
                        let f_len = fractional_part.digits().len() as i64;
                        match k <= f_len {
                            true => Ok(fractional_part.digits()[(k - 1) as usize]),
                            false => {
                                let r_len = repeated.digits().len() as i64;
                                match r_len == 0 {
                                    true => Ok(zero_digit),
                                    false => {
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
        }
    }
}
```

- [ ] **Step 2: Verify build**

Run: `cargo check`
Expected: PASS

- [ ] **Step 3: Commit**

```bash
git add src/math/operations/number_type.rs
git commit -m "feat: refactor RealNumber enum with continued fraction variants"
```

---

### Task 4: Add Continued Fractions Unit Tests

**Files:**
- Modify: `src/math/operations/tests.rs`

- [ ] **Step 1: Add test cases to `src/math/operations/tests.rs`**

```rust
use crate::math::positive_natural::PositiveNaturalNumber;
use crate::math::integer_number::IntegerNumber;
use crate::math::operations::number_type::{FiniteContinuedFractionNumber, FinitePrecision};

#[test]
fn test_finite_precision_exact() {
    let fp = FinitePrecision::<256> {
        integer_part: NaturalNumber::from_u128(5).unwrap(),
        fractional_part: NaturalNumber::from_u128(20).unwrap(),
    };
    // Exact finite precision returns Ok(0) out of bounds
    assert_eq!(fp.digit(0).unwrap().value(), 5);
    assert_eq!(fp.digit(-1).unwrap().value(), 20);
    assert_eq!(fp.digit(-2).unwrap().value(), 0);
}

#[test]
fn test_continued_fraction_conversions() {
    // [1; 2] = 1 + 1/2 = 3/2 = 1.5
    let fcf = FiniteContinuedFractionNumber::<256> {
        integer_part: IntegerNumber::try_from(1i128).unwrap(),
        coefficients: vec![PositiveNaturalNumber::try_from(2u128).unwrap()],
    };

    let rat = fcf.to_rational().unwrap();
    assert_eq!(u128::try_from(rat.numerator().clone()).unwrap(), 3);
    assert_eq!(u128::try_from(rat.denominator().clone()).unwrap(), 2);
}
```

- [ ] **Step 2: Run all tests in the workspace**

Run: `cargo test`
Expected: PASS

- [ ] **Step 3: Commit**

```bash
git add src/math/operations/tests.rs
git commit -m "test: verify FinitePrecision exactness and Continued Fraction rational conversion"
```

## Verification Plan

### Automated Tests
- Run `cargo test` to execute all unit tests.
