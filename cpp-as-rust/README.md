# C++ as Rust (CSR)

CSR is a set of header files that aims to provide Rust-like programming experience in C++. It implements `Option`, `Result`, and a list of macros and types.

# Installation

1. Clone the repository
2. Include `csr.h` to your project
3. Enjoy rust-like programming experience in C++

# Usage

## Types

All the rust basic types: signed/unsigned numbers (exclude `isize`), float numbers...

These types are defined in `types.h`.

## Macros

- `print`, `println`, `eprint`, `eprintln`
- `assert_eq` for assertion of equality
- `line`/`file` expands to the "line number" / "filename" on which it was invoked.
- Not recommended: `panic`, `unimplemented`, `todo`.
    - They are not recommended because they use `std::exit(EXIT_FAILURE)` to exit the program.

## Option

- `Option` can be constructed by using `Some(T)` and `None()`
- `is_some()` and `is_none()` can be used to inspect whether there are certain values inside the `Option`
- `expect()` and `unwrap()` can be used to acquire a reference to the value inside the `Option`

## Result

- `Result` can be constructed by using `Ok(T)` and `Err(E)`
- `is_ok()` and `is_err()` can be used to inspect whether there are certain values inside the `Result`
- `expect()` and `unwrap()` can be used to acquire a reference to the value inside the `Result`
- `expect_err()` and `unwrap_err()` can be used to acquire a reference to the error inside the `Result`
- `Result` must be used if they are returned from functions.

# License

[MIT License](./LICENSE)