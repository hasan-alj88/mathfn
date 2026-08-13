//! # Real and Complex Number Domains with Digit-wise Arithmetic
//!
//! This module defines the core trait `NumberType` and representations for
//! the Real and Complex number domains in a positional numeral system of base `BASE`.
//!
//! ## Mathematical Theory of Domain Operations
//!
//! Positional number domains are structured in a nested hierarchy of algebraic closures:
//! $$\mathbb{N} \subset \mathbb{I} \subset \mathbb{Q} \subset \mathbb{R} \subset \mathbb{C}$$
//!
//! Each step upward in this hierarchy is motivated by achieving algebraic closure under a key operation:
//!
//! 1. **Subtraction ($\mathbb{N} - \mathbb{N} = \mathbb{I}$)**:
//!    Natural numbers ($\mathbb{N} = \{0, 1, 2, \dots\}$) are not closed under subtraction because $a - b$
//!    where $a < b$ has no solution in $\mathbb{N}$. Introducing negative numbers extends the domain
//!    to the Integers ($\mathbb{I}$), which form an additive group where subtraction is always defined.
//!
//! 2. **Division ($\mathbb{I} / \mathbb{I} = \mathbb{Q}$)**:
//!    Integers ($\mathbb{I}$) are not closed under division because $a / b$ where $b \ne 0$ and $b \nmid a$
//!    has no solution in $\mathbb{I}$ (e.g. $1 / 2$). Extending the domain to quotients of integers
//!    yields the Rational numbers ($\mathbb{Q}$), which form a field.
//!
//! 3. **Roots and Powers ($\mathbb{Q} ^ {\mathbb{Q}} = \mathbb{R}$)**:
//!    Rational numbers ($\mathbb{Q}$) are not closed under exponentiation with rational powers
//!    (e.g., $2^{1/2} = \sqrt{2}$ has no rational solution). Extending the domain to the limits of
//!    all convergent rational sequences yields the Real numbers ($\mathbb{R}$).
//!
//! 4. **Negative Roots ($\mathbb{R} ^ {\mathbb{Q}} = \mathbb{C}$)**:
//!    Real numbers ($\mathbb{R}$) are not closed under powers of negative numbers (e.g., $(-1)^{1/2} = \sqrt{-1}$).
//!    Adding the imaginary unit $i = \sqrt{-1}$ extends reals to the Complex numbers ($\mathbb{C}$).

use crate::math::base_digit::Digit;
use crate::math::math_error::MathError;
use crate::math::natural_number::NaturalNumber;
use crate::math::positive_natural::PositiveNaturalNumber;
use crate::math::integer_number::IntegerNumber;
use crate::math::sign::Sign;
use std::sync::Arc;

/// A trait representing any positional number type generic over base `BASE`.
///
/// Implementors must support digit retrieval at any position `pos` (with `pos >= 0` indexing
/// the integer part, and `pos < 0` indexing the fractional part).
pub trait NumberType<const BASE: u128> {
    /// Returns the digit at position `pos`.
    ///
    /// # Guide & Indexing Rules
    /// - `pos = 0`: Units digit ($BASE^0$).
    /// - `pos > 0`: Integer digit at power $BASE^{pos}$.
    /// - `pos < 0`: Fractional digit at power $BASE^{pos}$.
    ///
    /// # Examples
    /// ```ignore
    /// # use mathfn::math::natural_number::NaturalNumber;
    /// # use mathfn::math::operations::NumberType;
    /// let nat = NaturalNumber::<256>::from_u128(123).unwrap();
    /// assert_eq!(nat.digit(0).unwrap().value(), 123);
    /// assert_eq!(nat.digit(1).unwrap().value(), 0);
    /// ```
    ///
    /// # Errors
    /// Returns `MathError::UnknownDigit` if the digit at the specified position is unknown
    /// (e.g., beyond the precision limit of a finite-precision real number).
    fn digit(&self, pos: i64) -> Result<Digit<BASE>, MathError>;
}

