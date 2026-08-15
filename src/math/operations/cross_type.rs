//! # Mathematical Theory of Cross-Type Domain Operations
//!
//! Positional number domains form a nested hierarchy of algebraic subsets:
//! $$\mathbb{Z}^+ \subset \mathbb{N} \subset \mathbb{I} \subset \mathbb{Q}$$
//!
//! Represented by the following types in this codebase:
//! `PositiveNaturalNumber` ($\mathbb{Z}^+$) $\subset$ `NaturalNumber` ($\mathbb{N}$) $\subset$ `IntegerNumber` ($\mathbb{I}$) $\subset$ `RationalNumber` ($\mathbb{Q}$)
//!
//! When performing cross-type operations, subset closures determine the resulting domain:
//!
//! 1. **Addition of Natural ($\mathbb{N}$) and Positive Natural ($\mathbb{Z}^+$)**:
//!    Let $n \in \mathbb{N}$ ($n \ge 0$) and $p \in \mathbb{Z}^+$ ($p \ge 1$).
//!    The sum is $n + p \ge 1$.
//!    Since the sum is strictly positive, the result is closed in $\mathbb{Z}^+$:
//!    $$\mathbb{N} + \mathbb{Z}^+ \to \mathbb{Z}^+$$
//!
//! 2. **Multiplication of Natural ($\mathbb{N}$) and Positive Natural ($\mathbb{Z}^+$)**:
//!    If $n = 0$, then $n \cdot p = 0 \notin \mathbb{Z}^+$.
//!    Since the product can be zero, the result is not closed in $\mathbb{Z}^+$, and must be represented in the wider domain $\mathbb{N}$:
//!    $$\mathbb{N} \times \mathbb{Z}^+ \to \mathbb{N}$$
//!
//! 3. **Operations with Integers ($\mathbb{I}$)**:
//!    Since $\mathbb{Z}^+ \subset \mathbb{I}$ and $\mathbb{N} \subset \mathbb{I}$, operations between `IntegerNumber` and any natural type promote directly to `IntegerNumber`.
//!
//! 4. **Operations with Rationals ($\mathbb{Q}$)**:
//!    Since $\mathbb{I} \subset \mathbb{Q}$, operations between `RationalNumber` and any integer/natural type promote directly to `RationalNumber`.
//!
//! 5. **Operations with Finite Continued Fractions (FCF)**:
//!    FCFs are isomorphic to $\mathbb{Q}$ but use a continued fraction representation. Arithmetic between FCF and other types is computed by converting both to `RationalNumber` and formatting the result back as an FCF.

use crate::math::natural_number::NaturalNumber;
use crate::math::positive_natural::PositiveNaturalNumber;
use crate::math::integer_number::IntegerNumber;
use crate::math::rational_number::RationalNumber;
use crate::math::operations::number_type::FiniteContinuedFractionNumber;
use crate::math::math_error::MathError;
use crate::math::sign::Sign;

// =========================================================================
// 1. NaturalNumber (N) and PositiveNaturalNumber (P)
// =========================================================================

impl<const BASE: u128> std::ops::Add<PositiveNaturalNumber<BASE>> for NaturalNumber<BASE> {
    type Output = Result<PositiveNaturalNumber<BASE>, MathError>;

    fn add(self, rhs: PositiveNaturalNumber<BASE>) -> Self::Output {
        let sum_offset = crate::math::natural_number::addition::nat_add_schoolbook(&self, rhs.offset_val())?;
        Ok(PositiveNaturalNumber::new_raw(sum_offset))
    }
}

impl<const BASE: u128> std::ops::Add<NaturalNumber<BASE>> for PositiveNaturalNumber<BASE> {
    type Output = Result<PositiveNaturalNumber<BASE>, MathError>;

    fn add(self, rhs: NaturalNumber<BASE>) -> Self::Output {
        let sum_offset = crate::math::natural_number::addition::nat_add_schoolbook(self.offset_val(), &rhs)?;
        Ok(PositiveNaturalNumber::new_raw(sum_offset))
    }
}

impl<const BASE: u128> std::ops::Mul<PositiveNaturalNumber<BASE>> for NaturalNumber<BASE> {
    type Output = Result<NaturalNumber<BASE>, MathError>;

    fn mul(self, rhs: PositiveNaturalNumber<BASE>) -> Self::Output {
        let rhs_nat = NaturalNumber::from(rhs);
        self * rhs_nat
    }
}

impl<const BASE: u128> std::ops::Mul<NaturalNumber<BASE>> for PositiveNaturalNumber<BASE> {
    type Output = Result<NaturalNumber<BASE>, MathError>;

    fn mul(self, rhs: NaturalNumber<BASE>) -> Self::Output {
        let self_nat = NaturalNumber::from(self);
        self_nat * rhs
    }
}

