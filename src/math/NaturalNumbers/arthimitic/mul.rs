use crate::math::NaturalNumbers::NaturalNumber;
use crate::math::operations::Mul;

impl Mul for NaturalNumber {
    type Output = NaturalNumber;

    fn mul(mut self, other: NaturalNumber) -> NaturalNumber {
        self *= other;
        self
    }
}
