# Design Spec: Sign Relocation, PositiveNaturalNumbers, and RationalNumbers

Relocate the Sign enum to a universal location, introduce the PositiveNaturalNumber struct (representing natural numbers shifted by +1), and implement the RationalNumber struct containing a numerator, positive denominator, and sign.

## Goal
1. Relocate the `Sign` enum to a universal top-level location `src/math/sign.rs` so it can be reused by all number types.
2. Define `PositiveNaturalNumber` (in `src/math/PositiveNaturalNumbers/`) to mathematically represent $N \ge 1$ while storing $N - 1$ internally using the natural number implementation (preventing denominator of zero by design).
3. Define `RationalNumber` (in `src/math/RationalNumbers/`) with a natural numerator, positive natural denominator, and universal sign.

## Proposed Architecture

```
src/math/
├── mod.rs (registers sign, PositiveNaturalNumbers, and RationalNumbers)
├── sign.rs (NEW: contains universal Sign enum)
├── operations/
│   ├── mod.rs (re-exports RationalNumber from RationalNumbers module)
│   └── placeholders.rs (removes RationalNumber placeholder struct)
├── PositiveNaturalNumbers/ (NEW)
│   ├── mod.rs (exposes positive_natural_numbers)
│   └── positive_natural_numbers.rs (contains PositiveNaturalNumber struct)
└── RationalNumbers/ (NEW)
    ├── mod.rs (exposes rational_numbers)
    └── rational_numbers.rs (contains RationalNumber struct)
```

### 1. Universal Sign Enum

#### `src/math/sign.rs`
```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sign {
    Positive,
    Negative,
}
```

### 2. PositiveNaturalNumber Struct

#### `src/math/PositiveNaturalNumbers/positive_natural_numbers.rs`
```rust
use crate::math::NaturalNumbers::NaturalNumber;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PositiveNaturalNumber {
    // The value stored internally is (mathematical value - 1)
    pub value_minus_one: NaturalNumber,
}

impl PositiveNaturalNumber {
    pub fn new(value_minus_one: NaturalNumber) -> Self {
        Self { value_minus_one }
    }

    /// Creates a PositiveNaturalNumber from a standard NaturalNumber.
    /// Fails if the value is 0 (i.e. empty limbs or all zero limbs).
    pub fn from_value(value: NaturalNumber) -> Result<Self, String> {
        if value.limbs.is_empty() || value.limbs.iter().all(|&x| x == 0) {
            Err("Value must be greater than zero".to_string())
        } else {
            let one = NaturalNumber::from(1u128);
            let value_minus_one = crate::math::operations::TrySub::try_sub(value, one)
                .map_err(|_| "Failed to decrement natural number".to_string())?;
            Ok(Self { value_minus_one })
        }
    }

    /// Returns the actual mathematical NaturalNumber (value_minus_one + 1)
    pub fn to_value(&self) -> NaturalNumber {
        let one = NaturalNumber::from(1u128);
        crate::math::operations::Add::add(self.value_minus_one.clone(), one)
    }
}
```

### 3. RationalNumber Struct

#### `src/math/RationalNumbers/rational_numbers.rs`
```rust
use crate::math::NaturalNumbers::NaturalNumber;
use crate::math::PositiveNaturalNumbers::PositiveNaturalNumber;
use crate::math::Sign;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RationalNumber {
    pub numerator: NaturalNumber,
    pub denominator: PositiveNaturalNumber,
    pub sign: Sign,
}

impl RationalNumber {
    pub fn new(numerator: NaturalNumber, denominator: PositiveNaturalNumber, sign: Sign) -> Self {
        Self { numerator, denominator, sign }
    }
}
```

## Integration Details
1. **`src/math/mod.rs`**: Register `sign`, `PositiveNaturalNumbers`, and `RationalNumbers` as public submodules, and re-export `Sign`.
2. **`src/math/IntegerNunbers/integer_numbers.rs`**: Remove `Sign` enum and import `crate::math::Sign`.
3. **`src/math/IntegerNunbers/mod.rs`**: Re-export `Sign` from `crate::math::Sign` if any consumers use it from there.
4. **`src/math/operations/placeholders.rs`**: Remove `RationalNumber` placeholder struct.
5. **`src/math/operations/mod.rs`**: Re-export `RationalNumber` from the new module instead.

## Verification Plan
1. Ensure the project compiles successfully.
2. Run `cargo test` to verify all tests pass.
