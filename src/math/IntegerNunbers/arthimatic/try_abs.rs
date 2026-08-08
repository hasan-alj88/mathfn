use crate::math::IntegerNunbers::IntegerNumber;
use crate::math::NaturalNumbers::NaturalNumber;
use crate::math::operations::TryAbs;
use crate::math::math_error::MathError;

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
    use crate::math::IntegerNunbers::Sign;
    use crate::math::operations::Abs;

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
