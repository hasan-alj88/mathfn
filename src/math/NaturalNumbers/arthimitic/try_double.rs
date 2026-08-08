use crate::math::NaturalNumbers::NaturalNumber;
use crate::math::operations::TryDouble;
use crate::math::math_error::MathError;

impl TryDouble for NaturalNumber {
    type Output = NaturalNumber;
    type Error = MathError;

    fn try_double(self) -> Result<Self::Output, Self::Error> {
        todo!()
    }
}
