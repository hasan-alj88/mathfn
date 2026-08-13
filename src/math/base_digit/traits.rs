/// Represents the outcome of a digit-wise arithmetic operation.
/// - `NoOverflow(T)` represents a single-digit outcome when there is no overflow, carry, or borrow.
/// - `Overflow(low, high)` represents a two-digit outcome when there is an overflow, carry, borrow, or remainder.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DigitOutcome<T> {
    NoOverflow(T),
    Overflow(T, T),
}

/// Unified trait for digit-wise arithmetic operations.
/// All arithmetic operations return a Result with a `DigitOutcome` payload:
/// - `NoOverflow` if the operation fits in a single digit.
/// - `Overflow` if the operation produces carry, borrow, overflow, or remainder (2 digits total).
pub trait DigitOperations<Rhs = Self> {
    type Output;

    /// Adds `self`, `other`, and `carry_in`.
    fn add_digit(self, other: Rhs, carry_in: Self) -> Result<DigitOutcome<Self::Output>, crate::math::math_error::MathError>;

    /// Subtracts `other` and `borrow_in` from `self`.
    fn sub_digit(self, other: Rhs, borrow_in: Self) -> Result<DigitOutcome<Self::Output>, crate::math::math_error::MathError>;

    /// Multiplies `self` by `other` and adds `carry_in`.
    fn mul_digit(self, other: Rhs, carry_in: Self) -> Result<DigitOutcome<Self::Output>, crate::math::math_error::MathError>;

    /// Divides a double-width digit `(high, low)` by `divisor`.
    fn div_rem_digit(high: Self, low: Self, divisor: Rhs) -> Result<DigitOutcome<Self::Output>, crate::math::math_error::MathError>;
}

/// Trait for splitting a larger double-width value into a quotient and remainder digit relative to the BASE.
pub trait DigitFromDoubleWide<DoubleWide> {
    /// Divides `double_wide_value` by `BASE` and returns `(quotient_digit, remainder_digit)`.
    fn from_double_wide(double_wide_value: DoubleWide) -> (Self, Self) where Self: Sized;
}

/// Represents the result of a base conversion operation.
/// It takes generic parameters for the payloads of each conversion outcome:
/// - `E`: Exact conversion result.
/// - `R`: Infinite repeating conversion result.
/// - `N`: Infinite non-repeating conversion result.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BaseConversion<E, R, N> {
    Exact(E),
    InfiniteRepeating(R),
    InfiniteNonRepeating(N),
}
