use crate::math::IntegerNunbers::IntegerNumber;
use crate::math::operations::Mul;

impl Mul for IntegerNumber {
    type Output = IntegerNumber;

    fn mul(self, other: Self) -> Self::Output {
        todo!()
    }
}
