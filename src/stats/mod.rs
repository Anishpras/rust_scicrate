use ndarray::Array1;
use crate::error::SciError;

pub fn mean(data: &Array1<f64>) -> Result<f64, SciError> {
    if data.is_empty() {
        return Err(SciError::InvalidInput("Empty array".to_string()));
    }
    Ok(data.mean().unwrap())
}

pub fn standard_deviation(data: &Array1<f64>) -> Result<f64, SciError> {
    if data.len() < 2 {
        return Err(SciError::InvalidInput("Insufficient data for standard deviation".to_string()));
    }
    let n = data.len() as f64;
    let mean = mean(data)?;
    let variance = data.iter().map(|&x| {
        let diff = x - mean;
        diff * diff
    }).sum::<f64>() / (n - 1.0);
    Ok(variance.sqrt())
}