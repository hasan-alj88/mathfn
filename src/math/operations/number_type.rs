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

/// A finite positional expansion representing an exact value.
///
/// $$V = \text{integer\_part} + \text{fractional\_part} \cdot \text{BASE}^{-\text{len}}$$
/// Digits outside the stored ranges are exactly 0.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FinitePrecision<const BASE: u128 = 256> {
    pub integer_part: NaturalNumber<BASE>,
    pub fractional_part: NaturalNumber<BASE>, // MSD-first order
}

impl<const BASE: u128> NumberType<BASE> for FinitePrecision<BASE> {
    fn digit(&self, pos: i64) -> Result<Digit<BASE>, MathError> {
        let zero_digit = Digit::new(0).unwrap();
        match pos {
            p if p >= 0 => {
                Ok(self.integer_part.digits()
                    .get(p as usize)
                    .copied()
                    .unwrap_or(zero_digit))
            }
            p => {
                let idx = (-p - 1) as usize;
                Ok(self.fractional_part.digits()
                    .get(idx)
                    .copied()
                    .unwrap_or(zero_digit))
            }
        }
    }
}

/// A finite continued fraction representation.
/// Represents a rational number exactly.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FiniteContinuedFractionNumber<const BASE: u128 = 256> {
    pub integer_part: IntegerNumber<BASE>,                // a_0
    pub coefficients: Vec<PositiveNaturalNumber<BASE>>,    // a_1, a_2, ..., a_n
}

impl<const BASE: u128> FiniteContinuedFractionNumber<BASE> {
    pub fn to_rational(&self) -> Result<crate::math::rational_number::RationalNumber<BASE>, MathError> {
        let mut p_prev2 = IntegerNumber::try_from(1i128)?;
        let mut p_prev1 = self.integer_part.clone();
        let mut q_prev2 = IntegerNumber::Zero;
        let mut q_prev1 = IntegerNumber::try_from(1i128)?;

        let mut p = p_prev1.clone();
        let mut q = q_prev1.clone();

        for coeff in &self.coefficients {
            let a_k = IntegerNumber::Positive(coeff.clone());
            p = ((a_k.clone() * p_prev1.clone())? + p_prev2)?;
            q = ((a_k * q_prev1.clone())? + q_prev2)?;

            p_prev2 = p_prev1;
            p_prev1 = p.clone();
            q_prev2 = q_prev1;
            q_prev1 = q.clone();
        }

        let p_nat = match &p {
            IntegerNumber::Zero => NaturalNumber::new(Vec::new()),
            IntegerNumber::Positive(abs) | IntegerNumber::Negative(abs) => NaturalNumber::from(abs.clone()),
        };
        let q_nat = match q {
            IntegerNumber::Zero => return Err(MathError::DivisionByZero),
            IntegerNumber::Positive(abs) | IntegerNumber::Negative(abs) => PositiveNaturalNumber::try_from(NaturalNumber::from(abs))?,
        };

        crate::math::rational_number::RationalNumber::new(p.sign(), PositiveNaturalNumber::try_from(p_nat)?, q_nat)
    }
}


impl<const BASE: u128> NumberType<BASE> for FiniteContinuedFractionNumber<BASE> {
    fn digit(&self, pos: i64) -> Result<Digit<BASE>, MathError> {
        self.to_rational()?.digit(pos)
    }
}