/// A Real Number representation ($\mathbb{R}$) in base `BASE`.
///
/// Real numbers can be represented using various models depending on their precision
/// and periodic patterns.
pub enum RealNumber<const BASE: u128 = 256> {
    /// A finite positional expansion.
    ///
    /// Represented exactly as:
    /// $$V = \text{integer\_part} + \text{fractional\_part} \cdot \text{BASE}^{-\text{len}}$$
    /// Digits beyond the fractional part are unknown, hence querying them fails.
    ///
    /// # Guide
    /// Stored fractional part digits are in Most Significant Digit first order.
    ///
    /// # Examples
    /// ```ignore
    /// # use mathfn::math::operations::number_type::RealNumber;
    /// # use mathfn::math::natural_number::NaturalNumber;
    /// let r = RealNumber::<256>::FinitePrecision {
    ///     integer_part: NaturalNumber::from_u128(12).unwrap(),
    ///     fractional_part: NaturalNumber::from_u128(34).unwrap(),
    /// };
    /// ```
    FinitePrecision {
        integer_part: NaturalNumber<BASE>,
        fractional_part: NaturalNumber<BASE>,
    },
    /// A floating-point number.
    ///
    /// Represented as:
    /// $$V = \text{sign} \cdot \text{mantissa} \cdot \text{BASE}^{\text{power}}$$
    /// This is an exact rational/real representation, so all digits below the mantissa's scale are exactly 0.
    ///
    /// # Examples
    /// ```ignore
    /// # use mathfn::math::operations::number_type::RealNumber;
    /// # use mathfn::math::positive_natural::PositiveNaturalNumber;
    /// # use mathfn::math::integer_number::IntegerNumber;
    /// # use mathfn::math::sign::Sign;
    /// let f = RealNumber::<256>::Float {
    ///     mantissa: PositiveNaturalNumber::try_from(123u128).unwrap(),
    ///     power: IntegerNumber::try_from(-2i32).unwrap(),
    ///     sign: Sign::Positive,
    /// };
    /// ```
    Float {
        mantissa: PositiveNaturalNumber<BASE>,
        power: IntegerNumber<BASE>,
        sign: Sign,
    },
    /// An arbitrary formula yielding the digit at any position.
    ///
    /// Useful for representing computable irrational numbers like $\pi$ or $e$.
    ///
    /// # Examples
    /// ```ignore
    /// # use std::sync::Arc;
    /// # use mathfn::math::operations::number_type::RealNumber;
    /// # use mathfn::math::base_digit::Digit;
    /// // Represents 0.3333... via a formula
    /// let formula = RealNumber::<256>::DigitalFormula(Arc::new(|pos| {
    ///     if pos < 0 {
    ///         Ok(Digit::new(3).unwrap())
    ///     } else {
    ///         Ok(Digit::new(0).unwrap())
    ///     }
    /// }));
    /// ```
    DigitalFormula(Arc<dyn Fn(i64) -> Result<Digit<BASE>, MathError> + Send + Sync>),
    /// A periodic / repeating positional expansion.
    ///
    /// Represented as:
    /// $$V = \text{integer\_part} + 0.\text{fractional\_part}(\text{repeated}\dots)$$
    /// An infinitely long expansion that is completely determined by its period.
    ///
    /// # Examples
    /// ```ignore
    /// # use mathfn::math::operations::number_type::RealNumber;
    /// # use mathfn::math::natural_number::NaturalNumber;
    /// // Represents 0.12(34)
    /// let r = RealNumber::<256>::Repeated {
    ///     integer_part: NaturalNumber::from_u128(0).unwrap(),
    ///     fractional_part: NaturalNumber::from_u128(12).unwrap(),
    ///     repeated: NaturalNumber::from_u128(34).unwrap(),
    /// };
    /// ```
    Repeated {
        integer_part: NaturalNumber<BASE>,
        fractional_part: NaturalNumber<BASE>,
        repeated: NaturalNumber<BASE>,
    },
}

