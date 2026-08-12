// Re-export standard library operations traits
#[allow(unused_imports)]
pub use std::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Deref,
    DerefMut, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Not, Rem, RemAssign, Shl,
    ShlAssign, Shr, ShrAssign, Sub, SubAssign,
};

pub mod basic;
pub mod logarithm;
pub mod power;
pub mod arithmetic;
pub mod placeholders;

pub use basic::{Double, DoubleAssign, Abs, TryDouble, TryDoubleAssign, TryAbs};
pub use logarithm::{Log, Ln, TryLog, TryLn};
pub use power::{
    Pow, PowAssign, Square, SquareAssign, Sqrt, TryPow, TryPowAssign, TrySquare, TrySquareAssign,
    TrySqrt,
};
pub use arithmetic::{
    TryAdd, TryAddAssign, TrySub, TrySubAssign, TryMul, TryMulAssign, TryDiv, TryDivAssign,
    TryRem, TryRemAssign,
};
pub use placeholders::RealNumber;
