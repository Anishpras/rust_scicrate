use thiserror::Error;

#[derive(Error, Debug)]
pub enum SciError {
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("Computation error: {0}")]
    ComputationError(String),
}