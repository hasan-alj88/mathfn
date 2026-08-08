use crate::math::IntegerNunbers::IntegerNumber;
use crate::math::operations::TrySqrt;
use crate::math::math_error::MathError;

impl TrySqrt for IntegerNumber {
    type Output = IntegerNumber;
    type Error = MathError;

    fn try_sqrt(self) -> Result<Self::Output, Self::Error> {
        todo!()
    }
}
