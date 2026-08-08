use crate::math::NaturalNumbers::NaturalNumber;
use crate::math::operations::Pow;

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