impl<const BASE: u128> NumberType<BASE> for RealNumber<BASE> {
    /// Yields the digit of this real number at the given index.
    ///
    /// # Guide
    /// - For `FinitePrecision`: Returns `MathError::UnknownDigit` if index is out of bounds for the fractional part.
    /// - For `Float`: Digits below the mantissa's scale return `Ok(0)`.
    /// - For `Repeated`: The fractional digits cycle infinitely through the `repeated` digits.
    ///
    /// # Examples
    /// ```ignore
    /// # use mathfn::math::operations::number_type::RealNumber;
    /// # use mathfn::math::natural_number::NaturalNumber;
    /// # use mathfn::math::operations::NumberType;
    /// let r = RealNumber::<256>::FinitePrecision {
    ///     integer_part: NaturalNumber::from_u128(5).unwrap(),
    ///     fractional_part: NaturalNumber::from_u128(6).unwrap(),
    /// };
    /// assert_eq!(r.digit(0).unwrap().value(), 5);
    /// assert_eq!(r.digit(-1).unwrap().value(), 6);
    /// assert!(r.digit(-2).is_err()); // Unknown digit
    /// ```
    fn digit(&self, pos: i64) -> Result<Digit<BASE>, MathError> {
        let zero_digit = Digit::new(0).unwrap();

        match self {
            RealNumber::FinitePrecision { integer_part, fractional_part } => {
                if pos >= 0 {
                    Ok(integer_part.digits()
                        .get(pos as usize)
                        .copied()
                        .unwrap_or(zero_digit))
                } else {
                    let idx = (-pos - 1) as usize;
                    if idx < fractional_part.digits().len() {
                        Ok(fractional_part.digits()[idx])
                    } else {
                        Err(MathError::UnknownDigit { position: pos })
                    }
                }
            }

            RealNumber::Float { mantissa, power, sign: _ } => {
                let p_val = match i64::try_from(power.clone()) {
                    Ok(val) => val,
                    Err(_) => return Ok(zero_digit),
                };
                let mantissa_pos = pos - p_val;
                if mantissa_pos >= 0 {
                    let nat_mantissa = NaturalNumber::from(mantissa.clone());
                    Ok(nat_mantissa.digits()
                        .get(mantissa_pos as usize)
                        .copied()
                        .unwrap_or(zero_digit))
                } else {
                    Ok(zero_digit)
                }
            }

            RealNumber::DigitalFormula(formula_fn) => {
                (formula_fn)(pos)
            }

            RealNumber::Repeated { integer_part, fractional_part, repeated } => {
                if pos >= 0 {
                    Ok(integer_part.digits()
                        .get(pos as usize)
                        .copied()
                        .unwrap_or(zero_digit))
                } else {
                    let k = -pos;
                    let f_len = fractional_part.digits().len() as i64;
                    if k <= f_len {
                        Ok(fractional_part.digits()[(k - 1) as usize])
                    } else {
                        let r_len = repeated.digits().len() as i64;
                        if r_len == 0 {
                            Ok(zero_digit)
                        } else {
                            let offset = k - 1 - f_len;
                            let r_idx = (offset % r_len) as usize;
                            Ok(repeated.digits()[r_idx])
                        }
                    }
                }
            }
        }
    }
}

/// A Complex Number ($\mathbb{C}$) of the form $a + bi$.
///
/// Generic over the real part `R` and imaginary part `I`, defaulting to `RealNumber`.
pub struct ComplexNumber<
    const BASE: u128 = 256,
    R: NumberType<BASE> = RealNumber<BASE>,
    I: NumberType<BASE> = RealNumber<BASE>,
> {
    pub re: R,
    pub im: I,
}

impl<const BASE: u128, R, I> ComplexNumber<BASE, R, I>
where
    R: NumberType<BASE>,
    I: NumberType<BASE>,
{
    /// Creates a new Complex Number from real and imaginary parts.
    ///
    /// # Examples
    /// ```ignore
    /// # use mathfn::math::operations::number_type::{ComplexNumber, RealNumber};
    /// # use mathfn::math::natural_number::NaturalNumber;
    /// let real = RealNumber::FinitePrecision {
    ///     integer_part: NaturalNumber::from_u128(2).unwrap(),
    ///     fractional_part: NaturalNumber::from_u128(0).unwrap(),
    /// };
    /// let imag = RealNumber::FinitePrecision {
    ///     integer_part: NaturalNumber::from_u128(3).unwrap(),
    ///     fractional_part: NaturalNumber::from_u128(0).unwrap(),
    /// };
    /// let c = ComplexNumber::new(real, imag); // 2 + 3i
    /// ```
    pub fn new(re: R, im: I) -> Self {
        Self { re, im }
    }
}

