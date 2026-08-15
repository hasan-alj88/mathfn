# Design Specification: Strongly-Typed Geometric Algebra (GA) MultiVector Library in Rust

## 1. Objective & Architecture

This library implements a compile-time safe, strongly-typed Geometric Algebra (GA) library in Rust. It utilizes Rust's const generics, zero-sized marker structs (typestate pattern), and strict enum boundaries to prevent dimensional and coordinate-system mixing at compile-time.

A primary design constraint is **Option 1: Standard Canonical Normalization with Coordinate-Specific Labeling**, where the internal multivector components are always sorted, merged, and pruned in an algebraic canonical order (grade-first, then bitmask-second). The coordinate systems themselves only dictate the display labeling and the conversion bijections.

All implementations strictly refrain from using `if-else` blocks, utilizing Rust's pattern-matching expressions and guards for maximum safety, elegance, and optimization.

---

## 2. File & Module Structure

The library will be placed in a new module, `src/math/geometric_algebra`.

```text
src/math/geometric_algebra/
├── mod.rs                       # Main entrypoint, re-exports public APIs
├── blade.rs                     # Blade and Component types, basis multiplication (sign flips)
├── multivector.rs               # MultiVector, normalization, Add/Sub/Mul/Neg/Zero/Display
├── coordinate_systems/          # Submodule for coordinate typestates and conversions
│   ├── mod.rs                   # CoordinateSystem trait and submodules registration
│   ├── cartesian.rs             # Cartesian system implementation
│   └── polar.rs                 # Polar system implementation & 2D conversions
├── transforms/                  # Submodule for subspace embeddings
│   ├── mod.rs                   # Embed trait & generic embedding logic
└── tests.rs                     # Comprehensive unit tests
```

---

## 3. Detailed Component Designs

### 3.1. Blade and Component (`src/math/geometric_algebra/blade.rs`)

`Blade` represents the algebraic basis of a multivector component. The `Scalar` variant represents a Grade-0 pure number, which is coordinate-invariant and universally compatible. `MultiVectorBlade` represents directional blades (Grades 1 through `MAX_DIM`), using bitwise encoding where the $i$-th basis vector corresponds to `1 << i`.

```rust
/// Algebraic basis representation.
///
/// Separates pure numbers (scalars) from directional geometry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Blade<const MAX_DIM: usize> {
    /// Grade-0 invariant scalar.
    Scalar,
    /// Directional basis blade with a non-zero bitmask.
    MultiVectorBlade { bits: usize },
}

/// A term in the multivector, combining a coefficient value and its basis blade.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Component<T, const MAX_DIM: usize> {
    pub value: T,
    pub blade: Blade<MAX_DIM>,
}
```

#### Swap Counting & Basis Multiplication Sign
To multiply two basis blades, we count the number of swaps required to bring the elements into canonical order (LSB-to-MSB) and compute the resulting sign:

```rust
/// Counts the number of swaps to sort basis vectors canonically.
pub fn count_swaps(bits_a: usize, bits_b: usize) -> u32 {
    let mut swaps = 0;
    let mut b = bits_b;
    loop {
        match b {
            0 => break,
            _ => {
                let lsb_pos = b.trailing_zeros();
                b &= b - 1; // Clear LSB
                let greater_bits = bits_a >> (lsb_pos + 1);
                swaps += greater_bits.count_ones();
            }
        }
    }
    swaps
}
```

Component multiplication pattern matches on the blade variants:

```rust
impl<T, const MAX_DIM: usize> Component<T, MAX_DIM>
where
    T: Clone + std::ops::Mul<Output = T> + std::ops::Neg<Output = T>,
{
    pub fn mul_components(a: &Component<T, MAX_DIM>, b: &Component<T, MAX_DIM>) -> Self {
        match (a.blade, b.blade) {
            (Blade::Scalar, Blade::Scalar) => Component {
                value: a.value.clone() * b.value.clone(),
                blade: Blade::Scalar,
            },
            (Blade::Scalar, Blade::MultiVectorBlade { bits }) => Component {
                value: a.value.clone() * b.value.clone(),
                blade: Blade::MultiVectorBlade { bits },
            },
            (Blade::MultiVectorBlade { bits }, Blade::Scalar) => Component {
                value: a.value.clone() * b.value.clone(),
                blade: Blade::MultiVectorBlade { bits },
            },
            (Blade::MultiVectorBlade { bits: bits_a }, Blade::MultiVectorBlade { bits: bits_b }) => {
                let bits_out = bits_a ^ bits_b;
                let swaps = count_swaps(bits_a, bits_b);
                let value_signed = match swaps % 2 {
                    0 => a.value.clone() * b.value.clone(),
                    _ => -(a.value.clone() * b.value.clone()),
                };
                let blade_out = match bits_out {
                    0 => Blade::Scalar,
                    _ => Blade::MultiVectorBlade { bits: bits_out },
                };
                Component {
                    value: value_signed,
                    blade: blade_out,
                }
            }
        }
    }
}
```

