use crate::math::IntegerNunbers::IntegerNumber;
use crate::math::operations::TryRem;
use crate::math::math_error::MathError;

impl TryRem for IntegerNumber {
    type Output = IntegerNumber;
    type Error = MathError;

    fn try_rem(self, other: Self) -> Result<Self::Output, Self::Error> {
        todo!()
    }
}
