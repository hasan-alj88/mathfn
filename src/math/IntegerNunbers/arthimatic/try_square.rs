use crate::math::IntegerNunbers::IntegerNumber;
use crate::math::operations::TrySquare;
use crate::math::math_error::MathError;

impl TrySquare for IntegerNumber {
    type Output = IntegerNumber;
    type Error = MathError;

    fn try_square(self) -> Result<Self::Output, Self::Error> {
        todo!()
    }
}
