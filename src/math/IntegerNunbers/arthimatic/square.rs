use crate::math::IntegerNunbers::IntegerNumber;
use crate::math::operations::Square;

impl Square for IntegerNumber {
    type Output = IntegerNumber;

    fn square(self) -> Self::Output {
        todo!()
    }
}
