# Design Spec: Remove Old Number Modules and Create Minimal BaseDigit Skeleton

This spec outlines the removal of the outdated `NaturalNumbers`, `IntegerNunbers`, `PositiveNaturalNumbers`, and `RationalNumbers` modules, and the creation of a minimal `BaseDigit` skeleton in their place to allow the user to implement it from scratch.

## Goal
1. Remove all old number modules completely.
2. Clean up module exports in `src/math/mod.rs` and `src/math/operations/mod.rs`.
3. Create a minimal `BaseDigit` struct skeleton with a const generic parameter `BASE` in `src/math/base_digit.rs`.

## Proposed Architecture

### 1. New Skeleton file (`src/math/base_digit.rs`)
```rust
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct BaseDigit<const BASE: u128 = 256>;

impl<const BASE: u128> BaseDigit<BASE> {
    // Implement storage enum, constructors, and display traits here.
}
```

### 2. Cleanup of Module Definitions (`src/math/mod.rs`)
Modify the file to remove module imports of:
* `NaturalNumbers`
* `IntegerNunbers`
* `PositiveNaturalNumbers`
* `RationalNumbers`

And add:
* `pub mod base_digit;`

### 3. Cleanup of Operation Re-exports (`src/math/operations/mod.rs`)
Modify the file to remove:
* `pub use crate::math::RationalNumbers::RationalNumber;`

## Verification Plan

### Manual Verification
* Ensure that the remaining files in the project compile successfully after cleanup using `cargo check`.
