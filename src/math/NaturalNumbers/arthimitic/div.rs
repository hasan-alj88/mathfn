use crate::math::NaturalNumbers::NaturalNumber;
use crate::math::operations::{Div, RationalNumber};

impl Div for NaturalNumber {
    type Output = RationalNumber;

    fn div(self, other: Self) -> Self::Output {
        todo!()
    }
}
