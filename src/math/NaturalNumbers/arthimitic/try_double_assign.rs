use crate::math::NaturalNumbers::NaturalNumber;
use crate::math::operations::TryDoubleAssign;
use crate::math::math_error::MathError;

impl TryDoubleAssign for NaturalNumber {
    type Error = MathError;

    fn try_double_assign(&mut self) -> Result<(), Self::Error> {
        todo!()
    }
}
