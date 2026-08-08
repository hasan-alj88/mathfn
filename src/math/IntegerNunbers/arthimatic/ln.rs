use crate::math::IntegerNunbers::IntegerNumber;
use crate::math::operations::{Ln, RealNumber};

impl Ln for IntegerNumber {
    type Output = RealNumber;

    fn ln(self) -> Self::Output {
        todo!()
    }
}
