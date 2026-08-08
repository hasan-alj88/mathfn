use crate::math::NaturalNumbers::NaturalNumber;
use crate::math::operations::Double;

impl Double for NaturalNumber {
    type Output = NaturalNumber;

    fn double(self) -> Self::Output {
        todo!()
    }
}
