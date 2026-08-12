#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct BaseDigit<const BASE: u128 = 256>;

impl<const BASE: u128> BaseDigit<BASE> {
    // Add your fields, storage enum, and methods here
}

/// Trait for digit-wise addition with carry.
pub trait DigitAdd<Rhs = Self> {
    type Output;
    /// Adds `self`, `other`, and a `carry_in`, returning `(sum, carry_out)`.
    fn add_digit(self, other: Rhs, carry_in: Self) -> (Self::Output, Self::Output);
}

/// Trait for digit-wise subtraction with borrow.
pub trait DigitSub<Rhs = Self> {
    type Output;
    /// Subtracts `other` and a `borrow_in` from `self`, returning `(diff, borrow_out)`.
    fn sub_digit(self, other: Rhs, borrow_in: Self) -> (Self::Output, Self::Output);
}

/// Trait for digit-wise multiplication with carry.
pub trait DigitMul<Rhs = Self> {
    type Output;
    /// Multiplies `self` by `other` and adds `carry_in`, returning `(low_digit, high_digit)`.
    fn mul_digit(self, other: Rhs, carry_in: Self) -> (Self::Output, Self::Output);
}

/// Trait for digit-wise division and remainder of a double-width digit by a single-width divisor.
pub trait DigitDivRem<Rhs = Self> {
    type Output;
    /// Divides a double-width digit `(high, low)` by `divisor`, returning `(quotient, remainder)`.
    fn div_rem_digit(high: Self, low: Self, divisor: Rhs) -> (Self::Output, Self::Output);
}

/// Trait for splitting a larger double-width value (or wide representation) into a quotient and remainder digit relative to the BASE.
pub trait DigitFromWide<Wide> {
    /// Divides `wide_value` by `BASE` and returns `(quotient_digit, remainder_digit)`.
    fn from_wide(wide_value: Wide) -> (Self, Self) where Self: Sized;
}
