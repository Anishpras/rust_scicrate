# Rust SciCrate

Rust SciCrate is a scientific computing library for Rust, inspired by SciPy.

## Features

- Linear algebra operations
- Statistical functions
- Optimization algorithms
- Numerical integration

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
rust_scicrate = "0.1.0"
```

## Usage

Here's a quick example of how to use Rust SciCrate:

```rust
use rust_scicrate::linalg::matrix_multiply;
use ndarray::array;

fn main() {
    let a = array![[1., 2.], [3., 4.]];
    let b = array![[5., 6.], [7., 8.]];
    let c = matrix_multiply(&a, &b).unwrap();
    println!("Result of matrix multiplication:\n{}", c);
}
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT.
