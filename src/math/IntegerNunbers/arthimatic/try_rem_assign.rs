use crate::math::IntegerNunbers::IntegerNumber;
use crate::math::operations::TryRemAssign;
use crate::math::math_error::MathError;

impl TryRemAssign for IntegerNumber {
    type Error = MathError;

    fn try_rem_assign(&mut self, other: Self) -> Result<(), Self::Error> {
        todo!()
    }
}
