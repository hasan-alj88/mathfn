use crate::math::natural_number::NaturalNumber;
use crate::math::natural_number::addition::nat_add_schoolbook;
use crate::math::natural_number::multiplication::nat_sub_schoolbook;
use crate::math::math_error::MathError;

/// A positive natural number (Z^+ = {1, 2, 3, ...}).
/// Zero is not representable in this type.
///
/// ### Positional Representation
/// To store values starting from 1, the internal representation is offset by -1.
/// The value of `PositiveNaturalNumber(N)` is mathematically equal to `N + 1`.
/// - The value 1 is represented by an internal `NaturalNumber` of 0 (empty digits).
/// - The value 2 is represented by an internal `NaturalNumber` of 1.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PositiveNaturalNumber<const BASE: u128 = 256>(NaturalNumber<BASE>);

impl<const BASE: u128> PositiveNaturalNumber<BASE> {
    pub fn new_raw(internal_val: NaturalNumber<BASE>) -> Self {
        Self(internal_val)
    }

    pub fn offset_val(&self) -> &NaturalNumber<BASE> {
        &self.0
    }
}

impl<const BASE: u128> TryFrom<u128> for PositiveNaturalNumber<BASE> {
    type Error = MathError;
    fn try_from(value: u128) -> Result<Self, Self::Error> {
        match value {
            0 => Err(MathError::ResultNotInDomain {
                this_domain: "PositiveNaturalNumbers".to_string(),
                result_domain: "Zero".to_string(),
            }),
            _ => {
                let internal_num = NaturalNumber::from_u128(value - 1)?;
                Ok(Self(internal_num))
            }
        }
    }
}

impl<const BASE: u128> TryFrom<PositiveNaturalNumber<BASE>> for u128 {
    type Error = MathError;
    fn try_from(num: PositiveNaturalNumber<BASE>) -> Result<Self, Self::Error> {
        let internal_val = num.0.to_u128()?;
        internal_val.checked_add(1).ok_or(MathError::QuotientOverflow)
    }
}

impl<const BASE: u128> TryFrom<NaturalNumber<BASE>> for PositiveNaturalNumber<BASE> {
    type Error = MathError;
    fn try_from(num: NaturalNumber<BASE>) -> Result<Self, Self::Error> {
        match num.is_zero() {
            true => Err(MathError::ResultNotInDomain {
                this_domain: "PositiveNaturalNumbers".to_string(),
                result_domain: "Zero".to_string(),
            }),
            false => {
                let one = NaturalNumber::from_u128(1)?;
                let offset_num = nat_sub_schoolbook(&num, &one)?;
                Ok(Self(offset_num))
            }
        }
    }
}

impl<const BASE: u128> From<PositiveNaturalNumber<BASE>> for NaturalNumber<BASE> {
    fn from(num: PositiveNaturalNumber<BASE>) -> Self {
        let one = NaturalNumber::from_u128(1).unwrap();
        nat_add_schoolbook(&num.0, &one).unwrap()
    }
}
