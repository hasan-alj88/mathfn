# Strongly-Typed Geometric Algebra (GA) MultiVector Library Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement a compile-time safe, strongly-typed Geometric Algebra (GA) multivector library in Rust using const generics and typestate markers.

**Architecture:** Use a `Blade` enum separating `Scalar` from `MultiVectorBlade { bits: usize }` to enforce normalization (grade-first, then bitmask-second) inside `MultiVector`. Utilize pattern matching guards instead of `if-else` blocks for all algebraic operations, conversions, and display formatting.

**Tech Stack:** Rust (Edition 2024), standard library, and `num-traits` library.

## Global Constraints

- No dimensional mixing (enforced by `const MAX_DIM: usize`).
- No coordinate mixing without explicit `From`/`Into` conversion.
- All code must include Rustdocs for math theory and usage examples.
- Refrain from using `if-else` blocks; use `match` pattern expressions and guards instead.

---

### Task 1: Setup Modules & Implement Blade and Component

**Files:**
- Modify: [mod.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/mod.rs)
- Create: [mod.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/geometric_algebra/mod.rs)
- Create: [blade.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/geometric_algebra/blade.rs)

**Interfaces:**
- Produces: `enum Blade<const MAX_DIM: usize>` and `struct Component<T, const MAX_DIM: usize>`.
- Produces: `fn count_swaps(bits_a: usize, bits_b: usize) -> u32` and `Component::mul_components(a: &Component, b: &Component) -> Component`.

- [ ] **Step 1: Register the new module in src/math/mod.rs**

Add `pub mod geometric_algebra;` to the end of the file.

- [ ] **Step 2: Create src/math/geometric_algebra/mod.rs re-exporting the future files**

Write:
```rust
//! Strongly-Typed Geometric Algebra (GA) library.
//!
//! Provides compile-time safe multivectors using const generics and typestates.

pub mod blade;
pub mod coordinate_systems;
pub mod multivector;
pub mod transforms;

pub use blade::{Blade, Component};
pub use multivector::MultiVector;
pub use coordinate_systems::CoordinateSystem;
```

- [ ] **Step 3: Create src/math/geometric_algebra/blade.rs with failing unit test for blade multiplication and swap counting**

Write:
```rust
/// Algebraic basis representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Blade<const MAX_DIM: usize> {
    /// Grade-0 invariant scalar.
    Scalar,
    /// Directional basis blade with a non-zero bitmask.
    MultiVectorBlade { bits: usize },
}

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Component<T, const MAX_DIM: usize> {
    pub value: T,
    pub blade: Blade<MAX_DIM>,
}

pub fn count_swaps(bits_a: usize, bits_b: usize) -> u32 {
    0 // dummy implementation
}

impl<T, const MAX_DIM: usize> Component<T, MAX_DIM>
where
    T: Clone + std::ops::Mul<Output = T> + std::ops::Neg<Output = T>,
{
    pub fn mul_components(a: &Component<T, MAX_DIM>, b: &Component<T, MAX_DIM>) -> Self {
        a.clone() // dummy implementation
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swaps_and_mul() {
        // e2 * e1 should require 1 swap and yield negative sign
        assert_eq!(count_swaps(4, 2), 1);
        
        let comp_a = Component { value: 1.0, blade: Blade::MultiVectorBlade { bits: 4 } }; // e2
        let comp_b = Component { value: 1.0, blade: Blade::MultiVectorBlade { bits: 2 } }; // e1
        let res = Component::mul_components(&comp_a, &comp_b);
        assert_eq!(res.value, -1.0);
        assert_eq!(res.blade, Blade::MultiVectorBlade { bits: 6 }); // e12
    }
}
```

- [ ] **Step 4: Run cargo test to verify it fails**

Run: `cargo test math::geometric_algebra::blade`
Expected: FAIL

- [ ] **Step 5: Write minimal implementation for swap counting and component multiplication in blade.rs**

