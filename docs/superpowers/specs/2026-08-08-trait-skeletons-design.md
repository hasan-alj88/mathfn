# Design Spec: Fallible & Standard Arithmetic Skeletons

Define trait skeletons with `todo!()` for all unimplemented unary and binary operations for both `NaturalNumber` and `IntegerNumber`. Enforce the trait separation rule by placing each implementation in its own separate file.

## Goal
1. Standardize and fully populate the trait skeleton surface area for `NaturalNumber` and `IntegerNumber`.
2. Strictly enforce the rule that **each trait implementation must reside in its own separate file** (e.g., separating `Add` and `AddAssign` into `add.rs` and `add_assign.rs` respectively).
3. Create placeholder structs `RationalNumber` and `RealNumber` to allow compiling division, square root, and log operations.

## Proposed Architecture

All operations for each domain will be split into individual files named after the trait (lowercase).

### 1. Placeholder Definitions
Define placeholder structures in `src/math/operations/placeholders.rs` and re-export them in `src/math/operations/mod.rs`:
```rust
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RationalNumber;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RealNumber;
```

### 2. NaturalNumber Module Structure (`src/math/NaturalNumbers/arthimitic/`)
The existing combined files (`addition.rs`, `multiplication.rs`, `subtraction.rs`, `exponentiation.rs`) will be split into separate files. Unimplemented operations will have skeletons containing `todo!()`.

#### Standard Binary Operations
- `add.rs` (migrated from `addition.rs`)
- `add_assign.rs` (migrated from `addition.rs`)
- `sub.rs` (migrated from `subtraction.rs`)
- `mul.rs` (migrated from `multiplication.rs`)
- `mul_assign.rs` (migrated from `multiplication.rs`)
- `pow.rs` (migrated from `exponentiation.rs`)
- `pow_assign.rs` (migrated from `exponentiation.rs`)
- `div.rs` (returns `RationalNumber`, calls `todo!()`)
- `rem.rs` (returns `NaturalNumber`, calls `todo!()`)
- `rem_assign.rs` (calls `todo!()`)

#### Try Binary Operations
- `try_add.rs` (returns `Result<NaturalNumber, MathError>`, calls `todo!()`)
- `try_add_assign.rs` (calls `todo!()`)
- `try_sub.rs` (migrated from `subtraction.rs`)
- `try_sub_assign.rs` (migrated from `subtraction.rs`)
- `try_mul.rs` (calls `todo!()`)
- `try_mul_assign.rs` (calls `todo!()`)
- `try_div.rs` (calls `todo!()`)
- `try_div_assign.rs` (calls `todo!()`)
- `try_rem.rs` (calls `todo!()`)
- `try_rem_assign.rs` (calls `todo!()`)
- `try_pow.rs` (calls `todo!()`)
- `try_pow_assign.rs` (calls `todo!()`)

#### Unary Operations
- `neg.rs` (returns `IntegerNumber`, calls `todo!()`)
- `double.rs` (returns `NaturalNumber`, calls `todo!()`)
- `double_assign.rs` (calls `todo!()`)
- `square.rs` (returns `NaturalNumber`, calls `todo!()`)
- `square_assign.rs` (calls `todo!()`)
- `sqrt.rs` (returns `RealNumber`, calls `todo!()`)
- `log.rs` (returns `RealNumber`, calls `todo!()`)
- `ln.rs` (returns `RealNumber`, calls `todo!()`)

#### Try Unary Operations
- `try_double.rs` (calls `todo!()`)
- `try_double_assign.rs` (calls `todo!()`)
- `try_square.rs` (calls `todo!()`)
- `try_square_assign.rs` (calls `todo!()`)
- `try_sqrt.rs` (calls `todo!()`)
- `try_log.rs` (calls `todo!()`)
- `try_ln.rs` (calls `todo!()`)

---

### 3. IntegerNumber Module Structure (`src/math/IntegerNunbers/arthimatic/`)
Similar to `NaturalNumber`, but for the `IntegerNumber` domain.

- `add.rs` (migrated from `addition.rs`)
- `add_assign.rs` (calls `todo!()`)
- `try_add.rs` (calls `todo!()`)
- `try_add_assign.rs` (calls `todo!()`)
- `sub.rs` (calls `todo!()`)
- `sub_assign.rs` (calls `todo!()`)
- `try_sub.rs` (calls `todo!()`)
- `try_sub_assign.rs` (calls `todo!()`)
- `mul.rs` (calls `todo!()`)
- `mul_assign.rs` (calls `todo!()`)
- `try_mul.rs` (calls `todo!()`)
- `try_mul_assign.rs` (calls `todo!()`)
- `div.rs` (returns `RationalNumber`, calls `todo!()`)
- `try_div.rs` (calls `todo!()`)
- `try_div_assign.rs` (calls `todo!()`)
- `rem.rs` (returns `IntegerNumber`, calls `todo!()`)
- `rem_assign.rs` (calls `todo!()`)
- `try_rem.rs` (calls `todo!()`)
- `try_rem_assign.rs` (calls `todo!()`)
- `pow.rs` (calls `todo!()`)
- `pow_assign.rs` (calls `todo!()`)
- `try_pow.rs` (calls `todo!()`)
- `try_pow_assign.rs` (calls `todo!()`)
- `neg.rs` (returns `IntegerNumber`, calls `todo!()`)
- `double.rs` (returns `IntegerNumber`, calls `todo!()`)
- `double_assign.rs` (calls `todo!()`)
- `try_double.rs` (calls `todo!()`)
- `try_double_assign.rs` (calls `todo!()`)
- `square.rs` (returns `IntegerNumber`, calls `todo!()`)
- `square_assign.rs` (calls `todo!()`)
- `try_square.rs` (calls `todo!()`)
- `try_square_assign.rs` (calls `todo!()`)
- `sqrt.rs` (returns `RealNumber`, calls `todo!()`)
- `try_sqrt.rs` (calls `todo!()`)
- `log.rs` (returns `RealNumber`, calls `todo!()`)
- `try_log.rs` (calls `todo!()`)
- `ln.rs` (returns `RealNumber`, calls `todo!()`)
- `try_ln.rs` (calls `todo!()`)

## Execution Strategy
To do this efficiently and without errors, we will write a scratch generation script `generate_skeletons.py` in the app data workspace, execute it to safely create/migrate all files and write the correct `mod.rs` registrations, and clean up the old combined files.

## Verification Plan
1. Run `cargo test` to ensure that all 50+ new modules compile cleanly and all existing tests pass.
