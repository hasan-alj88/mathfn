# Refactor BaseDigit Module Structure and Style Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Clean up `mod.rs` to only export modules and references, move implementation to `digit.rs`, and replace all `if-else` expressions with `match` blocks.

**Architecture:**
* Create `src/math/base_digit/digit.rs` and copy enum, implementation, traits implementation, and tests there.
* Refactor `digit.rs` to use `match` instead of any `if-else` block.
* Simplify `src/math/base_digit/mod.rs` to only contain exports.

**Tech Stack:** Rust (stable)

## Global Constraints

- `mod.rs` should only contain `pub mod traits;`, `pub mod digit;`, and re-exports.
- `digit.rs` must not use `if-else`. Use `match` instead. Single `if` blocks without `else` are permitted.

---

### Task 1: Refactor Code Structure and Replace if-else with match

**Files:**
- Create: [NEW] `src/math/base_digit/digit.rs`
- Modify: `src/math/base_digit/mod.rs`

- [ ] **Step 1: Create digit.rs and refactor if-else to match**
  Move all code from `src/math/base_digit/mod.rs` (except the module declarations and exports) to `src/math/base_digit/digit.rs`.
  Convert all `if-else` expressions to `match` expressions.

- [ ] **Step 2: Update mod.rs to only export module and references**
  Update `src/math/base_digit/mod.rs` to only export `traits` and `digit`.

- [ ] **Step 3: Verify Compilation and Run Tests**
  Run `cargo test`.

- [ ] **Step 4: Commit changes**
  ```bash
  git add src/math/base_digit/
  git commit -m "refactor: move BaseDigit implementation to digit.rs and use match instead of if-else"
  ```
