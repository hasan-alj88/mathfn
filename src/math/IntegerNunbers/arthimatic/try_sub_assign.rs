use crate::math::IntegerNunbers::IntegerNumber;
use crate::math::operations::TrySubAssign;
use crate::math::math_error::MathError;

impl TrySubAssign for IntegerNumber {
    type Error = MathError;

    fn try_sub_assign(&mut self, other: Self) -> Result<(), Self::Error> {
        todo!()
    }
}
