use crate::math::NaturalNumbers::NaturalNumber;
use crate::math::PositiveNaturalNumbers::PositiveNaturalNumber;
use crate::math::Sign;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RationalNumber {
    pub numerator: NaturalNumber,
    pub denominator: PositiveNaturalNumber,
    pub sign: Sign,
}

impl RationalNumber {
    pub fn new(numerator: NaturalNumber, denominator: PositiveNaturalNumber, sign: Sign) -> Self {
        Self { numerator, denominator, sign }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rational_construction() {
        let num = NaturalNumber::from(5u128);
        let den_val = NaturalNumber::from(10u128);
        let den = PositiveNaturalNumber::new(NaturalNumber::from(9u128));
        
        let rat = RationalNumber::new(num.clone(), den.clone(), Sign::Positive);
        assert_eq!(rat.numerator, num);
        assert_eq!(rat.denominator, den);
        assert_eq!(rat.sign, Sign::Positive);

        // Check value mapping in PositiveNaturalNumber
        assert_eq!(den.to_value(), den_val);
    }
}
