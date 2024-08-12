# Rust SciCrate

Rust SciCrate is a scientific computing library for Rust, inspired by SciPy. It aims to provide a comprehensive set of tools for numerical operations, statistical analysis, optimization, and more, all implemented in pure Rust for maximum performance and safety.

## Features

- **Linear Algebra**: Basic matrix operations, including multiplication and vector norms.
- **Statistics**: Fundamental statistical functions like mean and standard deviation.
- **Optimization**: Implementation of optimization algorithms, including Golden Section Search.
- **Numerical Integration**: Trapezoidal rule for numerical integration.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
rust_scicrate = "0.1.0"
```

## Usage

Here are some examples of how to use Rust SciCrate:

### Linear Algebra

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

### Statistics

```rust
use rust_scicrate::stats::{mean, standard_deviation};
use ndarray::array;

fn main() {
    let data = array![1., 2., 3., 4., 5.];
    println!("Mean: {}", mean(&data).unwrap());
    println!("Standard Deviation: {}", standard_deviation(&data).unwrap());
}
```

### Optimization

```rust
use rust_scicrate::optimize::golden_section_search;

fn main() {
    let f = |x: f64| (x - 2.).powi(2);
    let min = golden_section_search(f, 0., 4., 1e-6, 100).unwrap();
    println!("Minimum found at x = {}", min);
}
```

### Numerical Integration

```rust
use rust_scicrate::integrate::trapezoidal;
use std::f64::consts::PI;

fn main() {
    let f = |x: f64| x.sin();
    let integral = trapezoidal(f, 0., PI, 1000).unwrap();
    println!("Integral of sin(x) from 0 to pi: {}", integral);
}
```

## Documentation

For more detailed documentation, please visit [docs.rs/rust_scicrate](https://docs.rs/rust_scicrate).

## Contributing

We welcome contributions to Rust SciCrate! Here are ways you can contribute:

1. Implement new features or improve existing ones
2. Write tests to increase code coverage
3. Improve documentation
4. Report bugs and suggest enhancements

To contribute:

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

Please make sure to update tests as appropriate and adhere to the existing coding style.

## Roadmap

- Implement more advanced linear algebra operations (e.g., eigenvalue computation, matrix decompositions)
- Add more statistical functions and distributions
- Implement additional optimization algorithms
- Add support for solving ordinary differential equations
- Implement Fourier transforms

## License

This project is licensed under

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Acknowledgments

- Inspired by the SciPy library for Python
- Built with Rust and love for scientific computing

## Contact

If you want to contact me you can reach me at <anishprashun118@gmail.com>.

## Disclaimer

This library is in early stages of development. While we strive for correctness and efficiency, it may contain bugs or inefficiencies. Use with caution in production environments.
