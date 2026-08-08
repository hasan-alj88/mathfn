use crate::math::NaturalNumbers::NaturalNumber;
use crate::math::operations::TryPowAssign;
use crate::math::math_error::MathError;

impl TryPowAssign<u32> for NaturalNumber {
    type Error = MathError;

    fn try_pow_assign(&mut self, exp: u32) -> Result<(), Self::Error> {
        todo!()
    }
}
