use ndarray::{Array1, Array2};
use crate::error::SciError;

pub fn matrix_multiply(a: &Array2<f64>, b: &Array2<f64>) -> Result<Array2<f64>, SciError> {
    if a.ncols() != b.nrows() {
        return Err(SciError::InvalidInput("Incompatible matrix dimensions".to_string()));
    }
    Ok(a.dot(b))
}

pub fn vector_norm(v: &Array1<f64>) -> f64 {
    v.dot(v).sqrt()
}