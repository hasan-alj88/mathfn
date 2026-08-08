use crate::math::NaturalNumbers::NaturalNumber;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IntegerNumber {
    pub magnitude: NaturalNumber,
    pub sign: Sign,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Sign {
    Positive,
    Negative,
}
