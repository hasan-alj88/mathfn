use super::{IntegerNumber, Sign};
use crate::math::operations::{Add, AddAssign};

impl Add for IntegerNumber {
    type Output = IntegerNumber;
    fn add(self, other: IntegerNumber) -> IntegerNumber {
        todo!()
    }
}
