use crate::math::IntegerNunbers::IntegerNumber;
use crate::math::operations::Pow;

impl Pow<u32> for IntegerNumber {
    type Output = IntegerNumber;

    fn pow(self, exp: u32) -> Self::Output {
        todo!()
    }
}
