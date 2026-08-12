# BaseDigit Traits Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Remove placeholders.rs and add digit-wise operation traits to base_digit.rs.

**Architecture:** Delete `placeholders.rs`, remove its re-exports, and append trait definitions for digitwise arithmetic (`DigitAdd`, `DigitSub`, `DigitMul`, `DigitDivRem`, `DigitFromWide`) to `base_digit.rs`.

**Tech Stack:** Rust (stable)

## Global Constraints

- Remove placeholders.rs and its exports from src/math/operations/mod.rs.
- Add five digitwise traits (DigitAdd, DigitSub, DigitMul, DigitDivRem, DigitFromWide) in src/math/base_digit.rs.

---

### Task 1: Delete placeholders.rs and Clean Up Operations Mod

**Files:**
- Delete: `src/math/operations/placeholders.rs`
- Modify: `src/math/operations/mod.rs`

**Interfaces:**
- Consumes: None
- Produces: None

- [ ] **Step 1: Delete src/math/operations/placeholders.rs**
  Run command:
  `rm src/math/operations/placeholders.rs`

- [ ] **Step 2: Modify src/math/operations/mod.rs**
  Remove references to placeholders module and RealNumber.
  Change `src/math/operations/mod.rs` to:
  ```rust
  // Re-export standard library operations traits
  #[allow(unused_imports)]
  pub use std::ops::{
      Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Deref,
      DerefMut, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Not, Rem, RemAssign, Shl,
      ShlAssign, Shr, ShrAssign, Sub, SubAssign,
  };

  pub mod basic;
  pub mod logarithm;
  pub mod power;
  pub mod arithmetic;

  pub use basic::{Double, DoubleAssign, Abs, TryDouble, TryDoubleAssign, TryAbs};
  pub use logarithm::{Log, Ln, TryLog, TryLn};
  pub use power::{
      Pow, PowAssign, Square, SquareAssign, Sqrt, TryPow, TryPowAssign, TrySquare, TrySquareAssign,
      TrySqrt,
  };
  pub use arithmetic::{
      TryAdd, TryAddAssign, TrySub, TrySubAssign, TryMul, TryMulAssign, TryDiv, TryDivAssign,
      TryRem, TryRemAssign,
  };
  ```

- [ ] **Step 3: Commit changes**
  ```bash
  git add src/math/operations/mod.rs
  git rm src/math/operations/placeholders.rs
  git commit -m "refactor: remove placeholders.rs and its exports"
  ```

---

### Task 2: Add Digitwise Traits to base_digit.rs

**Files:**
- Modify: `src/math/base_digit.rs`

**Interfaces:**
- Consumes: None
- Produces: `DigitAdd`, `DigitSub`, `DigitMul`, `DigitDivRem`, `DigitFromWide` traits

- [ ] **Step 1: Append traits to src/math/base_digit.rs**
  Modify `src/math/base_digit.rs` to add the digitwise traits.
  The file should contain:
  ```rust
  #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
  pub struct BaseDigit<const BASE: u128 = 256>;

  impl<const BASE: u128> BaseDigit<BASE> {
      // Add your fields, storage enum, and methods here
  }

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

- [ ] **Step 2: Run verification**
  Run: `cargo check`
  Expected: Success

- [ ] **Step 3: Commit and verify**
  ```bash
  git add src/math/base_digit.rs
  git commit -m "feat: add digit-wise operation traits to base_digit.rs"
  ```
