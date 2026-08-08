use crate::math::IntegerNunbers::IntegerNumber;
use crate::math::operations::Double;

impl Double for IntegerNumber {
    type Output = IntegerNumber;

    fn double(self) -> Self::Output {
        todo!()
    }
}
