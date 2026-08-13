# Remove Unary Base Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Remove unary base support from BaseDigit entirely.

**Architecture:** Modify `src/math/base_digit/mod.rs` to disallow BASE = 1, strip Unary cases from trait implementations, and remove Unary unit tests.

**Tech Stack:** Rust (stable)

## Global Constraints

- Disallow BASE = 1 (Unary) in `BaseDigit::new`.
- Clean up BASE = 1 match cases in arithmetic trait implementations.
- Delete unary-specific unit tests.

---

### Task 1: Clean Up mod.rs to Remove Unary Base Support

**Files:**
- Modify: `src/math/base_digit/mod.rs`

**Interfaces:**
- Consumes: None
- Produces: BaseDigit without Unary base support.

- [ ] **Step 1: Update constructor check**
  Change `BaseDigit::new` check to disallow `BASE == 1` and remove the `1` bounds check:
  ```rust
      pub fn new(value: u128) -> Result<Self, &'static str> {
          if BASE == 1 {
              return Err("Base cannot be 1");
          }
          if BASE > 1 && value >= BASE {
              return Err("Digit value too high for the base");
          }
  ```

- [ ] **Step 2: Remove BASE = 1 match arms from trait implementations**
  In `DigitAdd`, `DigitSub`, `DigitMul`, `DigitDivRem`, and `DigitFromWide` implementations, remove the `1 => { ... }` arms.

- [ ] **Step 3: Delete test_unary_arithmetic test**
  Remove the `test_unary_arithmetic` block from the unit tests module.

- [ ] **Step 4: Verify Compilation and Run Tests**
  Run `cargo test`.
