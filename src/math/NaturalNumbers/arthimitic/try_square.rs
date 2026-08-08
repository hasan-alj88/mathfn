use crate::math::NaturalNumbers::NaturalNumber;
use crate::math::operations::TrySquare;
use crate::math::math_error::MathError;

impl TrySquare for NaturalNumber {
    type Output = NaturalNumber;
    type Error = MathError;

    fn try_square(self) -> Result<Self::Output, Self::Error> {
        todo!()
    }
}