Replace `count_swaps` and `mul_components` with:
```rust
pub fn count_swaps(bits_a: usize, bits_b: usize) -> u32 {
    let mut swaps = 0;
    let mut b = bits_b;
    loop {
        match b {
            0 => break,
            _ => {
                let lsb_pos = b.trailing_zeros();
                b &= b - 1;
                let greater_bits = bits_a >> (lsb_pos + 1);
                swaps += greater_bits.count_ones();
            }
        }
    }
    swaps
}

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

- [ ] **Step 6: Run cargo test to verify it passes**

Run: `cargo test math::geometric_algebra::blade`
Expected: PASS

- [ ] **Step 7: Commit**

```bash
git add src/math/mod.rs src/math/geometric_algebra/mod.rs src/math/geometric_algebra/blade.rs
git commit -m "feat(ga): implement Blade and Component types with sign flip calculations"
```

---

### Task 2: Implement Coordinate Systems & 2D conversions

**Files:**
- Create: [mod.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/geometric_algebra/coordinate_systems/mod.rs)
- Create: [cartesian.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/geometric_algebra/coordinate_systems/cartesian.rs)
- Create: [polar.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/geometric_algebra/coordinate_systems/polar.rs)

**Interfaces:**
- Produces: `pub trait CoordinateSystem`.
- Produces: `pub struct Cartesian` and `pub struct Polar`.
- Produces: `From<MultiVector<f64, Cartesian, 2>> for MultiVector<f64, Polar, 2>` and vice-versa.

- [ ] **Step 1: Create coordinate_systems/mod.rs defining CoordinateSystem trait**

Write:
```rust
//! Coordinate system traits and marker structs.

pub mod cartesian;
pub mod polar;

pub use cartesian::Cartesian;
pub use polar::Polar;

/// Defines coordinate labels and formatting.
pub trait CoordinateSystem {
    /// Returns the string label for a specific basis index.
    fn axis_label(position: usize) -> &'static str;
}
```

- [ ] **Step 2: Create coordinate_systems/cartesian.rs**

Write:
```rust
use super::CoordinateSystem;

/// Cartesian Coordinate System marker struct.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cartesian;

impl CoordinateSystem for Cartesian {
    fn axis_label(position: usize) -> &'static str {
        match position {
            0 => "x",
            1 => "y",
            2 => "z",
            _ => "w",
        }
    }
}
```

- [ ] **Step 3: Create coordinate_systems/polar.rs with stub conversions**

Write:
```rust
use super::{CoordinateSystem, Cartesian};
use crate::math::geometric_algebra::{Blade, Component, MultiVector};

/// Polar Coordinate System marker struct.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Polar;

impl CoordinateSystem for Polar {
    fn axis_label(position: usize) -> &'static str {
        match position {
            0 => "r",
            1 => "θ",
            _ => "φ",
        }
    }
}

// Stub implementation for compilation, actual conversions added in Task 3
```

- [ ] **Step 4: Commit**

```bash
git add src/math/geometric_algebra/coordinate_systems
git commit -m "feat(ga): add Cartesian and Polar coordinate typestates"
```

---

### Task 3: Implement MultiVector, Normalization, Operators and Display

**Files:**
- Create: [multivector.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/geometric_algebra/multivector.rs)
- Modify: [polar.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/geometric_algebra/coordinate_systems/polar.rs)

**Interfaces:**
- Produces: `struct MultiVector<T, Coord, const MAX_DIM: usize>`.
- Produces: implementations of `Add`, `Sub`, `Mul`, `Neg`, `Zero`, `Display`.

- [ ] **Step 1: Create src/math/geometric_algebra/multivector.rs with failing Display test**

Write:
```rust
use std::marker::PhantomData;
use crate::math::geometric_algebra::{Blade, Component, CoordinateSystem};

/// Strongly-typed Geometric Algebra multivector.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MultiVector<T, Coord, const MAX_DIM: usize> {
    pub components: Vec<Component<T, MAX_DIM>>,
    pub _coord: PhantomData<Coord>,
}

