# Design Specification: Real and Complex Number Domains with Digit-wise arithmetic

## Goal Description
Implement:
1. A generic `NumberType<const BASE: u128>` trait with failable `digit(pos)` retrieval.
2. Implementations of `NumberType` for exact domains: `NaturalNumber`, `IntegerNumber`, and `RationalNumber`.
3. A `RealNumber` enum supporting finite precision, float representation, repeating representations, and digital formulas.
4. A generic `ComplexNumber` struct defaulting to `RealNumber` components.
5. Generic operator overloading (`+`, `-`, `*`, `/`, `%`, `^`) requiring components to agree on numeral base `BASE`.

## Proposed Changes

We will introduce a new module under `src/math/operations/number_type.rs` and update existing domains.

### Directory Layout
- `src/math/operations/number_type.rs` [NEW] - Trait definition, `RealNumber` enum, `ComplexNumber` struct, and operator impls.
- `src/math/operations/mod.rs` [MODIFY] - Register `number_type` module.
- `src/math/math_error.rs` [MODIFY] - Add `UnknownDigit` error variant.
- `src/math/natural_number/mod.rs` [MODIFY] - Implement `NumberType` for `NaturalNumber`.
- `src/math/integer_number/mod.rs` [MODIFY] - Implement `NumberType` for `IntegerNumber`.
- `src/math/rational_number/rational_number.rs` [MODIFY] - Implement `NumberType` for `RationalNumber`.

---

## Component Details

### 1. `NumberType` Trait
```rust
pub trait NumberType<const BASE: u128> {
    /// Retrieve digit at positional index `pos`. 
    /// - `pos >= 0` indices integer part.
    /// - `pos < 0` indices fractional part.
    fn digit(&self, pos: i64) -> Result<Digit<BASE>, MathError>;
}
```

### 2. `RealNumber` Enum representation
```rust
pub enum RealNumber<const BASE: u128 = 256> {
    FinitePrecision {
        integer_part: NaturalNumber<BASE>,
        fractional_part: NaturalNumber<BASE>, // MSD-first order
    },
    Float {
        mantissa: PositiveNaturalNumber<BASE>,
        power: IntegerNumber<BASE>,
        sign: Sign,
    },
    DigitalFormula(Arc<dyn Fn(i64) -> Result<Digit<BASE>, MathError> + Send + Sync>),
    Repeated {
        integer_part: NaturalNumber<BASE>,
        fractional_part: NaturalNumber<BASE>, // MSD-first non-repeating section
        repeated: NaturalNumber<BASE>,        // MSD-first repeating section
    },
}
```

### 3. `ComplexNumber` representation
```rust
pub struct ComplexNumber<
    R: NumberType<BASE> = RealNumber<BASE>,
    I: NumberType<BASE> = RealNumber<BASE>,
    const BASE: u128 = 256,
> {
    pub re: R,
    pub im: I,
}
```

---

## Verification Plan

### Automated Tests
- Implement unit tests for:
  - Digit extraction logic of `RealNumber` variants.
  - Verification of `UnknownDigit` error propagation for `FinitePrecision`.
  - Verification of repeating decimal periodic retrieval.
  - Complex number addition, subtraction, and multiplication correctness.
