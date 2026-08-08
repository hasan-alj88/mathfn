pub trait Log<Base = Self> {
    type Output;
    fn log(self, base: Base) -> Self::Output;
}

pub trait Ln {
    type Output;
    fn ln(self) -> Self::Output;
}

pub trait TryLog<Base = Self> {
    type Output;
    type Error;
    fn try_log(self, base: Base) -> Result<Self::Output, Self::Error>;
}

pub trait TryLn {
    type Output;
    type Error;
    fn try_ln(self) -> Result<Self::Output, Self::Error>;
}
