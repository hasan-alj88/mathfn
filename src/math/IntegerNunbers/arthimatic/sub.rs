use crate::math::IntegerNunbers::IntegerNumber;
use crate::math::operations::Sub;

impl Sub for IntegerNumber {
    type Output = IntegerNumber;

    fn sub(self, other: Self) -> Self::Output {
        todo!()
    }
}
