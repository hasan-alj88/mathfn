use crate::math::IntegerNunbers::IntegerNumber;
use crate::math::NaturalNumbers::NaturalNumber;
use crate::math::operations::Abs;

impl Abs for IntegerNumber {
    type Output = NaturalNumber;

    fn abs(self) -> Self::Output {
        self.magnitude
    }
}
