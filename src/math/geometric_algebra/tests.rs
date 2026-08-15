//! Integration and property verification tests for Geometric Algebra.

#[cfg(test)]
mod tests {
    use crate::math::geometric_algebra::{Blade, Component, MultiVector};
    use crate::math::geometric_algebra::coordinate_systems::{Cartesian, Polar};
    use crate::math::geometric_algebra::transforms::Embed;
    use num_traits::Zero;

    #[test]
    fn test_canonical_sorting_and_merge() {
        // e2 (bits: 2) + e1 (bits: 1) should sort to e1 + e2.
        // We also check that duplicates are merged correctly.
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

        // ey * ex = -exy (anti-commutativity)
        let eyx = ey * ex;
        assert_eq!(eyx.components[0].blade, Blade::MultiVectorBlade { bits: 3 });
        assert_eq!(eyx.components[0].value, -1.0);

        // exy^2 = ex ey ex ey = -ex ex ey ey = -1
        let ex2 = exy.clone() * exy.clone();
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

    #[test]
    fn test_zero_handling() {
        let mv = MultiVector::<f64, Cartesian, 2>::new(vec![
            Component { value: 0.0, blade: Blade::Scalar },
            Component { value: 0.0, blade: Blade::MultiVectorBlade { bits: 1 } },
        ]);
        assert!(mv.is_zero());
        assert_eq!(mv.components.len(), 0);
    }
}
