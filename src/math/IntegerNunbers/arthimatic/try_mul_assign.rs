use crate::math::IntegerNunbers::IntegerNumber;
use crate::math::operations::TryMulAssign;
use crate::math::math_error::MathError;

impl TryMulAssign for IntegerNumber {
    type Error = MathError;

    fn try_mul_assign(&mut self, other: Self) -> Result<(), Self::Error> {
        todo!()
    }
}
