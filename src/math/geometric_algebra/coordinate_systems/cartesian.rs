//! Cartesian coordinate system.

use super::CoordinateSystem;

/// Cartesian Coordinate System.
///
/// Labels basis vector directions as standard spatial dimensions:
/// - index 0 -> "x"
/// - index 1 -> "y"
/// - index 2 -> "z"
/// - index >= 3 -> "w"
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
