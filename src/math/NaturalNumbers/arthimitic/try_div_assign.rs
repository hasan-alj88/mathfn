use crate::math::NaturalNumbers::NaturalNumber;
use crate::math::operations::TryDivAssign;
use crate::math::math_error::MathError;

impl TryDivAssign for NaturalNumber {
    type Error = MathError;

    fn try_div_assign(&mut self, other: Self) -> Result<(), Self::Error> {
        todo!()
    }
}
