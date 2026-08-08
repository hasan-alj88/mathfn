# Design Spec: Modular Math Operations and Try Traits

Group universal math traits into logical submodules under a new `operations` directory, and introduce `Try` variants of all math operation traits to support fallible domain/codomain transitions.

## Goal
1. Restructure the flat `operations.rs` file into a modular folder structure: `src/math/operations/`.
2. Group traits into related submodules: `basic`, `power`, and `logarithm`.
3. Introduce `Try` traits (e.g., `TryPow`, `TrySqrt`, `TryLog`) that return `Result<Output, Error>` to represent operations that can fail when the result falls outside the target domain or when the operation is undefined.

## Proposed Architecture

A new modular folder structure:
```
src/math/
├── operations/
│   ├── mod.rs (re-exports standard library operations + our custom traits)
│   ├── basic.rs (Double, DoubleAssign, Abs, and their Try equivalents)
│   ├── power.rs (Pow, PowAssign, Square, SquareAssign, Sqrt, and their Try equivalents)
│   └── logarithm.rs (Log, Ln, and their Try equivalents)
```

### Module definitions and Traits

#### `src/math/operations/basic.rs`
```rust
// Standard traits
pub trait Double {
    type Output;
    fn double(self) -> Self::Output;
}

pub trait DoubleAssign {
    fn double_assign(&mut self);
}

pub trait Abs {
    type Output;
    fn abs(self) -> Self::Output;
}

// Try traits
pub trait TryDouble {
    type Output;
    type Error;
    fn try_double(self) -> Result<Self::Output, Self::Error>;
}

pub trait TryDoubleAssign {
    type Error;
    fn try_double_assign(&mut self) -> Result<(), Self::Error>;
}

pub trait TryAbs {
    type Output;
    type Error;
    fn try_abs(self) -> Result<Self::Output, Self::Error>;
}
```

#### `src/math/operations/power.rs`
```rust
// Standard traits
pub trait Pow<Rhs = Self> {
    type Output;
    fn pow(self, rhs: Rhs) -> Self::Output;
}

pub trait PowAssign<Rhs = Self> {
    fn pow_assign(&mut self, rhs: Rhs);
}

pub trait Square {
    type Output;
    fn square(self) -> Self::Output;
}

pub trait SquareAssign {
    fn square_assign(&mut self);
}

pub trait Sqrt {
    type Output;
    fn sqrt(self) -> Self::Output;
}

// Try traits
pub trait TryPow<Rhs = Self> {
    type Output;
    type Error;
    fn try_pow(self, rhs: Rhs) -> Result<Self::Output, Self::Error>;
}

pub trait TryPowAssign<Rhs = Self> {
    type Error;
    fn try_pow_assign(&mut self, rhs: Rhs) -> Result<(), Self::Error>;
}

pub trait TrySquare {
    type Output;
    type Error;
    fn try_square(self) -> Result<Self::Output, Self::Error>;
}

pub trait TrySquareAssign {
    type Error;
    fn try_square_assign(&mut self) -> Result<(), Self::Error>;
}

pub trait TrySqrt {
    type Output;
    type Error;
    fn try_sqrt(self) -> Result<Self::Output, Self::Error>;
}
```

#### `src/math/operations/logarithm.rs`
```rust
// Standard traits
pub trait Log<Base = Self> {
    type Output;
    fn log(self, base: Base) -> Self::Output;
}

pub trait Ln {
    type Output;
    fn ln(self) -> Self::Output;
}

// Try traits
pub trait TryLog<Base = Self> {
    type Output;
    type Error;
    fn try_log(self, base: Base) -> Result<Self::Output, Self::Error>;
}

pub trait TryLn {
    type Output;
    type Error;
    fn try_ln(self) -> Result<Self::Output, Self::Error>;
}
```

#### `src/math/operations/mod.rs`
- Expose the submodules: `pub mod basic;`, `pub mod power;`, `pub mod logarithm;`.
- Re-export all standard library operation traits (`Add`, `Sub`, etc.) and all custom traits so they are accessible from `crate::math::operations::*`.

### Integration Details

1. **`src/math/mod.rs`**: Keep `pub mod operations;` unchanged (since operations will now be registered in its own directory's `mod.rs`).
2. **Delete `src/math/operations.rs`**: Delete the old flat operations file.
3. **Move Primitive Implementations**:
   The implementations of `Pow` and `PowAssign` for primitives (via macro `impl_pow_int!`) will move to `src/math/operations/power.rs`.

## Verification Plan
1. Ensure the project compiles successfully.
2. Run `cargo test` to make sure all existing tests (including the primitive exponentiation tests) still pass.
