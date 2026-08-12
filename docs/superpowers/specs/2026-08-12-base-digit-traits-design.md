# Design Spec: Remove placeholders.rs and Add BaseDigit Traits

This design spec outlines the removal of the placeholder `RealNumber` implementation in `placeholders.rs` and defines the core digitwise arithmetic traits for `BaseDigit` implementation.

## Goal
1. Remove `src/math/operations/placeholders.rs` and its exports.
2. Define five digitwise arithmetic traits (`DigitAdd`, `DigitSub`, `DigitMul`, `DigitDivRem`, `DigitFromWide`) in `src/math/base_digit.rs`.

## Proposed Architecture

### 1. Removal of `placeholders.rs`
* Delete `src/math/operations/placeholders.rs`.
* Modify `src/math/operations/mod.rs` to remove the references to `placeholders` and `RealNumber`.

### 2. BaseDigit Traits (`src/math/base_digit.rs`)
Append the following traits to `src/math/base_digit.rs`:

```rust
/// Trait for digit-wise addition with carry.
pub trait DigitAdd<Rhs = Self> {
    type Output;
    /// Adds `self`, `other`, and a `carry_in`, returning `(sum, carry_out)`.
    fn add_digit(self, other: Rhs, carry_in: Self) -> (Self::Output, Self::Output);
}

/// Trait for digit-wise subtraction with borrow.
pub trait DigitSub<Rhs = Self> {
    type Output;
    /// Subtracts `other` and a `borrow_in` from `self`, returning `(diff, borrow_out)`.
    fn sub_digit(self, other: Rhs, borrow_in: Self) -> (Self::Output, Self::Output);
}

/// Trait for digit-wise multiplication with carry.
pub trait DigitMul<Rhs = Self> {
    type Output;
    /// Multiplies `self` by `other` and adds `carry_in`, returning `(low_digit, high_digit)`.
    fn mul_digit(self, other: Rhs, carry_in: Self) -> (Self::Output, Self::Output);
}

/// Trait for digit-wise division and remainder of a double-width digit by a single-width divisor.
pub trait DigitDivRem<Rhs = Self> {
    type Output;
    /// Divides a double-width digit `(high, low)` by `divisor`, returning `(quotient, remainder)`.
    fn div_rem_digit(high: Self, low: Self, divisor: Rhs) -> (Self::Output, Self::Output);
}

/// Trait for splitting a larger double-width value (or wide representation) into a quotient and remainder digit relative to the BASE.
pub trait DigitFromWide<Wide> {
    /// Divides `wide_value` by `BASE` and returns `(quotient_digit, remainder_digit)`.
    fn from_wide(wide_value: Wide) -> (Self, Self) where Self: Sized;
}
```

## Verification Plan
* Verify that the codebase builds successfully using `cargo check` after these removals and additions.
