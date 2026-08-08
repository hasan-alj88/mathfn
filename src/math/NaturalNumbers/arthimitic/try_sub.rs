use crate::math::NaturalNumbers::NaturalNumber;
use crate::math::operations::TrySub;
use crate::math::math_error::MathError;

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
