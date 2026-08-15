//! Algebraic basis and term representation for Geometric Algebra.
//!
//! Provides the [`Blade`] enum to represent algebraic basis blades (coordinates vs. scalar)
//! and [`Component`] to represent a single term (coefficient value and basis blade).
//!
//! ### Mathematical Theory
//!
//! In an $n$-dimensional Geometric Algebra space, basis vectors $e_1, e_2, \dots, e_n$ generate
//! a vector space of dimension $2^n$ containing scalars (grade 0), vectors (grade 1), bivectors (grade 2),
//! up to pseudoscalars (grade $n$).
//!
//! We represent basis blades using unique bits in an unsigned integer:
//! - Position 0 ($001$ in binary) corresponds to $e_1$.
//! - Position 1 ($010$ in binary) corresponds to $e_2$.
//! - Position 2 ($100$ in binary) corresponds to $e_3$, and so on.
//!
//! The `Blade` enum separates the invariant grade-0 `Scalar` (which has no geometric bits and is
//! invariant across coordinate transformations) from directional `MultiVectorBlade`s.

/// Algebraic basis representation.
///
/// Separates pure numbers (scalars) from directional geometry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Blade<const MAX_DIM: usize> {
    /// Grade-0 invariant scalar.
    Scalar,
    /// Directional basis blade with a non-zero bitmask.
    MultiVectorBlade {
        /// Bitmask representing active basis vectors.
        bits: usize,
    },
}

impl<const MAX_DIM: usize> Blade<MAX_DIM> {
    /// Helper to get the grade of the blade.
    ///
    /// The grade is the number of set bits for a multivector blade, or 0 for a scalar.
    pub fn grade(&self) -> usize {
        match self {
            Blade::Scalar => 0,
            Blade::MultiVectorBlade { bits } => bits.count_ones() as usize,
        }
    }

    /// Returns a sorting key used to canonicalize components.
    ///
    /// The sort key enforces:
    /// 1. Grade first (scalars/grade-0 first, then higher grades).
    /// 2. Bitmask value second (for blades of the same grade).
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

/// A term in the multivector, combining a coefficient value and its basis blade.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Component<T, const MAX_DIM: usize> {
    /// The coefficient of the component.
    pub value: T,
    /// The basis blade representation.
    pub blade: Blade<MAX_DIM>,
}

/// Counts the number of swaps to sort basis vectors canonically.
///
/// Computes the number of adjacent swaps needed to sort the merged basis indices
/// of two blades to LSB-to-MSB canonical order.
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
    /// Multiplies two individual components.
    ///
    /// Implements the geometric product of two basis blades, returning the resulting
    /// component with its sign adjusted for anti-commuting basis vectors.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swaps_and_mul() {
        // e2 (bits: 4) * e1 (bits: 2) should require 1 swap and yield negative sign
        assert_eq!(count_swaps(4, 2), 1);
        
        let comp_a = Component::<f64, 3> { value: 1.0, blade: Blade::MultiVectorBlade { bits: 4 } }; // e2
        let comp_b = Component::<f64, 3> { value: 1.0, blade: Blade::MultiVectorBlade { bits: 2 } }; // e1
        let res = Component::mul_components(&comp_a, &comp_b);
        assert_eq!(res.value, -1.0);
        assert_eq!(res.blade, Blade::MultiVectorBlade { bits: 6 }); // e12
    }
}
