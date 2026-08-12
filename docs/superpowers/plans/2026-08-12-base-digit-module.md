# BaseDigit Module Folder Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Structure BaseDigit as an enum-based module folder, support 2^128 as the default base, and clean up the old file.

**Architecture:** Create `src/math/base_digit/traits.rs` and `src/math/base_digit/mod.rs`, delete `src/math/base_digit.rs`, and test compilation and logic.

**Tech Stack:** Rust (stable)

## Global Constraints

- Re-organize BaseDigit as a folder `src/math/base_digit/` containing `mod.rs` and `traits.rs`.
- Define BaseDigit as a generic enum with `BASE = 0` default (representing 2^128).
- Remove the old file src/math/base_digit.rs.

---

### Task 1: Create the BaseDigit Module Directory and Files

**Files:**
- Create: `src/math/base_digit/traits.rs`
- Create: `src/math/base_digit/mod.rs`
- Delete: `src/math/base_digit.rs`

**Interfaces:**
- Consumes: None
- Produces: `BaseDigit<const BASE: u128>` enum and digitwise operation traits.

- [ ] **Step 1: Create src/math/base_digit/traits.rs**
  Write the digit-wise operation traits into `src/math/base_digit/traits.rs`:
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

- [ ] **Step 2: Create src/math/base_digit/mod.rs**
  Write the `BaseDigit` enum, constructors, value method, and unit tests into `src/math/base_digit/mod.rs`:
  ```rust
  pub mod traits;

  pub use traits::{DigitAdd, DigitSub, DigitMul, DigitDivRem, DigitFromWide};

  #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
  pub enum BaseDigit<const BASE: u128 = 0> {
      Binary(u8),            // Base 2
      Quaternary(u8),        // Base 4
      Octal(u8),             // Base 8
      Decimal(u8),           // Base 10
      Dezonal(u8),           // Base 12
      Hexadecimal(u8),       // Base 16
      Octet(u8),             // Base 256
      Doublet(u16),          // Base 65536
      Quadlet(u32),          // Base 2^32
      Octlet(u64),           // Base 2^64
      DoubleOctlet(u128),    // Base 2^128 (default)
      Other(u128),           // Any other base
  }

  impl<const BASE: u128> BaseDigit<BASE> {
      /// Creates a new BaseDigit. Returns Err if value >= BASE (for non-2^128 bases).
      pub fn new(value: u128) -> Result<Self, &'static str> {
          if BASE == 1 {
              return Err("Base cannot be 1");
          }
          if BASE != 0 && value >= BASE {
              return Err("Digit value too high for the base");
          }

          Ok(match BASE {
              2 => Self::Binary(value as u8),
              4 => Self::Quaternary(value as u8),
              8 => Self::Octal(value as u8),
              10 => Self::Decimal(value as u8),
              12 => Self::Dezonal(value as u8),
              16 => Self::Hexadecimal(value as u8),
              256 => Self::Octet(value as u8),
              65536 => Self::Doublet(value as u16),
              4294967296 => Self::Quadlet(value as u32),
              18446744073709551616 => Self::Octlet(value as u64),
              0 => Self::DoubleOctlet(value),
              _ => Self::Other(value),
          })
      }

      /// Returns the inner value of the digit as a u128.
      pub fn value(self) -> u128 {
          match self {
              Self::Binary(v) => v as u128,
              Self::Quaternary(v) => v as u128,
              Self::Octal(v) => v as u128,
              Self::Decimal(v) => v as u128,
              Self::Dezonal(v) => v as u128,
              Self::Hexadecimal(v) => v as u128,
              Self::Octet(v) => v as u128,
              Self::Doublet(v) => v as u128,
              Self::Quadlet(v) => v as u128,
              Self::Octlet(v) => v as u128,
              Self::DoubleOctlet(v) => v,
              Self::Other(v) => v,
          }
      }
  }

  #[cfg(test)]
  mod tests {
      use super::*;

      #[test]
      fn test_base_digit_creation() {
          // Default base (BASE = 0, represents 2^128)
          let default_digit = BaseDigit::new(u128::MAX).unwrap();
          assert_eq!(default_digit.value(), u128::MAX);
          assert!(matches!(default_digit, BaseDigit::DoubleOctlet(_)));

          // Base 10 (Decimal)
          let dec_digit = BaseDigit::<10>::new(9).unwrap();
          assert_eq!(dec_digit.value(), 9);
          assert!(matches!(dec_digit, BaseDigit::Decimal(_)));
          assert!(BaseDigit::<10>::new(10).is_err());

          // Base 12 (Dezonal)
          let dez_digit = BaseDigit::<12>::new(11).unwrap();
          assert_eq!(dez_digit.value(), 11);
          assert!(matches!(dez_digit, BaseDigit::Dezonal(_)));
          assert!(BaseDigit::<12>::new(12).is_err());

          // Other custom base (e.g. Base 50)
          let custom_digit = BaseDigit::<50>::new(49).unwrap();
          assert_eq!(custom_digit.value(), 49);
          assert!(matches!(custom_digit, BaseDigit::Other(_)));
          assert!(BaseDigit::<50>::new(50).is_err());
      }
  }
  ```

- [ ] **Step 3: Delete old src/math/base_digit.rs**
  Run: `rm src/math/base_digit.rs`

- [ ] **Step 4: Verify Compilation and Run Tests**
  Run: `cargo test`
  Expected: Success (the new base digit tests pass)

- [ ] **Step 5: Commit changes**
  ```bash
  git add src/math/base_digit/
  git rm src/math/base_digit.rs
  git commit -m "feat: restructure BaseDigit into its own module folder with enum storage"
  ```
