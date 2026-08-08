use crate::math::IntegerNunbers::IntegerNumber;
use crate::math::operations::TryMul;
use crate::math::math_error::MathError;

impl TryMul for IntegerNumber {
    type Output = IntegerNumber;
    type Error = MathError;

    fn try_mul(self, other: Self) -> Result<Self::Output, Self::Error> {
        todo!()
    }
}
