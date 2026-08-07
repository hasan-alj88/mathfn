use thiserror::Error;

#[derive(Debug, Error)]
pub enum MathError {
    #[error("Result is not in {this_domain:?} domain. It is in {result_domain:?} domain.")]
    ResultNotInDomain {
        this_domain: String,
        result_domain: String,
    },
}