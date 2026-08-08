# Design Spec: Group Helper Functions by Functionality

Refactor the NaturalNumber implementation by extracting slice/limb utility and arithmetic helper functions from the multiplication module into their own files grouped by functionality.

## Goal
Improve module boundaries, keep individual source files focused, and make helper functions reusable across other NaturalNumber operations (like addition, comparison, or division in the future).

## Proposed Architecture

A new `utils` module will be introduced under the `NaturalNumbers` module hierarchy:

```
src/math/NaturalNumbers/
├── mod.rs (registers utils)
├── utils/
│   ├── mod.rs (exposes slice and arithmetic)
│   ├── slice.rs (slice utilities)
│   └── arithmetic.rs (slice/limb arithmetic primitives)
```

### Module exports

#### `src/math/NaturalNumbers/utils/slice.rs`
Contains slice manipulation and representation utility functions:
- `pub(crate) fn safe_split(slice: &[u128], m: usize) -> (&[u128], &[u128])`
- `pub(crate) fn pop_leading_zeros(slice: &mut Vec<u128>)`

#### `src/math/NaturalNumbers/utils/arithmetic.rs`
Contains low-level math arithmetic operations on slices/limbs:
- `pub(crate) fn add_slices(a: &[u128], b: &[u128]) -> Vec<u128>`
- `pub(crate) fn add_into(target: &mut [u128], addend: &[u128], offset: usize)`
- `pub(crate) fn sub_from(target: &mut [u128], sub: &[u128])`
- `pub(crate) fn mul_wide(a: u128, b: u128) -> (u128, u128)`

### Integration Details

1. **`src/math/NaturalNumbers/mod.rs`**:
   Add `pub(crate) mod utils;` to expose `utils` internal to the `math::NaturalNumbers` module hierarchy.
2. **`src/math/NaturalNumbers/arthimitic/multiplication.rs`**:
   - Remove these 6 helper functions from this file.
   - Add imports:
     ```rust
     use crate::math::NaturalNumbers::utils::slice::{safe_split, pop_leading_zeros};
     use crate::math::NaturalNumbers::utils::arithmetic::{add_slices, add_into, sub_from, mul_wide};
     ```
   - Fix compilation error inside `schoolbook` function by changing `vec![0; a.len() + b.len()]` to `vec![0u128; a.len() + b.len()]`.

## Verification Plan
1. Compile the codebase to ensure all module paths are correctly resolved.
2. Run `cargo test` to verify that all existing tests compile and pass.
