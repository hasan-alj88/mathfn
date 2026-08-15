//! MultiVector representation and operations.
//!
//! Provides the [`MultiVector`] struct which represents a general element of a
//! Geometric Algebra space, parameterized by coefficient type `T`, coordinate system `Coord`,
//! and dimension limit `MAX_DIM`.
//!
//! ### Mathematical Theory
//!
//! A multivector is a sum of components, each belonging to a specific basis blade:
//!
//! $$ A = \sum_{I} a_I e_I $$
//!
//! where $a_I$ is a scalar coefficient and $e_I$ is a basis blade.
//!
//! The elements are maintained in a canonical, sorted, and merged order:
//! 1. Components are sorted by grade, then by bitmask representation.
//! 2. Duplicate basis blades are combined.
//! 3. Components with zero coefficients are pruned.

use std::marker::PhantomData;
use crate::math::geometric_algebra::{Blade, Component, CoordinateSystem};

/// Strongly-typed Geometric Algebra multivector.
///
/// MultiVector instances represent arbitrary elements of a GA space. They are compile-time bound
/// to a coordinate system `Coord` and a const generic maximum dimension `MAX_DIM`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MultiVector<T, Coord, const MAX_DIM: usize> {
    /// Canonical components sorted and merged.
    pub components: Vec<Component<T, MAX_DIM>>,
    /// Phantom type marking the coordinate system.
    pub _coord: PhantomData<Coord>,
}

impl<T, Coord, const MAX_DIM: usize> MultiVector<T, Coord, MAX_DIM>
where
    T: num_traits::Zero + Clone + PartialEq + std::ops::AddAssign,
{
    /// Creates a new `MultiVector` and canonicalizes its components.
    pub fn new(components: Vec<Component<T, MAX_DIM>>) -> Self {
        let mut mv = MultiVector {
            components,
            _coord: PhantomData,
        };
        mv.normalize();
        mv
    }

    /// Canonicalizes the internal representation.
    ///
    /// Sorts components by blade key, combines duplicates, and filters out zero coefficients.
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

/// Helper function to format axis labels of a basis blade.
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
