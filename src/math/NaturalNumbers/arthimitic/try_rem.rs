use crate::math::NaturalNumbers::NaturalNumber;
use crate::math::operations::TryRem;
use crate::math::math_error::MathError;

impl TryRem for NaturalNumber {
    type Output = NaturalNumber;
    type Error = MathError;

    fn try_rem(self, other: Self) -> Result<Self::Output, Self::Error> {
        todo!()
    }
}
