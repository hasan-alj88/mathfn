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
}


#[derive(Debug, Error)]
pub enum ComputingError {
    #[error("Unable to cast {source_type:?} to {target_type:?}.")]
    CastingError {
        source_type: String,
        target_type: String,
    },

}