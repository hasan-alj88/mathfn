# BaseDigit Conversion Methods Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement `convert_overflow` and `into_digit` methods for `BaseDigit<BASE>` to support base conversion with overflow handling.

**Architecture:** Add `convert_overflow` and `into_digit` inside `impl<const BASE: u128> BaseDigit<BASE>` in `src/math/base_digit/mod.rs` and verify with unit tests.

**Tech Stack:** Rust (stable)

## Global Constraints

- Implement `into_digit<const NEW_BASE: u128>(self) -> Result<BaseDigit<NEW_BASE>, &'static str>`.
- Implement `convert_overflow<const NEW_BASE: u128>(self) -> Result<Vec<BaseDigit<NEW_BASE>>, &'static str>`.
- Return LSB-first order vector in `convert_overflow`.

---

### Task 1: Add Conversion Methods and Unit Tests

**Files:**
- Modify: `src/math/base_digit/mod.rs`

**Interfaces:**
- Consumes: None
- Produces: `into_digit` and `convert_overflow` methods.

- [ ] **Step 1: Implement into_digit and convert_overflow methods**
  Add the methods to `impl<const BASE: u128> BaseDigit<BASE>` in `src/math/base_digit/mod.rs`:

  ```rust
      /// Tries to convert this digit into a digit of the target base. Returns Err if value >= NEW_BASE.
      pub fn into_digit<const NEW_BASE: u128>(self) -> Result<BaseDigit<NEW_BASE>, &'static str> {
          BaseDigit::<NEW_BASE>::new(self.value())
      }

      /// Converts this digit's value to a vector of digits in the target base, LSB first.
      /// Ensures a minimum of two elements (digit + overflow/carry) are returned when value fits in a single digit.
      pub fn convert_overflow<const NEW_BASE: u128>(self) -> Result<Vec<BaseDigit<NEW_BASE>>, &'static str> {
          if NEW_BASE == 1 {
              return Err("Base cannot be 1");
          }
          let mut val = self.value();
          let mut result = Vec::new();

          if NEW_BASE == 0 {
              result.push(BaseDigit::<0>::new(val)?);
              result.push(BaseDigit::<0>::new(0)?);
              return Ok(result);
          }

          result.push(BaseDigit::<NEW_BASE>::new(val % NEW_BASE)?);
          val /= NEW_BASE;

          while val > 0 {
              result.push(BaseDigit::<NEW_BASE>::new(val % NEW_BASE)?);
              val /= NEW_BASE;
          }

          if result.len() == 1 {
              result.push(BaseDigit::<NEW_BASE>::new(0)?);
          }

          Ok(result)
      }
  ```

- [ ] **Step 2: Add unit tests**
  Add `test_base_digit_conversion` unit test inside the `tests` module verifying:
  * Converting `12` in base 10/16 to base 16 returns `[12_16, 0_16]`.
  * Converting `10` in base 10 to base 2 returns `[0_2, 1_2, 0_2, 1_2]`.
  * Converting `9` in base 10 to base 16 using `into_digit` returns `Ok(9_16)`.
  * Converting `18` in base 20 to base 10 using `into_digit` returns `Err("Digit value too high for the base")`.

- [ ] **Step 3: Verify Compilation and Run Tests**
  Run `cargo test`.

- [ ] **Step 4: Commit changes**
  ```bash
  git add src/math/base_digit/mod.rs
  git commit -m "feat: implement into_digit and convert_overflow methods for BaseDigit"
  ```
