use crate::math::IntegerNunbers::IntegerNumber;
use crate::math::operations::{Div, RationalNumber};

impl Div for IntegerNumber {
    type Output = RationalNumber;

    fn div(self, other: Self) -> Self::Output {
        todo!()
    }
}
