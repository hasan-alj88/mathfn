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
