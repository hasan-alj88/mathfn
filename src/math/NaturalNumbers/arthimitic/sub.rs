use crate::math::NaturalNumbers::NaturalNumber;
use crate::math::IntegerNunbers::integer_numbers::{IntegerNumber, Sign};
use crate::math::operations::Sub;

impl Sub for NaturalNumber {
    type Output = IntegerNumber;

    fn sub(self, other: NaturalNumber) -> Self::Output {
        if self < other {
            // Underflow subtraction yields negative integer
            let diff_limbs = crate::math::NaturalNumbers::utils::arithmetic::add_slices(&other.limbs, &[]); // skeleton placeholder
            IntegerNumber {
                magnitude: NaturalNumber::new(diff_limbs),
                sign: Sign::Negative,
            }
        } else {
            // Normal subtraction yields positive integer
            let diff_limbs = crate::math::NaturalNumbers::utils::arithmetic::add_slices(&self.limbs, &[]); // skeleton placeholder
            IntegerNumber {
                magnitude: NaturalNumber::new(diff_limbs),
                sign: Sign::Positive,
            }
        }
    }
}
