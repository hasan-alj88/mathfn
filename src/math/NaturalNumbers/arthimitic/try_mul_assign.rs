use crate::math::NaturalNumbers::NaturalNumber;
use crate::math::operations::TryMulAssign;
use crate::math::math_error::MathError;

impl TryMulAssign for NaturalNumber {
    type Error = MathError;

    fn try_mul_assign(&mut self, other: Self) -> Result<(), Self::Error> {
        todo!()
    }
}
