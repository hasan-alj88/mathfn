use crate::math::IntegerNunbers::IntegerNumber;
use crate::math::operations::Rem;

impl Rem for IntegerNumber {
    type Output = IntegerNumber;

    fn rem(self, other: Self) -> Self::Output {
        todo!()
    }
}
