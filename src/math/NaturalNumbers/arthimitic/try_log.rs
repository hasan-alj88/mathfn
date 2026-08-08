use crate::math::NaturalNumbers::NaturalNumber;
use crate::math::operations::TryLog;
use crate::math::math_error::MathError;

impl TryLog for NaturalNumber {
    type Output = NaturalNumber;
    type Error = MathError;

    fn try_log(self, base: Self) -> Result<Self::Output, Self::Error> {
        todo!()
    }
}
