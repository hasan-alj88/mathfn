use crate::math::NaturalNumbers::NaturalNumber;

#[derive(Clone, Debug, Eq)]
pub struct IntegerNumber {
    pub magnitude: NaturalNumber,
    pub sign: Sign,
}

#[derive(Clone, Debug, Eq)]
pub enum Sign {
    Positive,
    Negative,
}
