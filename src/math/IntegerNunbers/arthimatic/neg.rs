use crate::math::IntegerNunbers::IntegerNumber;
use crate::math::operations::Neg;

impl Neg for IntegerNumber {
    type Output = IntegerNumber;

    fn neg(self) -> Self::Output {
        todo!()
    }
}
