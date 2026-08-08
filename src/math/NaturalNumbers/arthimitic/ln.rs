use crate::math::NaturalNumbers::NaturalNumber;
use crate::math::operations::{Ln, RealNumber};

impl Ln for NaturalNumber {
    type Output = RealNumber;

    fn ln(self) -> Self::Output {
        todo!()
    }
}
