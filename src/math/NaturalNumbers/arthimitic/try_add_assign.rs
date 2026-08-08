use crate::math::NaturalNumbers::NaturalNumber;
use crate::math::operations::TryAddAssign;
use crate::math::math_error::MathError;

impl TryAddAssign for NaturalNumber {
    type Error = MathError;

    fn try_add_assign(&mut self, other: Self) -> Result<(), Self::Error> {
        todo!()
    }
}
