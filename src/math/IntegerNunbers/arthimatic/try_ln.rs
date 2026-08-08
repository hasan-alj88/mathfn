use crate::math::IntegerNunbers::IntegerNumber;
use crate::math::operations::TryLn;
use crate::math::math_error::MathError;

impl TryLn for IntegerNumber {
    type Output = IntegerNumber;
    type Error = MathError;

    fn try_ln(self) -> Result<Self::Output, Self::Error> {
        todo!()
    }
}
