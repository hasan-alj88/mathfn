use crate::math::NaturalNumbers::NaturalNumber;
use crate::math::operations::{Log, RealNumber};

impl Log for NaturalNumber {
    type Output = RealNumber;

    fn log(self, base: Self) -> Self::Output {
        todo!()
    }
}
