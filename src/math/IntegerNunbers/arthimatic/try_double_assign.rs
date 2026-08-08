use crate::math::IntegerNunbers::IntegerNumber;
use crate::math::operations::TryDoubleAssign;
use crate::math::math_error::MathError;

impl TryDoubleAssign for IntegerNumber {
    type Error = MathError;

    fn try_double_assign(&mut self) -> Result<(), Self::Error> {
        todo!()
    }
}
