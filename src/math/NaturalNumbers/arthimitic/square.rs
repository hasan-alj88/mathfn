use crate::math::NaturalNumbers::NaturalNumber;
use crate::math::operations::Square;

impl Square for NaturalNumber {
    type Output = NaturalNumber;

    fn square(self) -> Self::Output {
        todo!()
    }
}
