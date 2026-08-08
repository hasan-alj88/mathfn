use crate::math::IntegerNunbers::IntegerNumber;
use crate::math::operations::TrySub;
use crate::math::math_error::MathError;

impl TrySub for IntegerNumber {
    type Output = IntegerNumber;
    type Error = MathError;

    fn try_sub(self, other: Self) -> Result<Self::Output, Self::Error> {
        todo!()
    }
}
