use crate::math::IntegerNunbers::IntegerNumber;
use crate::math::operations::TryAddAssign;
use crate::math::math_error::MathError;

impl TryAddAssign for IntegerNumber {
    type Error = MathError;

    fn try_add_assign(&mut self, other: Self) -> Result<(), Self::Error> {
        todo!()
    }
}
