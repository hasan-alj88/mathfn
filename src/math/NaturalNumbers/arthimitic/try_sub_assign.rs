use crate::math::NaturalNumbers::NaturalNumber;
use crate::math::operations::TrySubAssign;
use crate::math::math_error::MathError;

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
