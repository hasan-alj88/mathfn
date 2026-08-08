use crate::math::NaturalNumbers::NaturalNumber;
use crate::math::operations::TryLn;
use crate::math::math_error::MathError;

impl TryLn for NaturalNumber {
    type Output = NaturalNumber;
    type Error = MathError;

    fn try_ln(self) -> Result<Self::Output, Self::Error> {
        todo!()
    }
}
