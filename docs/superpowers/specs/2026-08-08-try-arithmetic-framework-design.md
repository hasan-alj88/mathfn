# Design Spec: Fallible Arithmetic Traits and Subtraction Integration

Standardize the fallible operations framework by introducing custom standard arithmetic Try traits and placing the `Sub` and `TrySub` implementation skeletons for `NaturalNumber` using `IntegerNumber`.

## Goal
1. Standardize fallible math operations by defining `TryAdd`, `TrySub`, `TryMul`, `TryDiv`, and `TryRem` (and their `Assign` counterparts).
2. Integrate the currently unlinked `IntegerNunbers` module into the module tree so `IntegerNumber` is available.
3. Define the skeleton for subtraction on `NaturalNumber` where:
   - Standard subtraction operator `Sub` returns `IntegerNumber`.
   - Custom `TrySub` returns `Result<NaturalNumber, MathError>` (failing if the result is negative).

## Proposed Architecture

```
src/math/
├── mod.rs (registers IntegerNunbers)
├── IntegerNunbers/
│   ├── mod.rs (registers integer_numbers and arthimatic)
│   └── ...
├── operations/
│   ├── mod.rs (re-exports arithmetic traits)
│   └── arithmetic.rs (NEW: contains standard Try arithmetic traits)
└── NaturalNumbers/
    └── arthimitic/
        ├── mod.rs (registers subtraction)
        └── subtraction.rs (NEW: implements Sub and TrySub for NaturalNumber)
```

### 1. Defining Fallible Arithmetic Traits

#### [NEW] `src/math/operations/arithmetic.rs`
Contains `Try` variants of standard binary operations:
```rust
pub trait TryAdd<Rhs = Self> {
    type Output;
    type Error;
    fn try_add(self, rhs: Rhs) -> Result<Self::Output, Self::Error>;
}

pub trait TryAddAssign<Rhs = Self> {
    type Error;
    fn try_add_assign(&mut self, rhs: Rhs) -> Result<(), Self::Error>;
}

pub trait TrySub<Rhs = Self> {
    type Output;
    type Error;
    fn try_sub(self, rhs: Rhs) -> Result<Self::Output, Self::Error>;
}

pub trait TrySubAssign<Rhs = Self> {
    type Error;
    fn try_sub_assign(&mut self, rhs: Rhs) -> Result<(), Self::Error>;
}

pub trait TryMul<Rhs = Self> {
    type Output;
    type Error;
    fn try_mul(self, rhs: Rhs) -> Result<Self::Output, Self::Error>;
}

pub trait TryMulAssign<Rhs = Self> {
    type Error;
    fn try_mul_assign(&mut self, rhs: Rhs) -> Result<(), Self::Error>;
}

pub trait TryDiv<Rhs = Self> {
    type Output;
    type Error;
    fn try_div(self, rhs: Rhs) -> Result<Self::Output, Self::Error>;
}

pub trait TryDivAssign<Rhs = Self> {
    type Error;
    fn try_div_assign(&mut self, rhs: Rhs) -> Result<(), Self::Error>;
}

pub trait TryRem<Rhs = Self> {
    type Output;
    type Error;
    fn try_rem(self, rhs: Rhs) -> Result<Self::Output, Self::Error>;
}

pub trait TryRemAssign<Rhs = Self> {
    type Error;
    fn try_rem_assign(&mut self, rhs: Rhs) -> Result<(), Self::Error>;
}
```

Expose and re-export these in `src/math/operations/mod.rs`.

### 2. Linking IntegerNunbers Module

Update `src/math/mod.rs` to register:
```rust
#[allow(non_snake_case)]
pub mod IntegerNunbers;
```

Update `src/math/IntegerNunbers/mod.rs` to register:
```rust
pub mod integer_numbers;
pub mod arthimatic;
```

### 3. Implementing Subtraction Skeletons on NaturalNumber

#### [NEW] `src/math/NaturalNumbers/arthimitic/subtraction.rs`
Implement `Sub` (returning `IntegerNumber`), `TrySub` (returning `Result<NaturalNumber, MathError>`), and `TrySubAssign` (performing fallible in-place subtraction).

```rust
use crate::math::NaturalNumbers::NaturalNumber;
use crate::math::IntegerNunbers::integer_numbers::{IntegerNumber, Sign};
use crate::math::operations::{Sub, TrySub, TrySubAssign};
use crate::math::math_error::MathError;

impl Sub for NaturalNumber {
    type Output = IntegerNumber;

    fn sub(self, other: NaturalNumber) -> Self::Output {
        if self < other {
            // Underflow subtraction yields negative integer
            let diff_limbs = crate::math::NaturalNumbers::utils::arithmetic::add_slices(&other.limbs, &[]); // skeleton placeholder
            IntegerNumber {
                magnitude: NaturalNumber::new(diff_limbs),
                sign: Sign::Negative,
            }
        } else {
            // Normal subtraction yields positive integer
            let diff_limbs = crate::math::NaturalNumbers::utils::arithmetic::add_slices(&self.limbs, &[]); // skeleton placeholder
            IntegerNumber {
                magnitude: NaturalNumber::new(diff_limbs),
                sign: Sign::Positive,
            }
        }
    }
}

impl TrySub for NaturalNumber {
    type Output = NaturalNumber;
    type Error = MathError;

    fn try_sub(self, other: NaturalNumber) -> Result<Self::Output, Self::Error> {
        if self < other {
            Err(MathError::ResultNotInDomain {
                this_domain: "NaturalNumber".to_string(),
                result_domain: "IntegerNumber (negative)".to_string(),
            })
        } else {
            // Skeleton returns magnitude
            let diff_limbs = crate::math::NaturalNumbers::utils::arithmetic::add_slices(&self.limbs, &[]); // skeleton placeholder
            Ok(NaturalNumber::new(diff_limbs))
        }
    }
}

impl TrySubAssign for NaturalNumber {
    type Error = MathError;

    fn try_sub_assign(&mut self, other: NaturalNumber) -> Result<(), Self::Error> {
        if *self < other {
            Err(MathError::ResultNotInDomain {
                this_domain: "NaturalNumber".to_string(),
                result_domain: "IntegerNumber (negative)".to_string(),
            })
        } else {
            // Skeleton in-place mutation
            let diff_limbs = crate::math::NaturalNumbers::utils::arithmetic::add_slices(&self.limbs, &[]); // skeleton placeholder
            self.limbs = diff_limbs;
            Ok(())
        }
    }
}
```

Update `src/math/NaturalNumbers/arthimitic.rs` to include:
```rust
pub mod subtraction;
```

## Verification Plan
1. Compile the codebase to ensure all type paths are correctly linked.
2. Run `cargo test` to verify everything builds and passes.
