use crate::math::base_digit::Digit;
use crate::math::math_error::MathError;

#[cfg(test)]
mod tests;

pub mod addition;
pub mod multiplication;
pub mod power;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NaturalNumber<const BASE: u128 = 256> {
    digits: Vec<Digit<BASE>>,
}

impl<const BASE: u128> NaturalNumber<BASE> {
    pub fn new(digits: Vec<Digit<BASE>>) -> Self {
        let mut num = Self { digits };
        num.normalize();
        num
    }

    pub fn from_u128(mut value: u128) -> Result<Self, MathError> {
        let mut digits = Vec::new();
        while value > 0 {
            let digit_val = value % BASE;
            digits.push(Digit::new(digit_val).map_err(|_| MathError::BaseMismatch)?);
            value /= BASE;
        }
        Ok(Self::new(digits))
    }

    pub fn to_u128(&self) -> Result<u128, MathError> {
        let mut value: u128 = 0;
        let mut power: u128 = 1;
        for digit in &self.digits {
            let term = digit.value().checked_mul(power).ok_or(MathError::QuotientOverflow)?;
            value = value.checked_add(term).ok_or(MathError::QuotientOverflow)?;
            power = power.checked_mul(BASE).ok_or(MathError::QuotientOverflow)?;
        }
        Ok(value)
    }

    fn normalize(&mut self) {
        while let Some(last) = self.digits.last() {
            if last.value() == 0 {
                self.digits.pop();
            } else {
                break;
            }
        }
    }

    pub fn is_zero(&self) -> bool {
        self.digits.is_empty()
    }

    pub fn digits(&self) -> &[Digit<BASE>] {
        &self.digits
    }
}
