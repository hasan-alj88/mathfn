use crate::math::IntegerNunbers::IntegerNumber;
use crate::math::operations::TryDouble;
use crate::math::math_error::MathError;

impl TryDouble for IntegerNumber {
    type Output = IntegerNumber;
    type Error = MathError;

    fn try_double(self) -> Result<Self::Output, Self::Error> {
        todo!()
    }
}