use std::ops::{Add, Sub, Mul, Div, Rem};
use crate::math::operations::power::Pow;

/// Overload `+` operator for two Reals:
///
/// Adds two numbers digit-wise. Since it can be infinite, the resulting sum
/// is represented as a new `DigitalFormula` evaluating digits lazily.
///
/// # Examples
/// ```ignore
/// # use mathfn::math::operations::number_type::RealNumber;
/// # use mathfn::math::natural_number::NaturalNumber;
/// let a = RealNumber::FinitePrecision {
///     integer_part: NaturalNumber::from_u128(10).unwrap(),
///     fractional_part: NaturalNumber::from_u128(0).unwrap(),
/// };
/// let b = RealNumber::FinitePrecision {
///     integer_part: NaturalNumber::from_u128(5).unwrap(),
///     fractional_part: NaturalNumber::from_u128(0).unwrap(),
/// };
/// let sum = a + b;
/// assert_eq!(sum.digit(0).unwrap().value(), 15);
/// ```
impl<const BASE: u128, Rhs: NumberType<BASE> + Send + Sync + 'static> Add<Rhs> for RealNumber<BASE> {
    type Output = RealNumber<BASE>;
    fn add(self, rhs: Rhs) -> Self::Output {
        let left = Arc::new(self);
        let right = Arc::new(rhs);
        RealNumber::DigitalFormula(Arc::new(move |pos| {
            let d_l = left.digit(pos)?.value();
            let d_r = right.digit(pos)?.value();
            Digit::new((d_l + d_r) % BASE).map_err(|e| MathError::MathObjectIsUndefined { math_object: e.to_string() })
        }))
    }
}

/// Overload `-` operator for two Reals:
///
/// Performs digit-wise subtraction with base-borrow wrapping.
///
/// # Examples
/// ```ignore
/// # use mathfn::math::operations::number_type::RealNumber;
/// # use mathfn::math::natural_number::NaturalNumber;
/// let a = RealNumber::FinitePrecision {
///     integer_part: NaturalNumber::from_u128(10).unwrap(),
///     fractional_part: NaturalNumber::from_u128(0).unwrap(),
/// };
/// let b = RealNumber::FinitePrecision {
///     integer_part: NaturalNumber::from_u128(4).unwrap(),
///     fractional_part: NaturalNumber::from_u128(0).unwrap(),
/// };
/// let diff = a - b;
/// assert_eq!(diff.digit(0).unwrap().value(), 6);
/// ```
impl<const BASE: u128, Rhs: NumberType<BASE> + Send + Sync + 'static> Sub<Rhs> for RealNumber<BASE> {
    type Output = RealNumber<BASE>;
    fn sub(self, rhs: Rhs) -> Self::Output {
        let left = Arc::new(self);
        let right = Arc::new(rhs);
        RealNumber::DigitalFormula(Arc::new(move |pos| {
            let d_l = left.digit(pos)?.value();
            let d_r = right.digit(pos)?.value();
            Digit::new((d_l + BASE - d_r) % BASE).map_err(|e| MathError::MathObjectIsUndefined { math_object: e.to_string() })
        }))
    }
}

