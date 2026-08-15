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
