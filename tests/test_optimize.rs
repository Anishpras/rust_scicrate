use approx::assert_relative_eq;
use rust_scicrate::optimize::golden_section_search;

#[test]
fn test_golden_section_search() {
    let f = |x: f64| (x - 2.).powi(2);
    let result = golden_section_search(f, 0., 4., 1e-6, 100).unwrap();
    assert_relative_eq!(result, 2., epsilon = 1e-6);
}