/// Overload `*` operator for two Reals:
///
/// Performs digit-wise multiplication modulo BASE.
///
/// # Examples
/// ```ignore
/// # use mathfn::math::operations::number_type::RealNumber;
/// # use mathfn::math::natural_number::NaturalNumber;
/// let a = RealNumber::FinitePrecision {
///     integer_part: NaturalNumber::from_u128(3).unwrap(),
///     fractional_part: NaturalNumber::from_u128(0).unwrap(),
/// };
/// let b = RealNumber::FinitePrecision {
///     integer_part: NaturalNumber::from_u128(5).unwrap(),
///     fractional_part: NaturalNumber::from_u128(0).unwrap(),
/// };
/// let prod = a * b;
/// assert_eq!(prod.digit(0).unwrap().value(), 15);
/// ```
impl<const BASE: u128, Rhs: NumberType<BASE> + Send + Sync + 'static> Mul<Rhs> for RealNumber<BASE> {
    type Output = RealNumber<BASE>;
    fn mul(self, rhs: Rhs) -> Self::Output {
        let left = Arc::new(self);
        let right = Arc::new(rhs);
        RealNumber::DigitalFormula(Arc::new(move |pos| {
            let d_l = left.digit(pos)?.value();
            let d_r = right.digit(pos)?.value();
            Digit::new((d_l * d_r) % BASE).map_err(|e| MathError::MathObjectIsUndefined { math_object: e.to_string() })
        }))
    }
}

/// Overload `/` operator for two Reals:
///
/// Performs digit-wise division. Fails if the divisor digit is 0.
///
/// # Examples
/// ```ignore
/// # use mathfn::math::operations::number_type::RealNumber;
/// # use mathfn::math::natural_number::NaturalNumber;
/// let a = RealNumber::FinitePrecision {
///     integer_part: NaturalNumber::from_u128(10).unwrap(),
///     fractional_part: NaturalNumber::from_u128(0).unwrap(),
/// };
/// let b = RealNumber::FinitePrecision {
///     integer_part: NaturalNumber::from_u128(2).unwrap(),
///     fractional_part: NaturalNumber::from_u128(0).unwrap(),
/// };
/// let div = a / b;
/// assert_eq!(div.digit(0).unwrap().value(), 5);
/// ```
impl<const BASE: u128, Rhs: NumberType<BASE> + Send + Sync + 'static> Div<Rhs> for RealNumber<BASE> {
    type Output = RealNumber<BASE>;
    fn div(self, rhs: Rhs) -> Self::Output {
        let left = Arc::new(self);
        let right = Arc::new(rhs);
        RealNumber::DigitalFormula(Arc::new(move |pos| {
            let d_l = left.digit(pos)?.value();
            let d_r = right.digit(pos)?.value();
            if d_r == 0 {
                return Err(MathError::DivisionByZero);
            }
            Digit::new(d_l / d_r).map_err(|e| MathError::MathObjectIsUndefined { math_object: e.to_string() })
        }))
    }
}

/// Overload `%` operator for two Reals:
///
/// Performs digit-wise remainder operation.
///
/// # Examples
/// ```ignore
/// # use mathfn::math::operations::number_type::RealNumber;
/// # use mathfn::math::natural_number::NaturalNumber;
/// let a = RealNumber::FinitePrecision {
///     integer_part: NaturalNumber::from_u128(10).unwrap(),
///     fractional_part: NaturalNumber::from_u128(0).unwrap(),
/// };
/// let b = RealNumber::FinitePrecision {
///     integer_part: NaturalNumber::from_u128(3).unwrap(),
///     fractional_part: NaturalNumber::from_u128(0).unwrap(),
/// };
/// let rem = a % b;
/// assert_eq!(rem.digit(0).unwrap().value(), 1);
/// ```
impl<const BASE: u128, Rhs: NumberType<BASE> + Send + Sync + 'static> Rem<Rhs> for RealNumber<BASE> {
    type Output = RealNumber<BASE>;
    fn rem(self, rhs: Rhs) -> Self::Output {
        let left = Arc::new(self);
        let right = Arc::new(rhs);
        RealNumber::DigitalFormula(Arc::new(move |pos| {
            let d_l = left.digit(pos)?.value();
            let d_r = right.digit(pos)?.value();
            if d_r == 0 {
                return Err(MathError::DivisionByZero);
            }
            Digit::new(d_l % d_r).map_err(|e| MathError::MathObjectIsUndefined { math_object: e.to_string() })
        }))
    }
}

