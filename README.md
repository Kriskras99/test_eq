# `test_eq!`
[`assert_eq!`][assert_eq]-like macros that return a Result instead.

The primary use case is in writing parsers, where you want to check that magic values are correct and that values
are as expected. The macros are implemented using `macro_rules` and should therefore not have a significant impact
on compile times.

## Usage
This crate contains two kinds of macros:
1. Macros that check variables are as expected ([`test_eq!`][test_eq], [`test_any!`][test_any], …).
2. Macros that compose the test macros ([`test_and!`][test_and] and [`test_or!`][test_or]).

## Examples
```rust
use test_eq::{TestFailure, test_eq, test_and, test_or, test_any, test_ge};

pub fn parser() -> Result<(), TestFailure> {
    let magic: u32 = todo!();
    test_eq!(magic, 0xDEAD_BEEF)?;
    let version: u8 = todo!();
    test_any!(version, 1..4, "unsupported version found")?;
    let field: String = todo!();
    test_or!(test_ge!(field.len(), 8), test_eq!(field, "spam"))?;
    let field2: u32 = todo!();
    test_and!(
        test_any!(field2, 42..=2048),
        test_eq!(field2 % 2, 0),
        "related field: {}", field
    )?;
}

```

## Features
### `line-info`
Provide the location in the source file where the error happened. This feature is enabled by default.
This information is set at compile time and cannot be removed with `debug=false` or `strip=true`.

[assert_eq]: https://doc.rust-lang.org/std/macro.assert_eq.html
[test_eq]: https://docs.rs/test_eq/latest/test_eq/macro.test_eq.html
[test_any]: https://docs.rs/test_eq/latest/test_eq/macro.test_any.html
[test_and]: https://docs.rs/test_eq/latest/test_eq/macro.test_and.html
[test_or]: https://docs.rs/test_eq/latest/test_eq/macro.test_or.html

## Copyright
The implementation of these macros is based on the implementations of the `assert*!` macros in the standard library.
The Rust standard library is dual-licensed under the MIT and Apache-2.0 licenses just like this library.