impl<T, Coord, const MAX_DIM: usize> MultiVector<T, Coord, MAX_DIM>
where
    T: num_traits::Zero + Clone + PartialEq + std::ops::AddAssign,
{
    pub fn new(components: Vec<Component<T, MAX_DIM>>) -> Self {
        let mut mv = MultiVector {
            components,
            _coord: PhantomData,
        };
        mv.normalize();
        mv
    }

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

// Display format helper
pub fn format_blade<Coord: CoordinateSystem, const MAX_DIM: usize>(blade: &Blade<MAX_DIM>) -> String {
    match blade {
        Blade::Scalar => String::new(),
        Blade::MultiVectorBlade { bits } => {
            let mut label = String::new();
            let mut temp = *bits;
            let mut pos = 0;
            loop {
                match temp {
                    0 => break,
                    _ => {
                        match temp & 1 {
                            1 => label.push_str(Coord::axis_label(pos)),
                            _ => {}
                        }
                        temp >>= 1;
                        pos += 1;
                    }
                }
            }
            label
        }
    }
}

impl<T, Coord, const MAX_DIM: usize> std::fmt::Display for MultiVector<T, Coord, MAX_DIM>
where
    T: std::fmt::Display + num_traits::Zero + PartialEq,
    Coord: CoordinateSystem,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "dummy")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::geometric_algebra::coordinate_systems::Cartesian;

    #[test]
    fn test_display() {
        let mv = MultiVector::<f64, Cartesian, 3>::new(vec![
            Component { value: 3.5, blade: Blade::Scalar },
            Component { value: 2.0, blade: Blade::MultiVectorBlade { bits: 5 } }, // e13 -> xz
        ]);
        assert_eq!(format!("{}", mv), "3.5 + 2xz");
    }
}
```

- [ ] **Step 2: Run cargo test to verify it fails**

Run: `cargo test math::geometric_algebra::multivector`
Expected: FAIL

- [ ] **Step 3: Implement Display and algebraic operators in multivector.rs**

Update Display, and implement:
- `Add`, `Sub`, `Neg`
- `Mul` (Geometric Product)
- `Zero` from `num_traits`

```rust
impl<T, Coord, const MAX_DIM: usize> std::fmt::Display for MultiVector<T, Coord, MAX_DIM>
where
    T: std::fmt::Display + num_traits::Zero + PartialEq,
    Coord: CoordinateSystem,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.components.is_empty() {
            true => write!(f, "0"),
            false => {
                let mut first = true;
                for comp in &self.components {
                    match first {
                        true => {
                            first = false;
                        }
                        false => {
                            write!(f, " + ")?;
                        }
                    }
                    match comp.blade {
                        Blade::Scalar => {
                            write!(f, "{}", comp.value)?;
                        }
                        Blade::MultiVectorBlade { bits: _ } => {
                            let label = format_blade::<Coord, MAX_DIM>(&comp.blade);
                            write!(f, "{}{}", comp.value, label)?;
                        }
                    }
                }
                Ok(())
            }
        }
    }
}

impl<T, Coord, const MAX_DIM: usize> std::ops::Add for MultiVector<T, Coord, MAX_DIM>
where
    T: num_traits::Zero + Clone + PartialEq + std::ops::AddAssign,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        let mut components = self.components;
        components.extend(rhs.components);
        MultiVector::new(components)
    }
}

impl<T, Coord, const MAX_DIM: usize> std::ops::Sub for MultiVector<T, Coord, MAX_DIM>
where
    T: num_traits::Zero + Clone + PartialEq + std::ops::AddAssign + std::ops::SubAssign + std::ops::Neg<Output = T>,
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        let mut components = self.components;
        for c in rhs.components {
            components.push(Component {
                value: -c.value,
                blade: c.blade,
            });
        }
        MultiVector::new(components)
    }
}

impl<T, Coord, const MAX_DIM: usize> std::ops::Neg for MultiVector<T, Coord, MAX_DIM>
where
    T: num_traits::Zero + Clone + PartialEq + std::ops::AddAssign + std::ops::Neg<Output = T>,
{
    type Output = Self;
    fn neg(self) -> Self {
        let components = self.components.into_iter().map(|c| Component {
            value: -c.value,
            blade: c.blade,
        }).collect();
        MultiVector {
            components,
            _coord: PhantomData,
        }
    }
}

