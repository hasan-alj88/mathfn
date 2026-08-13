# Design Specification: RationalNumber and Euclidean Division

## Goal Description
Implement:
1. `Ord` and `PartialOrd` on `NaturalNumber` to compare arbitrary-precision integers.
2. Euclidean division (`nat_div_rem_schoolbook`) and GCD (`nat_gcd`) on `NaturalNumber`.
3. `RationalNumber` representing rational fractions ($\pm a/b$) reduced to lowest terms using GCD. Zero is normalized to $+ 1/1$.

## Proposed Changes

We will introduce a new module `src/math/rational_number/` and update `src/math/natural_number/`.

### Directory Layout
- `src/math/natural_number/natural_number.rs` - Implement `Ord` and `PartialOrd`.
- `src/math/natural_number/mod.rs` - Register `division` submodule.
- `src/math/natural_number/division.rs` - Implement division/modulo and GCD.
- `src/math/mod.rs` - Register `rational_number` module.
- `src/math/rational_number/mod.rs` - Module registration and exports.
- `src/math/rational_number/rational_number.rs` - `RationalNumber` struct definition and conversions.
- `src/math/rational_number/tests.rs` - Unit tests for division, GCD, and `RationalNumber`.

---

### Component Details

#### 1. Comparison on `NaturalNumber` (`src/math/natural_number/natural_number.rs`)
Implement `Ord` and `PartialOrd` by:
1. Comparing vector lengths (longer vector is larger).
2. For equal lengths, performing a reverse-lexicographical comparison on digits (most significant to least significant).

#### 2. Division and GCD (`src/math/natural_number/division.rs`)
- `pub fn nat_div_rem_schoolbook` using binary double-and-add restoring division.
- `pub fn nat_gcd` using the standard Euclidean algorithm.

#### 3. Rational Number (`src/math/rational_number/rational_number.rs`)
- `pub struct RationalNumber<const BASE: u128 = 256>` with fields:
  - `sign: Sign`
  - `numerator: PositiveNaturalNumber<BASE>`
  - `denominator: PositiveNaturalNumber<BASE>`
- Normalization in `new`:
  - If `sign == Sign::Zero`: set numerator and denominator to 1.
  - Else: compute $g = \gcd(\text{numerator}, \text{denominator})$. If $g > 1$, divide both by $g$.

---

## Verification Plan

### Automated Tests
- Create unit tests verifying:
  - division by zero error.
  - Correct division quotients and remainders.
  - GCD of small and large numbers.
  - Rational number creation, GCD simplification, and conversions.
- Verify tests pass using `cargo test`.
