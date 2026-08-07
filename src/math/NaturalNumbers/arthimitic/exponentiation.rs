use crate::math::NaturalNumbers::NaturalNumber;
use crate::math::operations::{Pow, PowAssign};

impl Pow<u32> for NaturalNumber {
    type Output = NaturalNumber;
    fn pow(self, exp: u32) -> NaturalNumber {
        let mut base = self;
        let mut res = NaturalNumber::from(1u128);
        let mut e = exp;
        while e > 0 {
            if e % 2 == 1 {
                res = res * base.clone();
            }
            base = base.clone() * base;
            e /= 2;
        }
        res
    }
}

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
