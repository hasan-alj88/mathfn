pub trait Double {
    type Output;
    fn double(self) -> Self::Output;
}

pub trait DoubleAssign {
    fn double_assign(&mut self);
}

pub trait Abs {
    type Output;
    fn abs(self) -> Self::Output;
}

pub trait TryDouble {
    type Output;
    type Error;
    fn try_double(self) -> Result<Self::Output, Self::Error>;
}

pub trait TryDoubleAssign {
    type Error;
    fn try_double_assign(&mut self) -> Result<(), Self::Error>;
}

pub trait TryAbs {
    type Output;
    type Error;
    fn try_abs(self) -> Result<Self::Output, Self::Error>;
}
