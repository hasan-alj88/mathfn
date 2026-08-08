use crate::math::IntegerNunbers::IntegerNumber;
use crate::math::operations::TryAdd;
use crate::math::math_error::MathError;

impl TryAdd for IntegerNumber {
    type Output = IntegerNumber;
    type Error = MathError;

    fn try_add(self, other: Self) -> Result<Self::Output, Self::Error> {
        todo!()
    }
}
