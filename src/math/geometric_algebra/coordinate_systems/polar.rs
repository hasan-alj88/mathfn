//! Polar coordinate system.

use super::CoordinateSystem;

/// Polar Coordinate System.
///
/// Labels basis vector directions as radial and angular dimensions:
/// - index 0 -> "r"
/// - index 1 -> "θ" (theta)
/// - index >= 2 -> "φ" (phi)
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

impl From<crate::math::geometric_algebra::MultiVector<f64, super::Cartesian, 2>> for crate::math::geometric_algebra::MultiVector<f64, Polar, 2> {
    fn from(src: crate::math::geometric_algebra::MultiVector<f64, super::Cartesian, 2>) -> Self {
        use crate::math::geometric_algebra::{Blade, Component, MultiVector};
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

impl From<crate::math::geometric_algebra::MultiVector<f64, Polar, 2>> for crate::math::geometric_algebra::MultiVector<f64, super::Cartesian, 2> {
    fn from(src: crate::math::geometric_algebra::MultiVector<f64, Polar, 2>) -> Self {
        use crate::math::geometric_algebra::{Blade, Component, MultiVector};
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