/// A Real Number representation ($\mathbb{R}$) in base `BASE`.
///
/// Real numbers can be represented using various models depending on their precision
/// and periodic patterns.
pub enum RealNumber<const BASE: u128 = 256> {
    /// An exact finite positional expansion.
    ExactFinite(FinitePrecision<BASE>),
    /// An approximate finite positional expansion.
    /// Digits beyond the stored fractional part are unknown (fails with UnknownDigit).
    Approximate(FinitePrecision<BASE>),
    /// A finite continued fraction.
    FiniteContinuedFraction(FiniteContinuedFractionNumber<BASE>),
    /// An infinite continued fraction where coefficients eventually repeat periodicially (e.g. sqrt(2) = [1; 2, 2, 2...]).
    RepeatedContinuedFraction {
        integer_part: IntegerNumber<BASE>,                    // a_0
        non_repeating: Vec<PositiveNaturalNumber<BASE>>,       // non-repeating coefficients
        repeating: Vec<PositiveNaturalNumber<BASE>>,           // periodic repeating coefficients
    },
    /// A continued fraction acting as an approximation of a real number (fails beyond its known convergent accuracy).
    ApproximateContinuedFraction(FiniteContinuedFractionNumber<BASE>),
    /// A floating-point number.
    Float {
        mantissa: PositiveNaturalNumber<BASE>,
        power: IntegerNumber<BASE>,
        sign: Sign,
    },
    /// An arbitrary formula yielding the digit at any position.
    DigitalFormula(Arc<dyn Fn(i64) -> Result<Digit<BASE>, MathError> + Send + Sync>),
    /// A periodic / repeating positional expansion.
    Repeated {
        integer_part: NaturalNumber<BASE>,
        fractional_part: NaturalNumber<BASE>,
        repeated: NaturalNumber<BASE>,
    },
}

