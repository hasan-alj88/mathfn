use super::number::{Compare, RoundingKind, Sign};
pub trait Convertible<N>{
    fn from_number(number: N) -> Self;
    fn to_number(&self) -> N;
}
pub trait  Infinity{
    type ResultType;
    fn is_infinite(&self) -> bool;
    fn is_finite(&self) -> bool;
    fn infinity() -> Self::ResultType;
}

pub trait Domain{
    type ResultType;
    fn is_defined(&self) -> bool;
    fn isin_domain(&self) -> bool;
}

pub trait OneInDomain{
    type ResultType;
    fn one() -> Self::ResultType;
    fn is_one(&self) -> bool;
}

pub trait ZeroInDomain{
    type ResultType;
    fn zero() -> Self::ResultType;
    fn is_zero(&self) -> bool;
}

pub trait PiInDomain{
    type ResultType;
    fn pi() -> Self::ResultType;
    fn tau() -> Self::ResultType;
}

pub trait EInDomain{
    type ResultType;
    fn e() -> Self::ResultType;
}
pub trait Angular {
    type ResultType: PiInDomain;
    fn to_degrees(&self) -> Self::ResultType;
    fn to_radians(&self) -> Self::ResultType;
}

pub trait Signed {
    fn abs(&self) -> Self;
    fn sign(&self) -> Sign;
}

pub trait Ordered{
    fn compare(&self, other: &Self) -> Compare;
    fn less_than(&self, other: &Self) -> bool;
    fn less_than_or_equal(&self, other: &Self) -> bool;
    fn greater_than(&self, other: &Self) -> bool;
    fn greater_than_or_equal(&self, other: &Self) -> bool;
    fn equal(&self, other: &Self) -> bool;
    fn not_equal(&self, other: &Self) -> bool;
    fn min(&self, other: &Self) -> Self;
    fn max(&self, other: &Self) -> Self;
    fn clamp(&self, min: &Self, max: &Self) -> Self;

}

pub trait Modular {
    type ResultType;
    fn mod_pow(&self, exp: Self, modulus: Self) -> Self::ResultType;
    fn mod_inv(&self, modulus: Self) -> Option<Self::ResultType>;
}

pub trait Additive{
    type ResultType;
    type OperandType;
    fn add(&self, other: Self::OperandType) -> Self::ResultType;
    fn sub(&self, other: Self::OperandType) -> Self::ResultType;
}

pub trait Multiplicative{
    type ResultType;
    type OperandType;
    fn mul(&self, other: Self::OperandType) -> Self::ResultType;
}

pub trait Divisible {
    type ResultType;
    type OperandType;
    fn div(&self, other: Self::OperandType) -> Self::ResultType;
    fn rem(&self, other: Self::OperandType) -> Self::ResultType;
    fn div_mod(&self, other: Self::OperandType) -> (Self::ResultType, Self::ResultType);

    fn divisors(&self) -> Vec<Self::ResultType>;
    fn divisor_count(&self) -> usize;
    fn divisor_sum(&self) -> Self::ResultType;

    fn gcd(&self, other: &Self) -> Self::ResultType;
    fn lcm(&self, other: &Self) -> Self::ResultType;
}


pub trait Trigonometric {
    type ResultType;
    fn sin(&self) -> Self::ResultType;
    fn cos(&self) -> Self::ResultType;
    fn tan(&self) -> Self::ResultType;
    fn cot(&self) -> Self::ResultType;
    fn sec(&self) -> Self::ResultType;
    fn csc(&self) -> Self::ResultType;
    fn arc_sin(&self) -> Self::ResultType;
    fn arc_cos(&self) -> Self::ResultType;
    fn arc_tan(&self) -> Self::ResultType;
    fn arc_cot(&self) -> Self::ResultType;
    fn arc_sec(&self) -> Self::ResultType;
    fn arc_csc(&self) -> Self::ResultType;
}

pub trait Hyperbolic {
    type ResultType;
    fn sinh(&self) -> Self::ResultType;
    fn cosh(&self) -> Self::ResultType;
    fn tanh(&self) -> Self::ResultType;
    fn coth(&self) -> Self::ResultType;
    fn sech(&self) -> Self::ResultType;
    fn csch(&self) -> Self::ResultType;
    fn arc_sinh(&self) -> Self::ResultType;
    fn arc_cosh(&self) -> Self::ResultType;
    fn arc_tanh(&self) -> Self::ResultType;
    fn arc_coth(&self) -> Self::ResultType;
    fn arc_sech(&self) -> Self::ResultType;
    fn arc_csch(&self) -> Self::ResultType;
}

pub trait Bounded{
    fn max_bound() -> Self;
    fn min_bound() -> Self;
    fn isin_bounds(&self) -> bool;
}

pub trait Exponential {
    type ResultType;
    type BaseType;
    type ExponentType;

    fn exp(&self) -> Self::ResultType;
    fn ln(&self) -> Self::ResultType;
    fn log(&self, base: Self::BaseType) -> Self::ResultType;
    fn log2(&self) -> Self::ResultType;
    fn log10(&self) -> Self::ResultType;
    fn sqrt(&self) -> Self::ResultType;
    fn cbrt(&self) -> Self::ResultType;
    fn pow(&self, exponent: Self::ExponentType) -> Self::ResultType; // Fix this
    fn nth_root(&self, n: u32) -> Self::ResultType;
}

