// Re-export standard library operations traits
#[allow(unused_imports)]
pub use std::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Deref,
    DerefMut, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Not, Rem, RemAssign, Shl,
    ShlAssign, Shr, ShrAssign, Sub, SubAssign,
};


/// Generic trait for exponentiation operation (`pow`).
pub trait Pow<Rhs = Self> {
    type Output;
    fn pow(self, rhs: Rhs) -> Self::Output;
}

/// Generic trait for in-place exponentiation operation (`pow_assign`).
pub trait PowAssign<Rhs = Self> {
    fn pow_assign(&mut self, rhs: Rhs);
}

// Implement Pow and PowAssign for primitive integer types with u32 exponent
macro_rules! impl_pow_int {
    ($($t:ty),*) => {
        $(
            impl Pow<u32> for $t {
                type Output = $t;
                fn pow(self, rhs: u32) -> Self::Output {
                    <$t>::pow(self, rhs)
                }
            }

            impl PowAssign<u32> for $t {
                fn pow_assign(&mut self, rhs: u32) {
                    *self = <$t>::pow(*self, rhs);
                }
            }
        )*
    };
}

impl_pow_int!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

// Implement Pow and PowAssign for floating point types
impl Pow<f32> for f32 {
    type Output = f32;
    fn pow(self, rhs: f32) -> Self::Output {
        self.powf(rhs)
    }
}

impl PowAssign<f32> for f32 {
    fn pow_assign(&mut self, rhs: f32) {
        *self = self.powf(rhs);
    }
}

impl Pow<f64> for f64 {
    type Output = f64;
    fn pow(self, rhs: f64) -> Self::Output {
        self.powf(rhs)
    }
}

impl PowAssign<f64> for f64 {
    fn pow_assign(&mut self, rhs: f64) {
        *self = self.powf(rhs);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pow_integers() {
        assert_eq!(2u32.pow(3u32), 8u32);
        let mut x = 3u64;
        x.pow_assign(3u32);
        assert_eq!(x, 27u64);
    }

    #[test]
    fn test_pow_floats() {
        assert_eq!(2.0f64.pow(3.0f64), 8.0f64);
        let mut f = 4.0f32;
        f.pow_assign(0.5f32);
        assert_eq!(f, 2.0f32);
    }
}

