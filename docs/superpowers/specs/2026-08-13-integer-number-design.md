# Design Specification: PositiveNaturalNumber and IntegerNumber

## Goal Description
Implement two new types:
1. `PositiveNaturalNumber` representing positive natural integers ($Z^+ = \{1, 2, 3, \dots\}$).
   To represent values starting from 1, the underlying storage is offset by -1 (i.e. digit value represents $V - 1$), meaning zero is not representable.
2. `IntegerNumber` representing signed integers, implemented as a Rust enum with `Positive`, `Negative`, and `Zero` variants.

We will also:
- Update the existing `Sign` enum in `src/math/sign.rs` to include a `Zero` variant.
- Implement failable conversion traits (`TryFrom`/`TryInto`) between these types and all primitives, and non-failable conversions from `NaturalNumber` to `IntegerNumber`.

## User Review Required
No major user review required as the scope has been aligned:
- `IntegerNumber` is implemented directly as an enum rather than a struct.
- `PositiveNaturalNumber` has a custom folder/module under `src/math/positive_natural/`.
- `IntegerNumber` has a custom folder/module under `src/math/integer_number/`.

## Proposed Changes

We will introduce two new modules under `src/math/`.

### Directory Layout
- `src/math/sign.rs` - Modify to add `Zero` to the `Sign` enum.
- `src/math/positive_natural/mod.rs` - Module registration and exports.
- `src/math/positive_natural/positive_natural.rs` - `PositiveNaturalNumber` struct definition and conversions.
- `src/math/integer_number/mod.rs` - Module registration and exports.
- `src/math/integer_number/integer_number.rs` - `IntegerNumber` enum definition and conversions.
- `src/math/integer_number/tests.rs` - Unit tests for conversions and behavior.

---

### Component Details

#### 1. Sign Enum (`src/math/sign.rs`)
Update to:
```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sign {
    Positive,
    Negative,
    Zero,
}
```

#### 2. Positive Natural Number (`src/math/positive_natural/positive_natural.rs`)
- `pub struct PositiveNaturalNumber<const BASE: u128 = 256>(NaturalNumber<BASE>);`
- Implements `TryFrom` for signed/unsigned primitives and `NaturalNumber` (returns `MathError::ResultNotInDomain` on zero or negative values).
- Implements `From<PositiveNaturalNumber> for NaturalNumber`.

#### 3. Integer Number (`src/math/integer_number/integer_number.rs`)
- `pub enum IntegerNumber<const BASE: u128 = 256> { Positive(PositiveNaturalNumber<BASE>), Negative(PositiveNaturalNumber<BASE>), Zero }`
- Implements `TryFrom` for signed/unsigned primitives (casts to target types, handles sign negation, checks for overflows).
- Implements infallible `From<NaturalNumber> for IntegerNumber`.
- Implements failable `TryFrom<IntegerNumber> for NaturalNumber` (fails on negative variant).

---

## Verification Plan

### Automated Tests
- Create unit tests in `src/math/integer_number/tests.rs` verifying:
  - Zero value cases.
  - Primitive failable conversions and bounds checking (overflow errors).
  - Conversions between `NaturalNumber`, `PositiveNaturalNumber`, and `IntegerNumber`.
- Verify tests pass using `cargo test`.
