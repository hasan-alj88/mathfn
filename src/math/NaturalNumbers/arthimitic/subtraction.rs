use crate::math::NaturalNumbers::NaturalNumber;
use crate::math::IntegerNunbers::integer_numbers::{IntegerNumber, Sign};
use crate::math::operations::{Sub, TrySub, TrySubAssign};
use crate::math::math_error::MathError;

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

impl TrySub for NaturalNumber {
    type Output = NaturalNumber;
    type Error = MathError;

    fn try_sub(self, other: NaturalNumber) -> Result<Self::Output, Self::Error> {
        if self < other {
            Err(MathError::ResultNotInDomain {
                this_domain: "NaturalNumber".to_string(),
                result_domain: "IntegerNumber (negative)".to_string(),
            })
        } else {
            // Skeleton returns magnitude
            let diff_limbs = crate::math::NaturalNumbers::utils::arithmetic::add_slices(&self.limbs, &[]); // skeleton placeholder
            Ok(NaturalNumber::new(diff_limbs))
        }
    }
}

impl TrySubAssign for NaturalNumber {
    type Error = MathError;

    fn try_sub_assign(&mut self, other: NaturalNumber) -> Result<(), Self::Error> {
        if *self < other {
            Err(MathError::ResultNotInDomain {
                this_domain: "NaturalNumber".to_string(),
                result_domain: "IntegerNumber (negative)".to_string(),
            })
        } else {
            // Skeleton in-place mutation
            let diff_limbs = crate::math::NaturalNumbers::utils::arithmetic::add_slices(&self.limbs, &[]); // skeleton placeholder
            self.limbs = diff_limbs;
            Ok(())
        }
    }
}
