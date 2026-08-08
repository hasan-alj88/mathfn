use crate::math::NaturalNumbers::NaturalNumber;
use crate::math::Sign;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IntegerNumber {
    pub magnitude: NaturalNumber,
    pub sign: Sign,
}
