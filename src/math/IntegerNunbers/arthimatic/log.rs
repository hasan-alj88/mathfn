use crate::math::IntegerNunbers::IntegerNumber;
use crate::math::operations::{Log, RealNumber};

impl Log for IntegerNumber {
    type Output = RealNumber;

    fn log(self, base: Self) -> Self::Output {
        todo!()
    }
}
