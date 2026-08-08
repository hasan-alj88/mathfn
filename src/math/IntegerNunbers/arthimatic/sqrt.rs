use crate::math::IntegerNunbers::IntegerNumber;
use crate::math::operations::{Sqrt, RealNumber};

impl Sqrt for IntegerNumber {
    type Output = RealNumber;

    fn sqrt(self) -> Self::Output {
        todo!()
    }
}
