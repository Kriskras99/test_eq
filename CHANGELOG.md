# Changelog

# 0.2.0
- Fix the `line-info` feature. 
    - In older versions the feature would only work if it was
      enabled on the codebase the library was used in.
    - Now it needs to be enabled on the `test_eq` crate.

# 0.1.1
- Make `test_or!` lazy
- Explain that `test_or!` and `test_and!` are composable with themselves
- Enable extra `rustdoc` lints
- Miscellaneous documentation improvements

# 0.1.0
- Initial release