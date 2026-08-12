# BaseDigit Skeleton Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Remove the outdated number modules and create a minimal BaseDigit skeleton.

**Architecture:** Delete old modules, update module declaration files, and write the minimal BaseDigit struct.

**Tech Stack:** Rust (stable)

## Global Constraints

- Remove NaturalNumbers, IntegerNunbers, PositiveNaturalNumbers, and RationalNumbers modules.
- Clean up module exports in src/math/mod.rs and src/math/operations/mod.rs.
- Create a minimal BaseDigit struct skeleton with a const generic parameter BASE in src/math/base_digit.rs.

---

### Task 1: Delete Old Modules and Clean Up Exports

**Files:**
- Delete: `src/math/NaturalNumbers` (directory)
- Delete: `src/math/IntegerNunbers` (directory)
- Delete: `src/math/PositiveNaturalNumbers` (directory)
- Delete: `src/math/RationalNumbers` (directory)
- Modify: `src/math/mod.rs`
- Modify: `src/math/operations/mod.rs`

**Interfaces:**
- Consumes: None
- Produces: None

- [ ] **Step 1: Delete old module directories**
  Run commands to remove the four old directories:
  * `rm -rf src/math/NaturalNumbers`
  * `rm -rf src/math/IntegerNunbers`
  * `rm -rf src/math/PositiveNaturalNumbers`
  * `rm -rf src/math/RationalNumbers`

- [ ] **Step 2: Modify src/math/mod.rs**
  Remove the module declarations for the deleted modules and declare the new `base_digit` module.
  Change `src/math/mod.rs` to:
  ```rust
  pub mod base_digit;
  pub mod group_theroy;
  pub mod math_error;
  pub mod operations;
  pub mod sign;

  pub use sign::Sign;
  ```

- [ ] **Step 3: Modify src/math/operations/mod.rs**
  Remove the line re-exporting `RationalNumber`.
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
  pub mod placeholders;

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
  pub use placeholders::RealNumber;
  ```

- [ ] **Step 4: Commit changes**
  ```bash
  git add src/math/mod.rs src/math/operations/mod.rs
  git commit -m "refactor: remove old number modules and clean up references"
  ```

---

### Task 2: Create BaseDigit Skeleton

**Files:**
- Create: `src/math/base_digit.rs`

**Interfaces:**
- Consumes: None
- Produces: `BaseDigit<const BASE: u128>` struct

- [ ] **Step 1: Create the minimal base_digit.rs skeleton**
  Write the following content to `src/math/base_digit.rs`:
  ```rust
  #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
  pub struct BaseDigit<const BASE: u128 = 256>;

  impl<const BASE: u128> BaseDigit<BASE> {
      // Add your fields, storage enum, and methods here
  }
  ```

- [ ] **Step 2: Run verification**
  Run: `cargo check`
  Expected: Success (the code compiles successfully)

- [ ] **Step 3: Commit and verify**
  ```bash
  git add src/math/base_digit.rs
  git commit -m "feat: add minimal BaseDigit skeleton"
  ```
