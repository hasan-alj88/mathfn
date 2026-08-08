use crate::math::NaturalNumbers::NaturalNumber;
use crate::math::operations::{Pow, PowAssign};

impl PowAssign<u32> for NaturalNumber {
    fn pow_assign(&mut self, exp: u32) {
        *self = self.clone().pow(exp);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exponentiation() {
        let a = NaturalNumber::from(3u128);
        let b = a.pow(4); // 3^4 = 81. limbs should be [81]
        assert_eq!(b.limbs, vec![81u128]);
    }

    #[test]
    fn test_pow_assign() {
        let mut a = NaturalNumber::from(2u128);
        a.pow_assign(5); // 2^5 = 32
        assert_eq!(a.limbs, vec![32u128]);
    }
}