pub trait Bitwise {
    type ResultType;

    // Bit manipulation
    fn bit_count(&self) -> u32; // population count
    fn leading_zeros(&self) -> u32;
    fn trailing_zeros(&self) -> u32;
    fn bit_reverse(&self) -> Self::ResultType;
    fn is_power_of_two(&self) -> bool;
    fn next_power_of_two(&self) -> Self::ResultType;

    // Bit operations
    fn set_bit(&self, pos: u32) -> Self::ResultType;
    fn clear_bit(&self, pos: u32) -> Self::ResultType;
    fn toggle_bit(&self, pos: u32) -> Self::ResultType;
    fn test_bit(&self, pos: u32) -> bool;

    // Rotation
    fn rotate_left(&self, n: u32) -> Self::ResultType;
    fn rotate_right(&self, n: u32) -> Self::ResultType;
}

pub trait Statistical {
    type ResultType;
    type CollectionType;

    // Basic statistics
    fn mean(data: &Self::CollectionType) -> Self::ResultType;
    fn median(data: &mut Self::CollectionType) -> Self::ResultType;
    fn mode(data: &Self::CollectionType) -> Vec<Self::ResultType>;
    fn variance(data: &Self::CollectionType) -> Self::ResultType;
    fn std_dev(data: &Self::CollectionType) -> Self::ResultType;
    fn skewness(data: &Self::CollectionType) -> Self::ResultType;
    fn kurtosis(data: &Self::CollectionType) -> Self::ResultType;

    // Quantiles
    fn percentile(data: &Self::CollectionType, p: Self) -> Self::ResultType;
    fn quartiles(data: &Self::CollectionType) -> (Self::ResultType, Self::ResultType, Self::ResultType);
    fn iqr(data: &Self::CollectionType) -> Self::ResultType; // Interquartile range

    // Correlation
    fn covariance(x: &Self::CollectionType, y: &Self::CollectionType) -> Self::ResultType;
    fn correlation(x: &Self::CollectionType, y: &Self::CollectionType) -> Self::ResultType;
    fn rank_correlation(x: &Self::CollectionType, y: &Self::CollectionType) -> Self::ResultType; // Spearman

    // Probability distributions
    fn normal_pdf(&self, mean: Self, std_dev: Self) -> Self::ResultType;
    fn normal_cdf(&self, mean: Self, std_dev: Self) -> Self::ResultType;
    fn normal_quantile(&self, mean: Self, std_dev: Self) -> Self::ResultType;

    fn poisson_pmf(&self, lambda: Self) -> Self::ResultType;
    fn binomial_pmf(&self, n: u32, p: Self) -> Self::ResultType;
    fn chi_squared_pdf(&self, df: Self) -> Self::ResultType;
    fn student_t_pdf(&self, df: Self) -> Self::ResultType;
}

pub trait RoundAble {
    type ResultType;
    fn round(&self, rounding_kind: RoundingKind) -> Self::ResultType;
}

pub trait Prime{
    type ResultType;
    fn is_prime(&self) -> bool;
    fn next_prime(&self) -> Self::ResultType;
    fn prev_prime(&self) -> Self::ResultType;
    fn prime_factors(&self) -> Vec<Self::ResultType>;
    fn is_prime_factor(&self, other: &Self) -> bool;

}

pub trait Combinatorial{
    type ResultType;
    fn factorial(&self) -> Self::ResultType; // n!
    fn double_factorial(&self) -> Self::ResultType; // n!!
    fn n_binomial_k(&self, k: Self) -> Self::ResultType;// C(n,k)
    fn n_permutations_k(&self, k: Self) -> Self::ResultType;// nPk
}
/*
todo!(
trait SpecialFunctions {
    type ResultType;

    // Gamma and related
    fn gamma(&self) -> Self::ResultType;
    fn log_gamma(&self) -> Self::ResultType;
    fn digamma(&self) -> Self::ResultType; // psi function
    fn beta(&self, other: Self) -> Self::ResultType;
    fn incomplete_gamma(&self, a: Self) -> Self::ResultType;

    // Error functions
    fn erf(&self) -> Self::ResultType;
    fn erfc(&self) -> Self::ResultType;
    fn erf_inv(&self) -> Self::ResultType;

    // Bessel functions
    fn bessel_j(&self, n: i32) -> Self::ResultType;
    fn bessel_y(&self, n: i32) -> Self::ResultType;
    fn bessel_i(&self, n: i32) -> Self::ResultType;
    fn bessel_k(&self, n: i32) -> Self::ResultType;

    // Elliptic integrals
    fn elliptic_k(&self) -> Self::ResultType;
    fn elliptic_e(&self) -> Self::ResultType;

    // Zeta and related
    fn riemann_zeta(&self) -> Self::ResultType;
    fn hurwitz_zeta(&self, a: Self) -> Self::ResultType;

    // Polylogarithm
    fn polylog(&self, s: Self) -> Self::ResultType;

    // Lambert W function
    fn lambert_w(&self) -> Self::ResultType;
    fn lambert_w_branch(&self, branch: i32) -> Self::ResultType;
}
    );

 */