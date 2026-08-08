use crate::math::NaturalNumbers::NaturalNumber;
use crate::math::operations::TryMul;
use crate::math::math_error::MathError;

impl TryMul for NaturalNumber {
    type Output = NaturalNumber;
    type Error = MathError;

    fn try_mul(self, other: Self) -> Result<Self::Output, Self::Error> {
        todo!()
    }
}
