use crate::math::NaturalNumbers::NaturalNumber;
use crate::math::operations::{Sqrt, RealNumber};

impl Sqrt for NaturalNumber {
    type Output = RealNumber;

    fn sqrt(self) -> Self::Output {
        todo!()
    }
}
