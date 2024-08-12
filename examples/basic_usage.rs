use ndarray::{array, Array2};
use rust_scicrate::linalg::matrix_multiply;
use rust_scicrate::stats::mean;
use rust_scicrate::optimize::golden_section_search;
use rust_scicrate::integrate::trapezoidal;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Linear algebra example
    let a = array![[1., 2.], [3., 4.]];
    let b = array![[5., 6.], [7., 8.]];
    let c = matrix_multiply(&a, &b)?;
    println!("Matrix multiplication result:\n{}", c);

    // Statistics example
    let data = array![1., 2., 3., 4., 5.];
    let mean_value = mean(&data)?;
    println!("Mean: {}", mean_value);

    // Optimization example
    let f = |x: f64| (x - 2.).powi(2);
    let min = golden_section_search(f, 0., 4., 1e-6, 100)?;
    println!("Minimum found at x = {}", min);

    // Integration example
    let g = |x: f64| x.sin();
    let integral = trapezoidal(g, 0., std::f64::consts::PI, 1000)?;
    println!("Integral of sin(x) from 0 to pi: {}", integral);

    Ok(())
}