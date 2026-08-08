use crate::math::IntegerNunbers::IntegerNumber;
use crate::math::operations::TryLog;
use crate::math::math_error::MathError;

impl TryLog for IntegerNumber {
    type Output = IntegerNumber;
    type Error = MathError;

    fn try_log(self, base: Self) -> Result<Self::Output, Self::Error> {
        todo!()
    }
}
