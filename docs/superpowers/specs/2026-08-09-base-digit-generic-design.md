# Design Spec: Stable Generic BaseDigit and NaturalNumber with Hybrid Storage

Introduce a generic `BaseDigit` struct parameterized by a const generic `BASE` parameter, using a hybrid stack/heap storage enum `DigitStorage` to avoid heap allocations for common bases (storing 1, 2, 4, or 8 byte digits inline) while falling back to a heap-allocated `Vec<u8>` for larger bases. This design compiles on stable Rust.

Additionally, define convenient type aliases for commonly used bases, specifically Base 10 (decimal) and Base 16 (hexadecimal).

## Goal
1. Implement a hybrid storage enum `DigitStorage` containing variants for `[u8; 1]`, `[u8; 2]`, `[u8; 4]`, `[u8; 8]`, and a fallback `Vec<u8>`.
2. Define `BaseDigit<const BASE: u128 = 256>` wrapping `DigitStorage`.
3. Transition `NaturalNumber` to be parameterized by `const BASE: u128 = 256`, utilizing `digits: Vec<BaseDigit<BASE>>` instead of `limbs: Vec<u128>`.
4. Propagate the generic `BASE` parameter to `IntegerNumber`, `PositiveNaturalNumber`, and `RationalNumber`.
5. Expose type aliases for Base 10 and Base 16 versions of all number types (e.g. `NaturalNumber10`, `NaturalNumber16`).
6. Implement `std::fmt::Display` for all number types with custom base formatting.
7. Implement a base conversion method `.convert_base::<const NEW_BASE: u128>()` using target-base arithmetic.

## Proposed Architecture

### 1. Core Types

#### `DigitStorage` and `BaseDigit` (in `src/math/NaturalNumbers/base_digit.rs`)
```rust
use std::fmt;

pub const fn bytes_needed_for_base(base: u128) -> usize {
    if base <= 1 {
        return 1;
    }
    let max_val = base - 1;
    let bits = 128 - max_val.leading_zeros();
    ((bits + 7) / 8) as usize
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum DigitStorage {
    One([u8; 1]),
    Two([u8; 2]),
    Four([u8; 4]),
    Eight([u8; 8]),
    Large(Vec<u8>),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct BaseDigit<const BASE: u128 = 256>(pub(crate) DigitStorage);

impl<const BASE: u128> BaseDigit<BASE> {
    pub fn new(bytes: &[u8]) -> Option<Self> {
        let expected_len = bytes_needed_for_base(BASE);
        if bytes.len() != expected_len {
            return None;
        }

        // Verify value < BASE
        let mut val = 0u128;
        for (i, &byte) in bytes.iter().enumerate() {
            if i >= 16 {
                return None; // Exceeds u128 limits
            }
            val |= (byte as u128) << (i * 8);
        }
        if val >= BASE {
            return None;
        }

        let storage = match expected_len {
            1 => DigitStorage::One([bytes[0]]),
            2 => DigitStorage::Two([bytes[0], bytes[1]]),
            3 | 4 => {
                let mut arr = [0u8; 4];
                arr[..bytes.len()].copy_from_slice(bytes);
                DigitStorage::Four(arr)
            }
            5..=8 => {
                let mut arr = [0u8; 8];
                arr[..bytes.len()].copy_from_slice(bytes);
                DigitStorage::Eight(arr)
            }
            _ => DigitStorage::Large(bytes.to_vec()),
        };

        Some(Self(storage))
    }

    pub fn value(&self) -> Vec<u8> {
        match &self.0 {
            DigitStorage::One(arr) => arr.to_vec(),
            DigitStorage::Two(arr) => arr.to_vec(),
            DigitStorage::Four(arr) => {
                let len = bytes_needed_for_base(BASE);
                arr[..len].to_vec()
            }
            DigitStorage::Eight(arr) => {
                let len = bytes_needed_for_base(BASE);
                arr[..len].to_vec()
            }
            DigitStorage::Large(vec) => vec.clone(),
        }
    }
}

impl<const BASE: u128> fmt::Display for BaseDigit<BASE> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bytes = self.value();
        let mut val = 0u128;
        for (i, &byte) in bytes.iter().enumerate() {
            val |= (byte as u128) << (i * 8);
        }
        if BASE <= 36 {
            let char_digit = if val < 10 {
                (b'0' + val as u8) as char
            } else {
                (b'a' + (val - 10) as u8) as char
            };
            write!(f, "{}_{}", char_digit, BASE)
        } else {
            write!(f, "{}_{}", val, BASE)
        }
    }
}
```

#### `NaturalNumber` (in `src/math/NaturalNumbers/naturalnumbers.rs`)
```rust
use crate::math::NaturalNumbers::base_digit::BaseDigit;
use std::fmt;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NaturalNumber<const BASE: u128 = 256> {
    pub digits: Vec<BaseDigit<BASE>>,
}

impl<const BASE: u128> NaturalNumber<BASE> {
    pub fn new(digits: Vec<BaseDigit<BASE>>) -> Self {
        Self { digits }
    }
}

// Common base type aliases
pub type NaturalNumber10 = NaturalNumber<10>;
pub type NaturalNumber16 = NaturalNumber<16>;
```

### 2. Display Implementations

