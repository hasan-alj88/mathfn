pub trait Pow<Rhs = Self> {
    type Output;
    fn pow(self, rhs: Rhs) -> Self::Output;
}

pub trait PowAssign<Rhs = Self> {
    fn pow_assign(&mut self, rhs: Rhs);
}

pub trait Square {
    type Output;
    fn square(self) -> Self::Output;
}

pub trait SquareAssign {
    fn square_assign(&mut self);
}

pub trait Sqrt {
    type Output;
    fn sqrt(self) -> Self::Output;
}

pub trait TryPow<Rhs = Self> {
    type Output;
    type Error;
    fn try_pow(self, rhs: Rhs) -> Result<Self::Output, Self::Error>;
}

pub trait TryPowAssign<Rhs = Self> {
    type Error;
    fn try_pow_assign(&mut self, rhs: Rhs) -> Result<(), Self::Error>;
}

pub trait TrySquare {
    type Output;
    type Error;
    fn try_square(self) -> Result<Self::Output, Self::Error>;
}

pub trait TrySquareAssign {
    type Error;
    fn try_square_assign(&mut self) -> Result<(), Self::Error>;
}

pub trait TrySqrt {
    type Output;
    type Error;
    fn try_sqrt(self) -> Result<Self::Output, Self::Error>;
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