---

### 3.2. MultiVector (`src/math/geometric_algebra/multivector.rs`)

`MultiVector` is the primary container representing a multivector. It enforces compile-time safety across different dimensions and coordinate system typestates.

```rust
use std::marker::PhantomData;

/// Strongly-typed Geometric Algebra multivector.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MultiVector<T, Coord, const MAX_DIM: usize> {
    pub components: Vec<Component<T, MAX_DIM>>,
    pub _coord: PhantomData<Coord>,
}
```

#### Canonicalization & Operations
MultiVectors are eagerly normalized:
1. Components are sorted by `Blade` canonical order: `Scalar` first, then by grade, then by `bits`.
2. Duplicate blades are combined.
3. Zero components are pruned.

```rust
impl<const MAX_DIM: usize> Blade<MAX_DIM> {
    pub fn sort_key(&self) -> (usize, usize) {
        match self {
            Blade::Scalar => (0, 0),
            Blade::MultiVectorBlade { bits } => (bits.count_ones() as usize, *bits),
        }
    }
}

impl<const MAX_DIM: usize> Ord for Blade<MAX_DIM> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.sort_key().cmp(&other.sort_key())
    }
}
impl<const MAX_DIM: usize> PartialOrd for Blade<MAX_DIM> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
```

Normalization implementation:

```rust
impl<T, Coord, const MAX_DIM: usize> MultiVector<T, Coord, MAX_DIM>
where
    T: num_traits::Zero + Clone + PartialEq + std::ops::AddAssign,
{
    pub fn normalize(&mut self) {
        self.components.sort_by(|a, b| a.blade.cmp(&b.blade));

        let mut merged: Vec<Component<T, MAX_DIM>> = Vec::with_capacity(self.components.len());
        for comp in self.components.drain(..) {
            match merged.last_mut() {
                Some(last) if last.blade == comp.blade => {
                    last.value += comp.value;
                }
                _ => {
                    merged.push(comp);
                }
            }
        }

        self.components = merged
            .into_iter()
            .filter(|c| match c.value.is_zero() {
                true => false,
                false => true,
            })
            .collect();
    }
}
```

Standard operators (`Add`, `Sub`, `Neg`, `Mul`, `Zero`) will be implemented using these normalized representations. For example, geometric product multiplies all component pairs and normalizes.

---

### 3.3. Coordinate Systems (`src/math/geometric_algebra/coordinate_systems/`)

The `CoordinateSystem` trait specifies how each system formats labels for active bits.

```rust
pub trait CoordinateSystem {
    fn axis_label(position: usize) -> &'static str;
}
```

- **`Cartesian`** maps index 0 $\to$ "x", 1 $\to$ "y", 2 $\to$ "z".
- **`Polar`** maps index 0 $\to$ "r", 1 $\to$ "θ".

For conversions, we implement standard `From`/`Into` traits:
- `From<MultiVector<f64, Cartesian, 2>> for MultiVector<f64, Polar, 2>` using polar formulas:
  $$ r = \sqrt{x^2 + y^2}, \quad \theta = \arctan2(y, x) $$
- `From<MultiVector<f64, Polar, 2>> for MultiVector<f64, Cartesian, 2>` using cartesian formulas:
  $$ x = r \cos(\theta), \quad y = r \sin(\theta) $$

---

### 3.4. Subspace Embeddings (`src/math/geometric_algebra/transforms/`)

Embeddings map components from a lower-dimensional space to a higher-dimensional space given a target subspace bitmask.

```rust
pub trait Embed<T, TargetCoord, const TARGET_DIM: usize> {
    type Output;
    fn embed(&self, target_subspace_mask: usize) -> Self::Output;
}
```

The embedding mapping maps each set bit position in the source blade to the corresponding active bit position in the target mask.

---

## 4. Verification Plan

We will add extensive tests in `src/math/geometric_algebra/tests.rs` covering:
1. **Canonical sorting & normalization:** Ensuring components are sorted by grade, then by bitmask, and zeros are removed.
2. **Geometric Product:** Validating basis products (e.g. $e_x e_y = -e_y e_x$, $e_x^2 = 1$).
3. **Coordinate-System Isolation:** Verifying that trying to add Cartesian and Polar multivectors results in a compile-time failure.
4. **Coordinate Conversions:** Verifying 2D Cartesian $\leftrightarrow$ Polar bijections.
5. **Subspace Embeddings:** Verifying X/Y vector embedding into XZ plane of a 3D Cartesian multivector.