#### `NaturalNumber<BASE>`
```rust
impl<const BASE: u128> fmt::Display for NaturalNumber<BASE> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.digits.is_empty() {
            return write!(f, "0_{}", BASE);
        }

        if BASE <= 36 {
            let mut s = String::new();
            for digit in self.digits.iter().rev() {
                let bytes = digit.value();
                let mut val = 0u128;
                for (i, &byte) in bytes.iter().enumerate() {
                    val |= (byte as u128) << (i * 8);
                }
                let char_digit = if val < 10 {
                    (b'0' + val as u8) as char
                } else {
                    (b'a' + (val - 10) as u8) as char
                };
                s.push(char_digit);
            }
            write!(f, "{}_{}", s, BASE)
        } else {
            let mut parts = Vec::new();
            for digit in self.digits.iter().rev() {
                let bytes = digit.value();
                let mut val = 0u128;
                for (i, &byte) in bytes.iter().enumerate() {
                    val |= (byte as u128) << (i * 8);
                }
                parts.push(val.to_string());
            }
            write!(f, "[{}]_{}", parts.join(", "), BASE)
        }
    }
}
```

#### `IntegerNumber<BASE>` (in `src/math/IntegerNunbers/integer_numbers.rs`)
```rust
use crate::math::NaturalNumbers::NaturalNumber;
use crate::math::Sign;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IntegerNumber<const BASE: u128 = 256> {
    pub magnitude: NaturalNumber<BASE>,
    pub sign: Sign,
}

impl<const BASE: u128> fmt::Display for IntegerNumber<BASE> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.sign == Sign::Negative {
            write!(f, "-{}", self.magnitude)
        } else {
            write!(f, "{}", self.magnitude)
        }
    }
}

// Common base type aliases
pub type IntegerNumber10 = IntegerNumber<10>;
pub type IntegerNumber16 = IntegerNumber<16>;
```

#### `PositiveNaturalNumber<BASE>` (in `src/math/PositiveNaturalNumbers/positive_natural_numbers.rs`)
```rust
use crate::math::NaturalNumbers::NaturalNumber;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PositiveNaturalNumber<const BASE: u128 = 256> {
    pub value_minus_one: NaturalNumber<BASE>,
}

impl<const BASE: u128> fmt::Display for PositiveNaturalNumber<BASE> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_value())
    }
}

// Common base type aliases
pub type PositiveNaturalNumber10 = PositiveNaturalNumber<10>;
pub type PositiveNaturalNumber16 = PositiveNaturalNumber<16>;
```

#### `RationalNumber<BASE>` (in `src/math/RationalNumbers/rational_numbers.rs`)
```rust
use crate::math::NaturalNumbers::NaturalNumber;
use crate::math::PositiveNaturalNumbers::PositiveNaturalNumber;
use crate::math::Sign;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RationalNumber<const BASE: u128 = 256> {
    pub numerator: NaturalNumber<BASE>,
    pub denominator: PositiveNaturalNumber<BASE>,
    pub sign: Sign,
}

impl<const BASE: u128> fmt::Display for RationalNumber<BASE> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sign_prefix = if self.sign == Sign::Negative { "-" } else { "" };
        write!(f, "{}{} / {}", sign_prefix, self.numerator, self.denominator)
    }
}

// Common base type aliases
pub type RationalNumber10 = RationalNumber<10>;
pub type RationalNumber16 = RationalNumber<16>;
```

### 3. Base Conversion Method

```rust
impl<const OLD_BASE: u128> NaturalNumber<OLD_BASE> {
    pub fn convert_base<const NEW_BASE: u128>(&self) -> NaturalNumber<NEW_BASE> {
        let mut result = NaturalNumber::<NEW_BASE>::from(0u128);
        let mut power = NaturalNumber::<NEW_BASE>::from(1u128);
        
        let old_base_in_new = NaturalNumber::<NEW_BASE>::from(OLD_BASE);

        for digit in &self.digits {
            let bytes = digit.value();
            let mut val = 0u128;
            for (i, &byte) in bytes.iter().enumerate() {
                val |= (byte as u128) << (i * 8);
            }
            let digit_in_new = NaturalNumber::<NEW_BASE>::from(val);
            
            let term = crate::math::operations::Mul::mul(digit_in_new, power.clone());
            result = crate::math::operations::Add::add(result, term);

            power = crate::math::operations::Mul::mul(power, old_base_in_new.clone());
        }

        result
    }
}
```

## Integration Details
1. **File Registration**: Expose `pub mod base_digit;` inside `src/math/NaturalNumbers/mod.rs` and re-export `BaseDigit` and `bytes_needed_for_base`.
2. **Operation traits**: Update arithmetic traits implementations in `src/math/NaturalNumbers/arthimitic/*.rs` and `src/math/IntegerNunbers/arthimatic/*.rs` to accept const parameter `<const BASE: u128>`.

## Verification Plan
1. Ensure the project builds successfully on the stable toolchain.
2. Implement unit tests verifying:
   - Digits mapping calculation (`bytes_needed_for_base`).
   - Digit instantiation with the `DigitStorage` stack/heap variants.
   - Correct base-to-base conversion using `.convert_base()`.
   - String formatting output for both small base (<= 36) and large base (> 36) cases.
   - Working operations with `NaturalNumber10` and `NaturalNumber16`.
