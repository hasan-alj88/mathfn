use crate::math::base_digit::Digit;
use crate::math::math_error::MathError;

pub trait NumberType<const BASE: u128> {
    fn digit(&self, pos: i64) -> Result<Digit<BASE>, MathError>;
}