/// Overload `^` operator for Real ^ Rhs:
///
/// Performs digit-wise exponentiation.
///
/// # Examples
/// ```ignore
/// # use mathfn::math::operations::number_type::RealNumber;
/// # use mathfn::math::natural_number::NaturalNumber;
/// # use mathfn::math::operations::power::Pow;
/// let a = RealNumber::FinitePrecision {
///     integer_part: NaturalNumber::from_u128(2).unwrap(),
///     fractional_part: NaturalNumber::from_u128(0).unwrap(),
/// };
/// let b = RealNumber::FinitePrecision {
///     integer_part: NaturalNumber::from_u128(3).unwrap(),
///     fractional_part: NaturalNumber::from_u128(0).unwrap(),
/// };
/// let power = a.pow(b);
/// assert_eq!(power.digit(0).unwrap().value(), 8);
/// ```
impl<const BASE: u128, Rhs: NumberType<BASE> + Send + Sync + 'static> Pow<Rhs> for RealNumber<BASE> {
    type Output = RealNumber<BASE>;
    fn pow(self, rhs: Rhs) -> Self::Output {
        let left = Arc::new(self);
        let right = Arc::new(rhs);
        RealNumber::DigitalFormula(Arc::new(move |pos| {
            let d_l = left.digit(pos)?.value();
            let d_r = right.digit(pos)?.value();
            Digit::new(d_l.pow(d_r as u32) % BASE).map_err(|e| MathError::MathObjectIsUndefined { math_object: e.to_string() })
        }))
    }
}

/// Complex Addition:
///
/// $$(a + bi) + (c + di) = (a + c) + (b + d)i$$
///
/// # Examples
/// ```ignore
/// # use mathfn::math::operations::number_type::{ComplexNumber, RealNumber};
/// # use mathfn::math::natural_number::NaturalNumber;
/// let x = ComplexNumber::new(
///     RealNumber::FinitePrecision { integer_part: NaturalNumber::from_u128(1).unwrap(), fractional_part: NaturalNumber::from_u128(0).unwrap() },
///     RealNumber::FinitePrecision { integer_part: NaturalNumber::from_u128(2).unwrap(), fractional_part: NaturalNumber::from_u128(0).unwrap() }
/// );
/// let y = ComplexNumber::new(
///     RealNumber::FinitePrecision { integer_part: NaturalNumber::from_u128(3).unwrap(), fractional_part: NaturalNumber::from_u128(0).unwrap() },
///     RealNumber::FinitePrecision { integer_part: NaturalNumber::from_u128(4).unwrap(), fractional_part: NaturalNumber::from_u128(0).unwrap() }
/// );
/// let sum = x + y;
/// assert_eq!(sum.re.digit(0).unwrap().value(), 4);
/// assert_eq!(sum.im.digit(0).unwrap().value(), 6);
/// ```
impl<const BASE: u128, Lre, Lim, Rre, Rim> Add<ComplexNumber<BASE, Rre, Rim>> for ComplexNumber<BASE, Lre, Lim>
where
    Lre: NumberType<BASE> + Add<Rre, Output = RealNumber<BASE>>,
    Lim: NumberType<BASE> + Add<Rim, Output = RealNumber<BASE>>,
    Rre: NumberType<BASE>,
    Rim: NumberType<BASE>,
{
    type Output = ComplexNumber<BASE, RealNumber<BASE>, RealNumber<BASE>>;
    fn add(self, rhs: ComplexNumber<BASE, Rre, Rim>) -> Self::Output {
        ComplexNumber::new(self.re + rhs.re, self.im + rhs.im)
    }
}