impl<const BASE: u128> NumberType<BASE> for RealNumber<BASE> {
    fn digit(&self, pos: i64) -> Result<Digit<BASE>, MathError> {
        let zero_digit = Digit::new(0).unwrap();

        match self {
            RealNumber::ExactFinite(fp) => fp.digit(pos),

            RealNumber::Approximate(fp) => {
                match pos {
                    p if p >= 0 => fp.digit(p),
                    p => {
                        let idx = (-p - 1) as usize;
                        match idx < fp.fractional_part.digits().len() {
                            true => fp.digit(p),
                            false => Err(MathError::UnknownDigit { position: p }),
                        }
                    }
                }
            }

            RealNumber::FiniteContinuedFraction(fcf) => fcf.digit(pos),

            RealNumber::ApproximateContinuedFraction(fcf) => {
                let rat = fcf.to_rational()?;
                let q_n = NaturalNumber::from(rat.denominator().clone());
                let q_len = q_n.digits().len() as i64;
                match pos < -q_len {
                    true => Err(MathError::UnknownDigit { position: pos }),
                    false => rat.digit(pos),
                }
            }

            RealNumber::RepeatedContinuedFraction { integer_part, non_repeating, repeating } => {
                let mut coeffs = non_repeating.clone();
                match repeating.is_empty() {
                    true => {}
                    false => {
                        for _ in 0..10 {
                            coeffs.extend(repeating.clone());
                        }
                    }
                }
                let fcf = FiniteContinuedFractionNumber {
                    integer_part: integer_part.clone(),
                    coefficients: coeffs,
                };
                fcf.digit(pos)
            }

            RealNumber::Float { mantissa, power, sign: _ } => {
                let p_val = match i64::try_from(power.clone()) {
                    Ok(val) => val,
                    Err(_) => return Ok(zero_digit),
                };
                let mantissa_pos = pos - p_val;
                match mantissa_pos >= 0 {
                    true => {
                        let nat_mantissa = NaturalNumber::from(mantissa.clone());
                        Ok(nat_mantissa.digits()
                            .get(mantissa_pos as usize)
                            .copied()
                            .unwrap_or(zero_digit))
                    }
                    false => Ok(zero_digit),
                }
            }

            RealNumber::DigitalFormula(formula_fn) => {
                (formula_fn)(pos)
            }

            RealNumber::Repeated { integer_part, fractional_part, repeated } => {
                match pos >= 0 {
                    true => {
                        Ok(integer_part.digits()
                            .get(pos as usize)
                            .copied()
                            .unwrap_or(zero_digit))
                    }
                    false => {
                        let k = -pos;
                        let f_len = fractional_part.digits().len() as i64;
                        match k <= f_len {
                            true => Ok(fractional_part.digits()[(k - 1) as usize]),
                            false => {
                                let r_len = repeated.digits().len() as i64;
                                match r_len == 0 {
                                    true => Ok(zero_digit),
                                    false => {
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
        }
    }
}

impl<const BASE: u128> From<FinitePrecision<BASE>> for RealNumber<BASE> {
    fn from(fp: FinitePrecision<BASE>) -> Self {
        Self::ExactFinite(fp)
    }
}

impl<const BASE: u128> From<FiniteContinuedFractionNumber<BASE>> for RealNumber<BASE> {
    fn from(fcf: FiniteContinuedFractionNumber<BASE>) -> Self {
        Self::FiniteContinuedFraction(fcf)
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

impl<const BASE: u128> std::ops::Add for FiniteContinuedFractionNumber<BASE> {
    type Output = Result<Self, MathError>;

    fn add(self, rhs: Self) -> Self::Output {
        let r1 = self.to_rational()?;
        let r2 = rhs.to_rational()?;
        let sum_rat = (r1 + r2)?;
        sum_rat.to_continued_fraction()
    }
}

impl<const BASE: u128> std::ops::Mul for FiniteContinuedFractionNumber<BASE> {
    type Output = Result<Self, MathError>;

    fn mul(self, rhs: Self) -> Self::Output {
        let r1 = self.to_rational()?;
        let r2 = rhs.to_rational()?;
        let prod_rat = (r1 * r2)?;
        prod_rat.to_continued_fraction()
    }
}

impl<const BASE: u128> TryFrom<FiniteContinuedFractionNumber<BASE>> for crate::math::rational_number::RationalNumber<BASE> {
    type Error = MathError;

    fn try_from(fcf: FiniteContinuedFractionNumber<BASE>) -> Result<Self, Self::Error> {
        fcf.to_rational()
    }
}

impl<const BASE: u128> TryFrom<FiniteContinuedFractionNumber<BASE>> for IntegerNumber<BASE> {
    type Error = MathError;

    fn try_from(fcf: FiniteContinuedFractionNumber<BASE>) -> Result<Self, Self::Error> {
        let rat = fcf.to_rational()?;
        IntegerNumber::try_from(rat)
    }
}

impl<const BASE: u128> TryFrom<FiniteContinuedFractionNumber<BASE>> for NaturalNumber<BASE> {
    type Error = MathError;

    fn try_from(fcf: FiniteContinuedFractionNumber<BASE>) -> Result<Self, Self::Error> {
        let rat = fcf.to_rational()?;
        NaturalNumber::try_from(rat)
    }
}

impl<const BASE: u128> TryFrom<FiniteContinuedFractionNumber<BASE>> for PositiveNaturalNumber<BASE> {
    type Error = MathError;

    fn try_from(fcf: FiniteContinuedFractionNumber<BASE>) -> Result<Self, Self::Error> {
        let rat = fcf.to_rational()?;
        PositiveNaturalNumber::try_from(rat)
    }
}

impl<const BASE: u128> TryFrom<IntegerNumber<BASE>> for FiniteContinuedFractionNumber<BASE> {
    type Error = MathError;

    fn try_from(num: IntegerNumber<BASE>) -> Result<Self, Self::Error> {
        crate::math::rational_number::RationalNumber::from(num).to_continued_fraction()
    }
}

impl<const BASE: u128> TryFrom<NaturalNumber<BASE>> for FiniteContinuedFractionNumber<BASE> {
    type Error = MathError;

    fn try_from(num: NaturalNumber<BASE>) -> Result<Self, Self::Error> {
        crate::math::rational_number::RationalNumber::from(num).to_continued_fraction()
    }
}

impl<const BASE: u128> TryFrom<PositiveNaturalNumber<BASE>> for FiniteContinuedFractionNumber<BASE> {
    type Error = MathError;

    fn try_from(num: PositiveNaturalNumber<BASE>) -> Result<Self, Self::Error> {
        crate::math::rational_number::RationalNumber::from(num).to_continued_fraction()
    }
}


