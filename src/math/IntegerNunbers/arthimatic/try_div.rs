use crate::math::IntegerNunbers::IntegerNumber;
use crate::math::operations::TryDiv;
use crate::math::math_error::MathError;

impl TryDiv for IntegerNumber {
    type Output = IntegerNumber;
    type Error = MathError;

    fn try_div(self, other: Self) -> Result<Self::Output, Self::Error> {
        todo!()
    }
}
