use crate::math::NaturalNumbers::NaturalNumber;
use crate::math::operations::Rem;

impl Rem for NaturalNumber {
    type Output = NaturalNumber;

    fn rem(self, other: Self) -> Self::Output {
        todo!()
    }
}
