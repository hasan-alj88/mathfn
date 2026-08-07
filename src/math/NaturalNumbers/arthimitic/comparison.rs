use crate::math::NaturalNumbers::NaturalNumber;
use std::cmp::Ordering;

impl PartialOrd for NaturalNumber {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for NaturalNumber {
    fn cmp(&self, other: &Self) -> Ordering {
        let length_cmp = self.limbs.len().cmp(&other.limbs.len());
        match length_cmp {
            Ordering::Equal => self
                .limbs
                .iter()
                .rev()
                .zip(other.limbs.iter())
                .rev()
                .find(|(a, b)| a != b)
                .map(|(a, b)| a.cmp(b))
                .unwrap_or(Ordering::Equal),
            _ => length_cmp,
        }
    }
}

impl PartialEq for NaturalNumber {
    fn eq(&self, other: &Self) -> bool {
        self.limbs == other.limbs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_comparison() {
        let a = NaturalNumber::from(100u128);
        let b = NaturalNumber::from(200u128);
        let c = NaturalNumber::from(100u128);

        assert!(a < b);
        assert!(b > a);
        assert_eq!(a, c);
        assert_ne!(a, b);
    }
}