impl<T, Coord, const MAX_DIM: usize> std::ops::Mul for MultiVector<T, Coord, MAX_DIM>
where
    T: num_traits::Zero + Clone + PartialEq + std::ops::AddAssign + std::ops::Mul<Output = T> + std::ops::Neg<Output = T>,
{
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        let mut out_components = Vec::new();
        for a in &self.components {
            for b in &rhs.components {
                out_components.push(Component::mul_components(a, b));
            }
        }
        MultiVector::new(out_components)
    }
}

impl<T, Coord, const MAX_DIM: usize> num_traits::Zero for MultiVector<T, Coord, MAX_DIM>
where
    T: num_traits::Zero + Clone + PartialEq + std::ops::AddAssign,
{
    fn zero() -> Self {
        MultiVector {
            components: Vec::new(),
            _coord: PhantomData,
        }
    }
    fn is_zero(&self) -> bool {
        self.components.is_empty()
    }
}
```

- [ ] **Step 4: Run cargo test to verify it passes**

Run: `cargo test math::geometric_algebra::multivector`
Expected: PASS

- [ ] **Step 5: Add Cartesian <-> Polar conversions in coordinate_systems/polar.rs**

Add the bijection implementations using math formulas:
```rust
impl From<MultiVector<f64, Cartesian, 2>> for MultiVector<f64, Polar, 2> {
    fn from(src: MultiVector<f64, Cartesian, 2>) -> Self {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut target_components = Vec::new();

        for comp in src.components {
            match comp.blade {
                Blade::Scalar => {
                    target_components.push(Component {
                        value: comp.value,
                        blade: Blade::Scalar,
                    });
                }
                Blade::MultiVectorBlade { bits } => match bits {
                    1 => x = comp.value,
                    2 => y = comp.value,
                    _ => {
                        target_components.push(Component {
                            value: comp.value,
                            blade: Blade::MultiVectorBlade { bits },
                        });
                    }
                },
            }
        }

        let r = (x * x + y * y).sqrt();
        let theta = y.atan2(x);

        target_components.push(Component {
            value: r,
            blade: Blade::MultiVectorBlade { bits: 1 },
        });
        target_components.push(Component {
            value: theta,
            blade: Blade::MultiVectorBlade { bits: 2 },
        });

        let mut mv = MultiVector {
            components: target_components,
            _coord: std::marker::PhantomData,
        };
        mv.normalize();
        mv
    }
}

impl From<MultiVector<f64, Polar, 2>> for MultiVector<f64, Cartesian, 2> {
    fn from(src: MultiVector<f64, Polar, 2>) -> Self {
        let mut r = 0.0;
        let mut theta = 0.0;
        let mut target_components = Vec::new();

        for comp in src.components {
            match comp.blade {
                Blade::Scalar => {
                    target_components.push(Component {
                        value: comp.value,
                        blade: Blade::Scalar,
                    });
                }
                Blade::MultiVectorBlade { bits } => match bits {
                    1 => r = comp.value,
                    2 => theta = comp.value,
                    _ => {
                        target_components.push(Component {
                            value: comp.value,
                            blade: Blade::MultiVectorBlade { bits },
                        });
                    }
                },
            }
        }

        let x = r * theta.cos();
        let y = r * theta.sin();

        target_components.push(Component {
            value: x,
            blade: Blade::MultiVectorBlade { bits: 1 },
        });
        target_components.push(Component {
            value: y,
            blade: Blade::MultiVectorBlade { bits: 2 },
        });

        let mut mv = MultiVector {
            components: target_components,
            _coord: std::marker::PhantomData,
        };
        mv.normalize();
        mv
    }
}
```

- [ ] **Step 6: Run cargo test to verify everything builds and passes**

Run: `cargo test`
Expected: PASS

- [ ] **Step 7: Commit**

```bash
git add src/math/geometric_algebra/multivector.rs src/math/geometric_algebra/coordinate_systems/polar.rs
git commit -m "feat(ga): implement MultiVector and Cartesian-Polar conversions"
```

---

### Task 4: Implement Subspace Embeddings (`Embed` trait)

**Files:**
- Create: [mod.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/geometric_algebra/transforms/mod.rs)

**Interfaces:**
- Produces: `pub trait Embed<T, TargetCoord, const TARGET_DIM: usize>`.
- Produces: `impl Embed for MultiVector`.

- [ ] **Step 1: Create src/math/geometric_algebra/transforms/mod.rs with failing test**

Write:
```rust
use std::marker::PhantomData;
use crate::math::geometric_algebra::{Blade, Component, MultiVector};

