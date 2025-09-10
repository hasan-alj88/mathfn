pub enum Sign {
    Positive,
    Negative,
}

pub enum OrderDirection {
    Ascending,
    Descending,
}

pub enum RoundingKind{
    HalfToEven,
    HalfToOdd,
    HalfAwayFromZero,
    HalfToZero,
    HalfDown,
    HalfUp,
    Ceiling,
    Floor,
}



pub enum Compare{
    LessThan,
    Equal,
    GreaterThan,
}

pub enum Number<N> {
    Finite(N),
    Infinity(Sign),
    Undefined,
    NotInDomain,
}

pub enum FiniteNumber<N> {
    Finite(N),
    Undefined,
    NotInDomain,
}

pub enum PositiveNumber<N> {
    Finite(N),
    Undefined,
    NotInDomain,
    Infinity(Sign),
}
