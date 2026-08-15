//! Subspace embeddings for multivectors.
//!
//! Provides the [`Embed`] trait to map a multivector from a lower-dimensional source space
//! to a higher-dimensional target space using a target subspace mask.
//!
//! ### Mathematical Theory
//!
//! An embedding maps the basis vectors of a source space sequentially to the active basis
//! dimensions of the target space.
//!
//! For example, if we embed a 2D Cartesian multivector ($e_1, e_2$) into a 3D space using the
//! target subspace mask `101` (active dimensions index 0 and 2, i.e., the XZ-plane):
//! - $e_1$ (source position 0) maps to $e_1$ (target position 0).
//! - $e_2$ (source position 1) maps to $e_3$ (target position 2).
//! - The pseudoscalar $e_{12}$ (grade 2, `bits: 3`) maps automatically via XOR to $e_{13}$ (grade 2, `bits: 5`).

use std::marker::PhantomData;
use crate::math::geometric_algebra::{Blade, Component, MultiVector};

/// Trait for embedding a multivector into a higher-dimensional subspace.
pub trait Embed<T, TargetCoord, const TARGET_DIM: usize> {
    /// Mapped higher-dimensional multivector output.
    type Output;

    /// Embeds the multivector into a higher-dimensional subspace represented by `target_subspace_mask`.
    fn embed(&self, target_subspace_mask: usize) -> Self::Output;
}

/// Helper function to map source bit representations to target subspace bits.
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
        let embedded: MultiVector<f64, Cartesian, 3> = mv.embed(5);
        // e2 should map to second active position, which is index 2 (e3, bits: 4)
        assert_eq!(embedded.components[0].blade, Blade::MultiVectorBlade { bits: 4 });
    }
}
