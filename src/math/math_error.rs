use thiserror::Error;

#[derive(Debug, Error)]
pub enum MathError {
    #[error("Result is not in {this_domain:?} domain. It is in {result_domain:?} domain.")]
    ResultNotInDomain {
        this_domain: String,
        result_domain: String,
    },
    #[error("Math object {math_object}.")]
    MathObjectIsUndefined{math_object: String},
    #[error("Division by zero.")]
    DivisionByZero,
    #[error("Quotient overflow.")]
    QuotientOverflow,
    #[error("Base mismatch.")]
    BaseMismatch,
    #[error("Unknown digit at position {position}.")]
    UnknownDigit { position: i64 },
}



#[derive(Debug, Error)]
pub enum ComputingError {
    #[error("Unable to cast {source_type:?} to {target_type:?}.")]
    CastingError {
        source_type: String,
        target_type: String,
    },

}

pub trait IntoMathError {
    fn into_math_error(self) -> MathError;
}

impl IntoMathError for MathError {
    fn into_math_error(self) -> MathError {
        self
    }
}

impl IntoMathError for std::convert::Infallible {
    fn into_math_error(self) -> MathError {
        match self {}
    }
}