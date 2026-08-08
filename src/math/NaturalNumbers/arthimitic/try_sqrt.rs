use crate::math::NaturalNumbers::NaturalNumber;
use crate::math::operations::TrySqrt;
use crate::math::math_error::MathError;

impl TrySqrt for NaturalNumber {
    type Output = NaturalNumber;
    type Error = MathError;

    fn try_sqrt(self) -> Result<Self::Output, Self::Error> {
        todo!()
    }
}
