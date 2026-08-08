use crate::math::IntegerNunbers::IntegerNumber;
use crate::math::operations::TryPow;
use crate::math::math_error::MathError;

impl TryPow<u32> for IntegerNumber {
    type Output = IntegerNumber;
    type Error = MathError;

    fn try_pow(self, exp: u32) -> Result<Self::Output, Self::Error> {
        todo!()
    }
}
