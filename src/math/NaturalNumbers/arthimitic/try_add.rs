use crate::math::NaturalNumbers::NaturalNumber;
use crate::math::operations::TryAdd;
use crate::math::math_error::MathError;

impl TryAdd for NaturalNumber {
    type Output = NaturalNumber;
    type Error = MathError;

    fn try_add(self, other: Self) -> Result<Self::Output, Self::Error> {
        todo!()
    }
}
