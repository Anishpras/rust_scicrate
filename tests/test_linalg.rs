use approx::assert_relative_eq;
use ndarray::{array, Array2};
use rust_scicrate::linalg::{matrix_multiply, vector_norm};

#[test]
fn test_matrix_multiply() {
    let a = array![[1., 2.], [3., 4.]];
    let b = array![[5., 6.], [7., 8.]];
    let result = matrix_multiply(&a, &b).unwrap();
    let expected = array![[19., 22.], [43., 50.]];
    assert_eq!(result, expected);
}

#[test]
fn test_vector_norm() {
    let v = array![3., 4.];
    assert_relative_eq!(vector_norm(&v), 5., epsilon = 1e-10);
}