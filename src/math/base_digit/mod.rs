pub mod traits;

pub use traits::{DigitAdd, DigitSub, DigitMul, DigitDivRem, DigitFromWide};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BaseDigit<const BASE: u128 = 0> {
    Binary(u8),            // Base 2
    Quaternary(u8),        // Base 4
    Octal(u8),             // Base 8
    Decimal(u8),           // Base 10
    Dezonal(u8),           // Base 12
    Hexadecimal(u8),       // Base 16
    Octet(u8),             // Base 256
    Doublet(u16),          // Base 65536
    Quadlet(u32),          // Base 2^32
    Octlet(u64),           // Base 2^64
    DoubleOctlet(u128),    // Base 2^128 (default)
    Other(u128),           // Any other base
}

impl<const BASE: u128> BaseDigit<BASE> {
    /// Creates a new BaseDigit. Returns Err if value >= BASE (for non-2^128 bases).
    pub fn new(value: u128) -> Result<Self, &'static str> {
        if BASE == 1 {
            return Err("Base cannot be 1");
        }
        if BASE != 0 && value >= BASE {
            return Err("Digit value too high for the base");
        }

        Ok(match BASE {
            2 => Self::Binary(value as u8),
            4 => Self::Quaternary(value as u8),
            8 => Self::Octal(value as u8),
            10 => Self::Decimal(value as u8),
            12 => Self::Dezonal(value as u8),
            16 => Self::Hexadecimal(value as u8),
            256 => Self::Octet(value as u8),
            65536 => Self::Doublet(value as u16),
            4294967296 => Self::Quadlet(value as u32),
            18446744073709551616 => Self::Octlet(value as u64),
            0 => Self::DoubleOctlet(value),
            _ => Self::Other(value),
        })
    }

    /// Returns the inner value of the digit as a u128.
    pub fn value(self) -> u128 {
        match self {
            Self::Binary(v) => v as u128,
            Self::Quaternary(v) => v as u128,
            Self::Octal(v) => v as u128,
            Self::Decimal(v) => v as u128,
            Self::Dezonal(v) => v as u128,
            Self::Hexadecimal(v) => v as u128,
            Self::Octet(v) => v as u128,
            Self::Doublet(v) => v as u128,
            Self::Quadlet(v) => v as u128,
            Self::Octlet(v) => v as u128,
            Self::DoubleOctlet(v) => v,
            Self::Other(v) => v,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base_digit_creation() {
        // Default base (BASE = 0, represents 2^128)
        let default_digit = BaseDigit::<0>::new(u128::MAX).unwrap();
        assert_eq!(default_digit.value(), u128::MAX);
        assert!(matches!(default_digit, BaseDigit::<0>::DoubleOctlet(_)));

        // Base 10 (Decimal)
        let dec_digit = BaseDigit::<10>::new(9).unwrap();
        assert_eq!(dec_digit.value(), 9);
        assert!(matches!(dec_digit, BaseDigit::Decimal(_)));
        assert!(BaseDigit::<10>::new(10).is_err());

        // Base 12 (Dezonal)
        let dez_digit = BaseDigit::<12>::new(11).unwrap();
        assert_eq!(dez_digit.value(), 11);
        assert!(matches!(dez_digit, BaseDigit::Dezonal(_)));
        assert!(BaseDigit::<12>::new(12).is_err());

        // Other custom base (e.g. Base 50)
        let custom_digit = BaseDigit::<50>::new(49).unwrap();
        assert_eq!(custom_digit.value(), 49);
        assert!(matches!(custom_digit, BaseDigit::Other(_)));
        assert!(BaseDigit::<50>::new(50).is_err());
    }
}
