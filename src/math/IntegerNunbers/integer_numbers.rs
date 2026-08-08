use crate::math::NaturalNumbers::NaturalNumber;
use crate::math::operations::{Abs, TryAbs};
use crate::math::math_error::MathError;

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

impl Abs for IntegerNumber {
    type Output = NaturalNumber;

    fn abs(self) -> Self::Output {
        self.magnitude
    }
}

impl TryAbs for IntegerNumber {
    type Output = NaturalNumber;
    type Error = MathError;

    fn try_abs(self) -> Result<Self::Output, Self::Error> {
        Ok(self.magnitude)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_abs() {
        let n = NaturalNumber::from(100u128);
        let i = IntegerNumber {
            magnitude: n.clone(),
            sign: Sign::Negative,
        };
        assert_eq!(i.clone().abs(), n);
        assert_eq!(i.try_abs().unwrap(), n);
    }
}
