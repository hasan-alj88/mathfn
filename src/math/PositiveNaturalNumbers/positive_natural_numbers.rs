use crate::math::NaturalNumbers::NaturalNumber;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PositiveNaturalNumber {
    pub value_minus_one: NaturalNumber,
}

impl PositiveNaturalNumber {
    pub fn new(value_minus_one: NaturalNumber) -> Self {
        Self { value_minus_one }
    }

    /// Creates a PositiveNaturalNumber from a standard NaturalNumber.
    /// Fails if the value is 0 (i.e. empty limbs or all zero limbs).
    pub fn from_value(value: NaturalNumber) -> Result<Self, String> {
        if value.limbs.is_empty() || value.limbs.iter().all(|&x| x == 0) {
            Err("Value must be greater than zero".to_string())
        } else {
            let one = NaturalNumber::from(1u128);
            let value_minus_one = crate::math::operations::TrySub::try_sub(value, one)
                .map_err(|_| "Failed to decrement natural number".to_string())?;
            Ok(Self { value_minus_one })
        }
    }

    /// Returns the actual mathematical NaturalNumber (value_minus_one + 1)
    pub fn to_value(&self) -> NaturalNumber {
        let one = NaturalNumber::from(1u128);
        crate::math::operations::Add::add(self.value_minus_one.clone(), one)
    }
}
