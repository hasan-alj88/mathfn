use crate::math::NaturalNumbers::NaturalNumber;
use crate::math::IntegerNunbers::IntegerNumber;
use crate::math::operations::Neg;

impl Neg for NaturalNumber {
    type Output = IntegerNumber;

    fn neg(self) -> Self::Output {
        todo!()
    }
}