// Macro helper for standard promotions to a wider target type
macro_rules! impl_cross_arithmetic {
    ($Lhs:ty, $Rhs:ty, $Output:ty, $conv_l:expr, $conv_r:expr, $conv_res:expr) => {
        impl<const BASE: u128> std::ops::Add<$Rhs> for $Lhs {
            type Output = Result<$Output, crate::math::math_error::MathError>;

            fn add(self, rhs: $Rhs) -> Self::Output {
                let l = ($conv_l)(self)?;
                let r = ($conv_r)(rhs)?;
                let res = (l + r)?;
                ($conv_res)(res)
            }
        }

        impl<const BASE: u128> std::ops::Add<$Lhs> for $Rhs {
            type Output = Result<$Output, crate::math::math_error::MathError>;

            fn add(self, rhs: $Lhs) -> Self::Output {
                let l = ($conv_l)(rhs)?;
                let r = ($conv_r)(self)?;
                let res = (l + r)?;
                ($conv_res)(res)
            }
        }

        impl<const BASE: u128> std::ops::Mul<$Rhs> for $Lhs {
            type Output = Result<$Output, crate::math::math_error::MathError>;

            fn mul(self, rhs: $Rhs) -> Self::Output {
                let l = ($conv_l)(self)?;
                let r = ($conv_r)(rhs)?;
                let res = (l * r)?;
                ($conv_res)(res)
            }
        }

        impl<const BASE: u128> std::ops::Mul<$Lhs> for $Rhs {
            type Output = Result<$Output, crate::math::math_error::MathError>;

            fn mul(self, rhs: $Lhs) -> Self::Output {
                let l = ($conv_l)(rhs)?;
                let r = ($conv_r)(self)?;
                let res = (l * r)?;
                ($conv_res)(res)
            }
        }
    };
}

// =========================================================================
// 2. IntegerNumber (I) and NaturalNumber (N)
// =========================================================================
impl_cross_arithmetic!(
    IntegerNumber<BASE>,
    NaturalNumber<BASE>,
    IntegerNumber<BASE>,
    |x| Ok(x),
    |x| Ok(IntegerNumber::from(x)),
    |x| Ok(x)
);

// =========================================================================
// 3. IntegerNumber (I) and PositiveNaturalNumber (P)
// =========================================================================
impl_cross_arithmetic!(
    IntegerNumber<BASE>,
    PositiveNaturalNumber<BASE>,
    IntegerNumber<BASE>,
    |x| Ok(x),
    |x| Ok(IntegerNumber::from(NaturalNumber::from(x))),
    |x| Ok(x)
);

// =========================================================================
// 4. RationalNumber (Q) and IntegerNumber (I)
// =========================================================================
impl_cross_arithmetic!(
    RationalNumber<BASE>,
    IntegerNumber<BASE>,
    RationalNumber<BASE>,
    |x| Ok(x),
    |x| Ok(RationalNumber::from(x)),
    |x| Ok(x)
);

// =========================================================================
// 5. RationalNumber (Q) and NaturalNumber (N)
// =========================================================================
impl_cross_arithmetic!(
    RationalNumber<BASE>,
    NaturalNumber<BASE>,
    RationalNumber<BASE>,
    |x| Ok(x),
    |x| Ok(RationalNumber::from(x)),
    |x| Ok(x)
);

// =========================================================================
// 6. RationalNumber (Q) and PositiveNaturalNumber (P)
// =========================================================================
impl_cross_arithmetic!(
    RationalNumber<BASE>,
    PositiveNaturalNumber<BASE>,
    RationalNumber<BASE>,
    |x| Ok(x),
    |x| Ok(RationalNumber::from(x)),
    |x| Ok(x)
);

// =========================================================================
// 7. FiniteContinuedFractionNumber (FCF) and RationalNumber (Q)
// =========================================================================
impl_cross_arithmetic!(
    FiniteContinuedFractionNumber<BASE>,
    RationalNumber<BASE>,
    FiniteContinuedFractionNumber<BASE>,
    |x: FiniteContinuedFractionNumber<BASE>| x.to_rational(),
    |x| Ok(x),
    |x: RationalNumber<BASE>| x.to_continued_fraction()
);

// =========================================================================
// 8. FiniteContinuedFractionNumber (FCF) and IntegerNumber (I)
// =========================================================================
impl_cross_arithmetic!(
    FiniteContinuedFractionNumber<BASE>,
    IntegerNumber<BASE>,
    FiniteContinuedFractionNumber<BASE>,
    |x: FiniteContinuedFractionNumber<BASE>| x.to_rational(),
    |x| Ok(RationalNumber::from(x)),
    |x: RationalNumber<BASE>| x.to_continued_fraction()
);

// =========================================================================
// 9. FiniteContinuedFractionNumber (FCF) and NaturalNumber (N)
// =========================================================================
impl_cross_arithmetic!(
    FiniteContinuedFractionNumber<BASE>,
    NaturalNumber<BASE>,
    FiniteContinuedFractionNumber<BASE>,
    |x: FiniteContinuedFractionNumber<BASE>| x.to_rational(),
    |x| Ok(RationalNumber::from(x)),
    |x: RationalNumber<BASE>| x.to_continued_fraction()
);

// =========================================================================
// 10. FiniteContinuedFractionNumber (FCF) and PositiveNaturalNumber (P)
// =========================================================================
impl_cross_arithmetic!(
    FiniteContinuedFractionNumber<BASE>,
    PositiveNaturalNumber<BASE>,
    FiniteContinuedFractionNumber<BASE>,
    |x: FiniteContinuedFractionNumber<BASE>| x.to_rational(),
    |x| Ok(RationalNumber::from(x)),
    |x: RationalNumber<BASE>| x.to_continued_fraction()
);
