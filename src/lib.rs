#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]

use std::fmt::{Debug, Display, Formatter};

mod macros;

/// An error returned when a test in one of the macros fails.
///
/// The error message will display the expected value and the actual value. If the input was not
/// a literal it will also show the variable name.
///
/// When the `line-info` feature is enabled, the error message will show the source file, line and column
/// of the failed test.
pub struct TestFailure {
    /// The failure message.
    error: String,
}

impl std::error::Error for TestFailure {}

impl Display for TestFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(&self.error)
    }
}

impl Debug for TestFailure {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        <Self as Display>::fmt(self, f)
    }
}

impl TestFailure {
    /// Create a failed test from the given `message` and optional `args`, showing the values of `.*val`.
    ///
    /// `left_ident` is the name of `left_val`.
    /// `right_ident` is the name of `right_val`.
    #[doc(hidden)]
    #[inline(never)]
    #[cold]
    pub fn test_failed_two_idents<T, U>(
        message: &'static str,
        first_ident: &'static str,
        first_val: &T,
        second_ident: &'static str,
        second_val: &U,
        args: Option<std::fmt::Arguments<'_>>,
    ) -> Self
    where
        T: std::fmt::Debug + ?Sized,
        U: std::fmt::Debug + ?Sized,
    {
        Self::test_failed_inner_two_idents(
            message,
            first_ident,
            &first_val,
            second_ident,
            &second_val,
            args,
        )
    }

    /// Non-generic version of [`test_failed_two_idents`] to reduce code bloat.
    #[doc(hidden)]
    fn test_failed_inner_two_idents(
        message: &'static str,
        first_ident: &'static str,
        first_val: &dyn std::fmt::Debug,
        second_ident: &'static str,
        second_val: &dyn std::fmt::Debug,
        args: Option<std::fmt::Arguments<'_>>,
    ) -> Self {
        let error = match args {
            Some(args) => format!(
                "{message}: {args}\n{first_ident}: {first_val:?}\n{second_ident}: {second_val:?}"
            ),
            None => {
                format!("{message}\n{first_ident}: {first_val:?}\n{second_ident}: {second_val:?}")
            }
        };

        Self { error }
    }

    /// Create a failed test from the given `message` and optional `args`, showing the value of `val`.
    ///
    /// `ident` is the name of `val`.
    #[doc(hidden)]
    #[inline(never)]
    #[cold]
    pub fn test_failed_one_ident<T>(
        message: &'static str,
        ident: &'static str,
        val: &T,
        args: Option<std::fmt::Arguments<'_>>,
    ) -> Self
    where
        T: std::fmt::Debug + ?Sized,
    {
        Self::test_failed_inner_one_ident(message, ident, &val, args)
    }

    /// Non-generic version of [`test_failed_one_ident`] to reduce code bloat.
    #[doc(hidden)]
    fn test_failed_inner_one_ident(
        message: &'static str,
        ident: &'static str,
        val: &dyn std::fmt::Debug,
        args: Option<std::fmt::Arguments<'_>>,
    ) -> Self {
        let error = match args {
            Some(args) => format!("{message}: {args}\n{ident}: {val:?}"),
            None => format!("{message}\n{ident}: {val:?}"),
        };

        Self { error }
    }

    /// Create a failed test from the given `message` and optional `args`.
    #[doc(hidden)]
    #[inline(never)]
    #[cold]
    pub fn test_failed_no_ident<T>(
        message: &'static str,
        args: Option<std::fmt::Arguments<'_>>,
    ) -> Self
    where
        T: std::fmt::Debug + ?Sized,
    {
        let error = match args {
            Some(args) => format!("{message}: {args}"),
            None => message.to_string(),
        };

        Self { error }
    }

    /// Create a failed test from two failed test.
    #[doc(hidden)]
    #[inline(never)]
    #[cold]
    pub fn two_tests_failed(
        first: Self,
        second: Self,
        args: Option<std::fmt::Arguments<'_>>,
    ) -> Self {
        // offset the error messages by 3 spaces for clarity
        let mut first = first.error;
        let mut second = second.error;
        let mut start_of_search = 0;
        while let Some(position) = first[start_of_search..].find('\n') {
            first.insert_str(start_of_search + position + 1, "   ");
            start_of_search += position + 3;
        }
        let mut start_of_search = 0;
        while let Some(position) = second[start_of_search..].find('\n') {
            second.insert_str(start_of_search + position + 1, "   ");
            start_of_search += position + 3;
        }
        let error = if let Some(args) = args {
            format!("Both tests failed: {args}\n1: {first}\n2: {second}")
        } else {
            format!("Both tests failed:\n1: {first}\n2: {second}")
        };
        Self { error }
    }

    /// Create a failed test from one failed test.
    #[doc(hidden)]
    #[inline(never)]
    #[cold]
    pub fn one_test_failed(failure: Self, args: Option<std::fmt::Arguments<'_>>) -> Self {
        // offset the error message by 3 spaces for clarity
        let mut failure = failure.error;
        let mut start_of_search = 0;
        while let Some(position) = failure[start_of_search..].find('\n') {
            failure.insert_str(start_of_search + position + 1, "   ");
            start_of_search += position + 3;
        }
        let error = if let Some(args) = args {
            format!("One of the tests failed: {args}\n   {failure}")
        } else {
            format!("One of the tests failed: {failure}")
        };
        Self { error }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_test_eq() {
        let a = 5;
        let b = 19;
        assert!(test_eq!(a, b).is_err());
        let a = "5";
        let b = "19";
        assert!(test_eq!(a, b).is_err());
        let a = "5";
        let b = "19".to_string();
        assert!(test_eq!(a, b).is_err());
        let a = 42;
        let b = 42;
        assert!(test_eq!(a, b).is_ok());
        let a = "42";
        let b = "42";
        assert!(test_eq!(a, b).is_ok());
        let a = "42";
        let b = "42".to_string();
        assert!(test_eq!(a, b).is_ok());
        let a = "hello";
        let b = "world";
        assert!(test_eq!(a, b).is_err());
    }

    #[test]
    pub fn test_test_ne() {
        let a = 5;
        let b = 19;
        assert!(test_ne!(a, b).is_ok());
        let a = "5";
        let b = "19";
        assert!(test_ne!(a, b).is_ok());
        let a = "5";
        let b = "19".to_string();
        assert!(test_ne!(a, b).is_ok());
        let a = 42;
        let b = 42;
        assert!(test_ne!(a, b).is_err());
        let a = "42";
        let b = "42";
        assert!(test_ne!(a, b).is_err());
        let a = "42";
        let b = "42".to_string();
        assert!(test_ne!(a, b).is_err());
    }

    #[test]
    pub fn test_test_ge() {
        let a = 5;
        let b = 19;
        assert!(test_ge!(a, b).is_err());
        assert!(test_ge!(b, a).is_ok());
        let a = 'a';
        let b = 'b';
        assert!(test_ge!(a, b).is_err());
        assert!(test_ge!(b, a).is_ok());
        let a = 42;
        let b = 42;
        assert!(test_ge!(a, b).is_ok());
        assert!(test_ge!(b, a).is_ok());
        let a = 5;
        let b = 10;
        assert!(test_ge!(a, b).is_err());
        assert!(test_ge!(b, a).is_ok());
    }

    #[test]
    pub fn test_test_or() {
        let a = 5;
        let b = 10;
        let c = "hello";
        let d = "world";
        assert!(test_or!(test_ge!(b, a), test_eq!(c, d)).is_ok());
        assert!(test_or!(test_ge!(a, b), test_eq!(c, d)).is_err());
    }
}
