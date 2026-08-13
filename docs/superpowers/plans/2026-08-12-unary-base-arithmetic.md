# BaseDigit Arithmetic and Unary Base 1 Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement digitwise arithmetic traits for `BaseDigit<BASE>` supporting all bases, and support Unary Base 1 where `1` represents the valid tally mark and `0` represents the invalid/empty state.

**Architecture:** Modify `src/math/base_digit/mod.rs` to remove the `Unary` enum variant (using `Other` for BASE = 1), treat `0` as the empty/invalid state for BASE = 1, and implement all 5 arithmetic traits.

**Tech Stack:** Rust (stable)

## Global Constraints

- Do not use a separate Unary enum variant; BASE = 1 maps to `Other`.
- For BASE = 1, `1` is the only valid tally mark and `0` is the empty/invalid state.
- Implement DigitAdd, DigitSub, DigitMul, DigitDivRem, and DigitFromWide traits for BaseDigit.

---

### Task 1: Update BaseDigit Enum and Constructors for BASE = 1

**Files:**
- Modify: `src/math/base_digit/mod.rs`

**Interfaces:**
- Consumes: None
- Produces: `BaseDigit<const BASE: u128>` enum and constructors.

- [ ] **Step 1: Remove Unary variant and adjust constructors**
  Modify `src/math/base_digit/mod.rs` to:
  * Remove `Unary` variant from the enum.
  * Update `new` so that for `BASE == 1`, only `0` and `1` are allowed (`0` is the empty/invalid state, `1` is the tally). For other bases, keep the standard bounds check.
  * Update `value` to reflect the removed variant.

  ```rust
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
      Other(u128),           // Any other base (including Unary Base 1)
  }
  ```

  And the constructor logic:
  ```rust
      pub fn new(value: u128) -> Result<Self, &'static str> {
          if BASE == 1 && value > 1 {
              return Err("Digit value too high for unary base");
          }
          if BASE > 1 && value >= BASE {
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
  ```

---

### Task 2: Implement Helper Functions and Arithmetic Traits

**Files:**
- Modify: `src/math/base_digit/mod.rs`

**Interfaces:**
- Consumes: None
- Produces: `DigitAdd`, `DigitSub`, `DigitMul`, `DigitDivRem`, `DigitFromWide` implementations.

- [ ] **Step 1: Write helper functions for u128 overflowing arithmetic**
  Add `mul_u128_add_u128` and `div_rem_u256_by_u128` inside `impl<const BASE: u128> BaseDigit<BASE>`.

- [ ] **Step 2: Implement traits**
  Write the trait implementations in `src/math/base_digit/mod.rs` with the specified `BASE == 1` behavior:
  * For addition: `sum_val = if total_tally >= 1 { 1 } else { 0 }`, `carry_out_val = if total_tally >= 2 { 1 } else { 0 }`.
  * For subtraction: `diff_val = if net_tally == 1 { 1 } else { 0 }`, `borrow_out_val = if net_tally < 0 { 1 } else { 0 }`.
  * For multiplication: `low = if total_tally >= 1 { 1 } else { 0 }`, `high = if total_tally >= 2 { 1 } else { 0 }`.
  * For division: `q = low`, `r = 0`.
  * For from_wide: `q = if wide_value > 0 { 1 } else { 0 }`, `r = 0`.

- [ ] **Step 3: Add unit tests**
  Verify:
  * Unary addition: `1 + 1 = 1 overflow 1`, `1 - 1 = 0`, `1 + 0 = 1`.
  * Standard Base 10 arithmetic.
  * Base 2^128 overflowing arithmetic.

- [ ] **Step 4: Verify Compilation and Run Tests**
  Run `cargo test`.
