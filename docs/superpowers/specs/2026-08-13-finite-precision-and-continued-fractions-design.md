# Design Specification: FinitePrecision and Continued Fraction Number Types

## Goal Description
Refactor the real and complex number domains to:
1. Introduce a standalone, exact `FinitePrecision` struct representing exact decimals/pos-numerals.
2. Refactor `RealNumber` to have `ExactFinite` and `Approximate` variants wrapping `FinitePrecision`.
3. Introduce `FiniteContinuedFractionNumber` representing exact finite continued fractions.
4. Add `FiniteContinuedFraction`, `RepeatedContinuedFraction`, and `ApproximateContinuedFraction` variants to `RealNumber`.
5. Implement `NumberType` for all new types and variants using match patterns.

## Proposed Changes

We will update `src/math/operations/number_type.rs` and add unit tests.

### Directory Layout
- `src/math/operations/number_type.rs` [MODIFY] - Add `FinitePrecision`, `FiniteContinuedFractionNumber`, update `RealNumber`, update operator impls.
- `src/math/operations/tests.rs` [MODIFY] - Add tests for new variants and conversions.

---

## Component Details

### 1. `FinitePrecision` Struct
```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FinitePrecision<const BASE: u128 = 256> {
    pub integer_part: NaturalNumber<BASE>,
    pub fractional_part: NaturalNumber<BASE>,
}
```

### 2. `FiniteContinuedFractionNumber` Struct
```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FiniteContinuedFractionNumber<const BASE: u128 = 256> {
    pub integer_part: IntegerNumber<BASE>,
    pub coefficients: Vec<PositiveNaturalNumber<BASE>>,
}
```

### 3. Updated `RealNumber` Enum
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
```

---

## Verification Plan

### Automated Tests
- Test exact digit queries on `FinitePrecision` yielding 0 out of bounds.
- Test conversion of finite continued fractions to exact rational numbers.
- Test digit queries on `FiniteContinuedFractionNumber` and `ApproximateContinuedFraction`.
- Test repeated continued fractions convergents threshold.
