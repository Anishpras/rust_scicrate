use approx::assert_relative_eq;
use std::f64::consts::PI;
use rust_scicrate::integrate::trapezoidal;

#[test]
fn test_trapezoidal() {
    let f = |x: f64| x.sin();
    let result = trapezoidal(f, 0., PI, 1000).unwrap();
    assert_relative_eq!(result, 2., epsilon = 1e-3);
}