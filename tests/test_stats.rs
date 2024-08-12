use approx::assert_relative_eq;
use ndarray::array;
use rust_scicrate::stats::{mean, standard_deviation};

#[test]
fn test_mean() {
    let data = array![1., 2., 3., 4., 5.];
    assert_relative_eq!(mean(&data).unwrap(), 3., epsilon = 1e-10);
}

#[test]
fn test_standard_deviation() {
    let data = array![2., 4., 4., 4., 5., 5., 7., 9.];
    assert_relative_eq!(standard_deviation(&data).unwrap(), 2.138089935299395, epsilon = 1e-10);
}