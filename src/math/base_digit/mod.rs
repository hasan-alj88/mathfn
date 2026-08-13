pub mod traits;
pub mod digit;
pub mod primitives;
#[cfg(test)]
mod tests;

pub use traits::{DigitOperations, DigitFromDoubleWide, BaseConversion, DigitOutcome};
pub use digit::Digit;