pub trait Embed<T, TargetCoord, const TARGET_DIM: usize> {
    type Output;
    fn embed(&self, target_subspace_mask: usize) -> Self::Output;
}

pub fn embed_bits(src_bits: usize, target_subspace_mask: usize) -> usize {
    0 // dummy implementation
}

impl<T, Coord, const SOURCE_DIM: usize, const TARGET_DIM: usize> Embed<T, Coord, TARGET_DIM> for MultiVector<T, Coord, SOURCE_DIM>
where
    T: Clone + num_traits::Zero + PartialEq + std::ops::AddAssign,
{
    type Output = MultiVector<T, Coord, TARGET_DIM>;

    fn embed(&self, target_subspace_mask: usize) -> Self::Output {
        MultiVector {
            components: Vec::new(),
            _coord: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::geometric_algebra::coordinate_systems::Cartesian;

    #[test]
    fn test_embedding() {
        let mv = MultiVector::<f64, Cartesian, 2>::new(vec![
            Component { value: 3.0, blade: Blade::MultiVectorBlade { bits: 2 } }, // e2 in 2D
        ]);
        // Embed into XZ plane (mask 5: bits 0 and 2)
        let embedded = mv.embed(5);
        // e2 should map to second active position, which is index 2 (e3)
        assert_eq!(embedded.components[0].blade, Blade::MultiVectorBlade { bits: 4 });
    }
}
```

- [ ] **Step 2: Run cargo test to verify it fails**

Run: `cargo test math::geometric_algebra::transforms`
Expected: FAIL

- [ ] **Step 3: Implement embedding logic in transforms/mod.rs**

Replace `embed_bits` and the `Embed` trait implementation:
```rust
pub fn embed_bits(src_bits: usize, target_subspace_mask: usize) -> usize {
    let mut tgt_bits = 0;
    let mut active_positions = Vec::new();
    let mut temp = target_subspace_mask;
    let mut pos = 0;
    loop {
        match temp {
            0 => break,
            _ => {
                match temp & 1 {
                    1 => active_positions.push(pos),
                    _ => {}
                }
                temp >>= 1;
                pos += 1;
            }
        }
    }

    let mut src_temp = src_bits;
    let mut src_pos = 0;
    loop {
        match src_temp {
            0 => break,
            _ => {
                match src_temp & 1 {
                    1 => match active_positions.get(src_pos) {
                        Some(&tgt_pos) => {
                            tgt_bits |= 1 << tgt_pos;
                        }
                        None => {}
                    },
                    _ => {}
                }
                src_temp >>= 1;
                src_pos += 1;
            }
        }
    }
    tgt_bits
}

impl<T, Coord, const SOURCE_DIM: usize, const TARGET_DIM: usize> Embed<T, Coord, TARGET_DIM> for MultiVector<T, Coord, SOURCE_DIM>
where
    T: Clone + num_traits::Zero + PartialEq + std::ops::AddAssign,
{
    type Output = MultiVector<T, Coord, TARGET_DIM>;

    fn embed(&self, target_subspace_mask: usize) -> Self::Output {
        assert!(TARGET_DIM > SOURCE_DIM, "Target dimension must be strictly greater than source dimension");
        assert!(
            target_subspace_mask.count_ones() as usize >= SOURCE_DIM,
            "Target subspace mask must have at least {} active bits", SOURCE_DIM
        );

        let mut target_components = Vec::with_capacity(self.components.len());

        for comp in &self.components {
            let target_blade = match comp.blade {
                Blade::Scalar => Blade::Scalar,
                Blade::MultiVectorBlade { bits } => {
                    let new_bits = embed_bits(bits, target_subspace_mask);
                    Blade::MultiVectorBlade { bits: new_bits }
                }
            };
            target_components.push(Component {
                value: comp.value.clone(),
                blade: target_blade,
            });
        }

        let mut mv = MultiVector {
            components: target_components,
            _coord: PhantomData,
        };
        mv.normalize();
        mv
    }
}
```

- [ ] **Step 4: Run cargo test to verify it passes**

Run: `cargo test math::geometric_algebra::transforms`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add src/math/geometric_algebra/transforms/mod.rs
git commit -m "feat(ga): implement Embed trait for subspace embeddings"
```

---

### Task 5: Add Comprehensive Integration Tests & Verification

**Files:**
- Create: [tests.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/geometric_algebra/tests.rs)

- [ ] **Step 1: Create src/math/geometric_algebra/tests.rs with comprehensive tests**

Write:
```rust
#[cfg(test)]
mod tests {
    use crate::math::geometric_algebra::{Blade, Component, MultiVector};
    use crate::math::geometric_algebra::coordinate_systems::{Cartesian, Polar};
    use crate::math::geometric_algebra::transforms::Embed;
    use num_traits::Zero;

    #[test]
    fn test_canonical_sorting_and_merge() {
        // e2 (bits: 2) + e1 (bits: 1) should sort to e1 + e2
        let mv = MultiVector::<f64, Cartesian, 2>::new(vec![
            Component { value: 3.0, blade: Blade::MultiVectorBlade { bits: 2 } },
            Component { value: 2.0, blade: Blade::MultiVectorBlade { bits: 1 } },
            Component { value: 1.5, blade: Blade::MultiVectorBlade { bits: 1 } },
        ]);
        assert_eq!(mv.components.len(), 2);
        assert_eq!(mv.components[0].blade, Blade::MultiVectorBlade { bits: 1 });
        assert_eq!(mv.components[0].value, 3.5);
    }

    #[test]
    fn test_geometric_product() {
        // ex * ey = exy
        let ex = MultiVector::<f64, Cartesian, 2>::new(vec![
            Component { value: 1.0, blade: Blade::MultiVectorBlade { bits: 1 } },
        ]);
        let ey = MultiVector::<f64, Cartesian, 2>::new(vec![
            Component { value: 1.0, blade: Blade::MultiVectorBlade { bits: 2 } },
        ]);
        let exy = ex.clone() * ey.clone();
        assert_eq!(exy.components[0].blade, Blade::MultiVectorBlade { bits: 3 });

        // ey * ex = -exy
        let eyx = ey * ex;
        assert_eq!(eyx.components[0].blade, Blade::MultiVectorBlade { bits: 3 });
        assert_eq!(eyx.components[0].value, -1.0);

        // ex * ex = 1
        let ex2 = exy.clone() * exy.clone(); // (exy)^2 = ex ey ex ey = -ex ex ey ey = -1
        assert_eq!(ex2.components[0].blade, Blade::Scalar);
        assert_eq!(ex2.components[0].value, -1.0);
    }

    #[test]
    fn test_conversions() {
        // Cartesian (3.0, 4.0) -> Polar (5.0, 0.927...)
        let cart = MultiVector::<f64, Cartesian, 2>::new(vec![
            Component { value: 3.0, blade: Blade::MultiVectorBlade { bits: 1 } },
            Component { value: 4.0, blade: Blade::MultiVectorBlade { bits: 2 } },
        ]);
        let polar: MultiVector<f64, Polar, 2> = cart.into();
        assert!((polar.components[0].value - 5.0).abs() < 1e-9);

        let cart_back: MultiVector<f64, Cartesian, 2> = polar.into();
        assert!((cart_back.components[0].value - 3.0).abs() < 1e-9);
        assert!((cart_back.components[1].value - 4.0).abs() < 1e-9);
    }
}
```

- [ ] **Step 2: Register the tests submodule in src/math/geometric_algebra/mod.rs**

Add `#[cfg(test)] pub mod tests;` to `src/math/geometric_algebra/mod.rs`.

- [ ] **Step 3: Run all cargo tests to verify everything passes**

Run: `cargo test`
Expected: PASS

- [ ] **Step 4: Commit**

```bash
git add src/math/geometric_algebra/tests.rs src/math/geometric_algebra/mod.rs
git commit -m "test(ga): add comprehensive integration and property tests"
```