/// Complex Subtraction:
///
/// $$(a + bi) - (c + di) = (a - c) + (b - d)i$$
///
/// # Examples
/// ```ignore
/// # use mathfn::math::operations::number_type::{ComplexNumber, RealNumber};
/// # use mathfn::math::natural_number::NaturalNumber;
/// let x = ComplexNumber::new(
///     RealNumber::FinitePrecision { integer_part: NaturalNumber::from_u128(5).unwrap(), fractional_part: NaturalNumber::from_u128(0).unwrap() },
///     RealNumber::FinitePrecision { integer_part: NaturalNumber::from_u128(6).unwrap(), fractional_part: NaturalNumber::from_u128(0).unwrap() }
/// );
/// let y = ComplexNumber::new(
///     RealNumber::FinitePrecision { integer_part: NaturalNumber::from_u128(2).unwrap(), fractional_part: NaturalNumber::from_u128(0).unwrap() },
///     RealNumber::FinitePrecision { integer_part: NaturalNumber::from_u128(3).unwrap(), fractional_part: NaturalNumber::from_u128(0).unwrap() }
/// );
/// let diff = x - y;
/// assert_eq!(diff.re.digit(0).unwrap().value(), 3);
/// assert_eq!(diff.im.digit(0).unwrap().value(), 3);
/// ```
impl<const BASE: u128, Lre, Lim, Rre, Rim> Sub<ComplexNumber<BASE, Rre, Rim>> for ComplexNumber<BASE, Lre, Lim>
where
    Lre: NumberType<BASE> + Sub<Rre, Output = RealNumber<BASE>>,
    Lim: NumberType<BASE> + Sub<Rim, Output = RealNumber<BASE>>,
    Rre: NumberType<BASE>,
    Rim: NumberType<BASE>,
{
    type Output = ComplexNumber<BASE, RealNumber<BASE>, RealNumber<BASE>>;
    fn sub(self, rhs: ComplexNumber<BASE, Rre, Rim>) -> Self::Output {
        ComplexNumber::new(self.re - rhs.re, self.im - rhs.im)
    }
}

/// Complex Multiplication:
///
/// $$(a + bi) \cdot (c + di) = (ac - bd) + (ad + bc)i$$
///
/// # Examples
/// ```ignore
/// # use mathfn::math::operations::number_type::{ComplexNumber, RealNumber};
/// # use mathfn::math::natural_number::NaturalNumber;
/// let x = ComplexNumber::new(
///     RealNumber::FinitePrecision { integer_part: NaturalNumber::from_u128(2).unwrap(), fractional_part: NaturalNumber::from_u128(0).unwrap() },
///     RealNumber::FinitePrecision { integer_part: NaturalNumber::from_u128(3).unwrap(), fractional_part: NaturalNumber::from_u128(0).unwrap() }
/// );
/// let y = ComplexNumber::new(
///     RealNumber::FinitePrecision { integer_part: NaturalNumber::from_u128(4).unwrap(), fractional_part: NaturalNumber::from_u128(0).unwrap() },
///     RealNumber::FinitePrecision { integer_part: NaturalNumber::from_u128(5).unwrap(), fractional_part: NaturalNumber::from_u128(0).unwrap() }
/// );
/// let prod = x * y; // (2*4 - 3*5) + (2*5 + 3*4)i = -7 + 22i
/// ```
impl<const BASE: u128, Lre, Lim, Rre, Rim> Mul<ComplexNumber<BASE, Rre, Rim>> for ComplexNumber<BASE, Lre, Lim>
where
    Lre: NumberType<BASE> + Mul<Rre, Output = RealNumber<BASE>> + Mul<Rim, Output = RealNumber<BASE>> + Clone,
    Lim: NumberType<BASE> + Mul<Rim, Output = RealNumber<BASE>> + Mul<Rre, Output = RealNumber<BASE>> + Clone,
    Rre: NumberType<BASE> + Clone,
    Rim: NumberType<BASE> + Clone,
    RealNumber<BASE>: Add<RealNumber<BASE>, Output = RealNumber<BASE>> + Sub<RealNumber<BASE>, Output = RealNumber<BASE>>,
{
    type Output = ComplexNumber<BASE, RealNumber<BASE>, RealNumber<BASE>>;
    fn mul(self, rhs: ComplexNumber<BASE, Rre, Rim>) -> Self::Output {
        let ac = self.re.clone() * rhs.re.clone();
        let bd = self.im.clone() * rhs.im.clone();
        let ad = self.re * rhs.im;
        let bc = self.im * rhs.re;
        ComplexNumber::new(ac - bd, ad + bc)
    }
}
