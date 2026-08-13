# Design Specification: NaturalNumber and Arbitrary-Precision Arithmetic

## Goal Description
Implement a new type `NaturalNumber` representing non-negative arbitrary-precision integers generic over a base `BASE`.
`NaturalNumber` is represented as a sequence of `Digit<BASE>` in Least Significant Digit (LSD) first order.
This allows us to perform arbitrary-precision arithmetic.

We will implement:
- Struct definition, constructors, and conversions.
- Schoolbook addition (`nat_add_schoolbook`).
- Schoolbook multiplication (`nat_mul_schoolbook`).
- Karatsuba multiplication (`nat_mul_karatsuba`).
- Binary power / exponentiation by squaring (`nat_pow_binary`).
- Internal helper subtraction (`nat_sub_schoolbook`) to support Karatsuba.

## User Review Required
No major user review required as the scope has been aligned:
- The base type defaults to `BASE = 256` (so digits range `0..=255` fit in a `u8`).
- General subtraction and division are deferred to TODOs to be implemented alongside `IntegerNumbers` and `RationalNumbers` respectively.

## Proposed Changes

We will introduce a new module under `src/math/natural_number/`.

### Directory Layout
- `src/math/natural_number/mod.rs` - Main struct, constructors, normalization, and module declaration.
- `src/math/natural_number/addition.rs` - Schoolbook addition.
- `src/math/natural_number/multiplication.rs` - Schoolbook and Karatsuba multiplication, subtraction helper.
- `src/math/natural_number/power.rs` - Exponentiation by squaring, `div_by_2` helper.
- `src/math/natural_number/tests.rs` - Unit tests verifying correctness of arithmetic operations.

---

### Component Details

#### 1. Struct Definition & Basic Methods (`src/math/natural_number/mod.rs`)
- `digits: Vec<Digit<BASE>>` in LSD-first order.
- Normalization invariant: `digits` has no trailing zero values. The number zero is represented by an empty vector `[]`.
- Public functions:
  - `pub fn new(digits: Vec<Digit<BASE>>) -> Self`
  - `pub fn from_u128(value: u128) -> Result<Self, MathError>`
  - `pub fn to_u128(&self) -> Result<u128, MathError>`
  - `pub fn is_zero(&self) -> bool`
  - `pub fn digits(&self) -> &[Digit<BASE>]`

#### 2. Addition (`src/math/natural_number/addition.rs`)
- `pub fn nat_add_schoolbook<const BASE: u128>(a: &NaturalNumber<BASE>, b: &NaturalNumber<BASE>) -> Result<NaturalNumber<BASE>, MathError>`
- Implements schoolbook digit addition with carry.

#### 3. Multiplication (`src/math/natural_number/multiplication.rs`)
- Helper functions:
  - `add_assign_digit<const BASE: u128>(digits: &mut Vec<Digit<BASE>>, index: usize, addend: Digit<BASE>) -> Result<(), MathError>`
  - `shift_left<const BASE: u128>(num: &NaturalNumber<BASE>, m: usize) -> NaturalNumber<BASE>`
  - `split_at<const BASE: u128>(num: &NaturalNumber<BASE>, m: usize) -> (NaturalNumber<BASE>, NaturalNumber<BASE>)`
- Subtraction helper (assuming $a \ge b$):
  - `pub fn nat_sub_schoolbook<const BASE: u128>(a: &NaturalNumber<BASE>, b: &NaturalNumber<BASE>) -> Result<NaturalNumber<BASE>, MathError>`
- Multiplication functions:
  - `pub fn nat_mul_schoolbook<const BASE: u128>(a: &NaturalNumber<BASE>, b: &NaturalNumber<BASE>) -> Result<NaturalNumber<BASE>, MathError>`
  - `pub fn nat_mul_karatsuba<const BASE: u128>(a: &NaturalNumber<BASE>, b: &NaturalNumber<BASE>) -> Result<NaturalNumber<BASE>, MathError>`

#### 4. Exponentiation (`src/math/natural_number/power.rs`)
- Helper `div_by_2` to perform division by 2 on a `NaturalNumber` and return the remainder (0 or 1).
- `pub fn nat_pow_binary<const BASE: u128>(base: &NaturalNumber<BASE>, exponent: &NaturalNumber<BASE>) -> Result<NaturalNumber<BASE>, MathError>`

---

## Verification Plan

### Automated Tests
- Create unit tests in `src/math/natural_number/tests.rs` for:
  - Small number operations (matching primitive arithmetic).
  - Normalization invariants.
  - Addition carrying.
  - Multiplication (Schoolbook vs Karatsuba consistency).
  - Exponentiation with larger values.
- Verify tests pass using `cargo test`.
