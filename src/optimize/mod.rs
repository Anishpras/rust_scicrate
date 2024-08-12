use crate::error::SciError;

pub fn golden_section_search<F: Fn(f64) -> f64>(f: F, a: f64, b: f64, tol: f64, max_iter: usize) -> Result<f64, SciError> {
    if a >= b || tol <= 0.0 {
        return Err(SciError::InvalidInput("Invalid input parameters".to_string()));
    }

    let phi = (5.0_f64.sqrt() + 1.0) / 2.0;
    let mut a = a;
    let mut b = b;
    let mut c = b - (b - a) / phi;
    let mut d = a + (b - a) / phi;

    for _ in 0..max_iter {
        if (b - a).abs() < tol {
            return Ok((a + b) / 2.0);
        }
        if f(c) < f(d) {
            b = d;
        } else {
            a = c;
        }
        c = b - (b - a) / phi;
        d = a + (b - a) / phi;
    }

    Err(SciError::ComputationError("Maximum iterations reached".to_string()))
}