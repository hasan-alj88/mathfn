use crate::math::NaturalNumbers::NaturalNumber;
use crate::math::operations::TrySquareAssign;
use crate::math::math_error::MathError;

impl TrySquareAssign for NaturalNumber {
    type Error = MathError;

    fn try_square_assign(&mut self) -> Result<(), Self::Error> {
        todo!()
    }
}
