use crate::error::SciError;

pub fn trapezoidal<F: Fn(f64) -> f64>(f: F, a: f64, b: f64, n: usize) -> Result<f64, SciError> {
    if a >= b || n == 0 {
        return Err(SciError::InvalidInput("Invalid input parameters".to_string()));
    }

    let h = (b - a) / n as f64;
    let mut sum = 0.5 * (f(a) + f(b));
    for i in 1..n {
        sum += f(a + i as f64 * h);
    }
    Ok(h * sum)
}