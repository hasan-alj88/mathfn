use crate::math::NaturalNumbers::NaturalNumber;
use crate::math::operations::TryDiv;
use crate::math::math_error::MathError;

impl TryDiv for NaturalNumber {
    type Output = NaturalNumber;
    type Error = MathError;

    fn try_div(self, other: Self) -> Result<Self::Output, Self::Error> {
        todo!()
    }
}
