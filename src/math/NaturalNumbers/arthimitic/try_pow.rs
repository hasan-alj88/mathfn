use crate::math::NaturalNumbers::NaturalNumber;
use crate::math::operations::TryPow;
use crate::math::math_error::MathError;

impl TryPow<u32> for NaturalNumber {
    type Output = NaturalNumber;
    type Error = MathError;

    fn try_pow(self, exp: u32) -> Result<Self::Output, Self::Error> {
        todo!()
    }
}
