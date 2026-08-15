//! Coordinate system typestates for Geometric Algebra.
//!
//! Provides the [`CoordinateSystem`] trait which allows different coordinate systems
//! (e.g., [`Cartesian`], [`Polar`]) to define axis labels for their basis dimensions.

pub mod cartesian;
pub mod polar;

pub use cartesian::Cartesian;
pub use polar::Polar;

/// Defines coordinate labels and formatting for a multivector space.
///
/// Implementations of this trait are zero-sized marker types that bind the coordinate
/// system at compile-time and map numeric basis bit positions to human-readable strings.
pub trait CoordinateSystem {
    /// Returns the string label for a specific basis vector index.
    ///
    /// - For Cartesian: index 0 -> "x", index 1 -> "y", index 2 -> "z", etc.
    /// - For Polar: index 0 -> "r", index 1 -> "θ", etc.
    fn axis_label(position: usize) -> &'static str;
}
