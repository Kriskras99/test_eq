//! The implementations of the macros.

/// Tests that two expressions are equal to each other (using [`PartialEq`]).
///
/// This macro returns a [`Result`] and hints the compiler that the failure
/// case is unlikely to happen.
///
/// A custom message can be added, with [`format!`] support.
///
/// # Examples
/// ```
/// use test_eq::test_eq;
/// let a = 3;
/// let b = 1 + 2;
/// let c = b * 2;
/// test_eq!(a, b).expect("This is true");
/// println!("{:?}", test_eq!(a, c, "and b is {}", b));
/// // prints:
/// // Err([src/main.rs:5:1]: Test failed: a != c: and b is 3
/// // a: 3
/// // c: 6)
/// ```
#[macro_export]
macro_rules! test_eq {
    ($left:expr, $right:literal $(,)?) => {{
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(left_val == right_val) {
                    // "[src/main:2:5]: Test failed: a * 2 != b * 5"
                    #[cfg(feature = "line-info")]
                    let message = ::std::concat!(
                        '[', ::std::file!(), ':', ::std::line!(), ':', ::std::column!(), "]: Test failed: ",
                        ::std::stringify!($left), " != ", ::std::stringify!($right)
                    );

                    // "Test failed: a * 2 != b * 5"
                    #[cfg(not(feature = "line-info"))]
                    let message = ::std::concat!(
                        "Test failed: ", ::std::stringify!($left), " != ", ::std::stringify!($right)
                    );

                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down.
                    ::std::result::Result::Err($crate::TestFailure::test_failed_one_ident(message, ::std::stringify!($left), &*left_val, ::std::option::Option::None))
                } else {
                    ::std::result::Result::Ok(())
                }
            }
        }
    }};
    ($left:literal, $right:expr $(,)?) => {{
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(left_val == right_val) {
                    // "[src/main:2:5]: Test failed: a * 2 != b * 5"
                    #[cfg(feature = "line-info")]
                    let message = ::std::concat!(
                        '[', ::std::file!(), ':', ::std::line!(), ':', ::std::column!(), "]: Test failed: ",
                        ::std::stringify!($left), " != ", ::std::stringify!($right)
                    );

                    // "Test failed: a * 2 != b * 5"
                    #[cfg(not(feature = "line-info"))]
                    let message = ::std::concat!(
                        "Test failed: ", ::std::stringify!($left), " != ", ::std::stringify!($right)
                    );

                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down.
                    ::std::result::Result::Err($crate::TestFailure::test_failed_one_ident(message, ::std::stringify!($right), &*right_val, ::std::option::Option::None))
                } else {
                    ::std::result::Result::Ok(())
                }
            }
        }
    }};
    ($left:expr, $right:expr $(,)?) => {{
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(left_val == right_val) {
                    // "[src/main:2:5]: Test failed: a * 2 != b * 5"
                    #[cfg(feature = "line-info")]
                    let message = ::std::concat!(
                        '[', ::std::file!(), ':', ::std::line!(), ':', ::std::column!(), "]: Test failed: ",
                        ::std::stringify!($left), " != ", ::std::stringify!($right)
                    );

                    // "Test failed: a * 2 != b * 5"
                    #[cfg(not(feature = "line-info"))]
                    let message = ::std::concat!(
                        "Test failed: ", ::std::stringify!($left), " != ", ::std::stringify!($right)
                    );

                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down.
                    ::std::result::Result::Err($crate::TestFailure::test_failed_two_idents(message, ::std::stringify!($left), &*left_val, ::std::stringify!($right), &*right_val, ::std::option::Option::None))
                } else {
                    ::std::result::Result::Ok(())
                }
            }
        }
    }};
    ($left:literal, $right:expr, $($arg:tt)+) => {{
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(left_val == right_val) {
                    // "[src/main:2:5]: Test failed: a * 2 != b * 5"
                    #[cfg(feature = "line-info")]
                    let message = ::std::concat!(
                        '[', ::std::file!(), ':', ::std::line!(), ':', ::std::column!(), "]: Test failed: ",
                        ::std::stringify!($left), " != ", ::std::stringify!($right)
                    );

                    // "Test failed: a * 2 != b * 5"
                    #[cfg(not(feature = "line-info"))]
                    let message = ::std::concat!(
                        "Test failed: ", ::std::stringify!($left), " != ", ::std::stringify!($right)
                    );

                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down.
                    ::std::result::Result::Err($crate::TestFailure::test_failed_one_ident(message, ::std::stringify!($right), &*right_val, ::std::option::Option::Some(::std::format_args!($($arg)+))))
                } else {
                    ::std::result::Result::Ok(())
                }
            }
        }
    }};
    ($left:expr, $right:literal, $($arg:tt)+) => {{
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(left_val == right_val) {
                    // "[src/main:2:5]: Test failed: a * 2 != b * 5"
                    #[cfg(feature = "line-info")]
                    let message = ::std::concat!(
                        '[', ::std::file!(), ':', ::std::line!(), ':', ::std::column!(), "]: Test failed: ",
                        ::std::stringify!($left), " != ", ::std::stringify!($right)
                    );

                    // "Test failed: a * 2 != b * 5"
                    #[cfg(not(feature = "line-info"))]
                    let message = ::std::concat!(
                        "Test failed: ", ::std::stringify!($left), " != ", ::std::stringify!($right)
                    );

                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down.
                    ::std::result::Result::Err($crate::TestFailure::test_failed_one_ident(message, ::std::stringify!($left), &*left_val, ::std::option::Option::Some(::std::format_args!($($arg)+))))
                } else {
                    ::std::result::Result::Ok(())
                }
            }
        }
    }};
    ($left:expr, $right:expr, $($arg:tt)+) => {{
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(left_val == right_val) {
                    // "[src/main:2:5]: Test failed: a * 2 != b * 5"
                    #[cfg(feature = "line-info")]
                    let message = ::std::concat!(
                        '[', ::std::file!(), ':', ::std::line!(), ':', ::std::column!(), "]: Test failed: ",
                        ::std::stringify!($left), " != ", ::std::stringify!($right)
                    );

                    // "Test failed: a * 2 != b * 5"
                    #[cfg(not(feature = "line-info"))]
                    let message = ::std::concat!(
                        "Test failed: ", ::std::stringify!($left), " != ", ::std::stringify!($right)
                    );

                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down.
                    ::std::result::Result::Err($crate::TestFailure::test_failed_two_idents(message, ::std::stringify!($left), &*left_val, ::std::stringify!($right), &*right_val, ::std::option::Option::Some(::std::format_args!($($arg)+))))
                } else {
                    ::std::result::Result::Ok(())
                }
            }
        }
    }};
}

/// Tests that two expressions are not equal to each other (using [`PartialEq`]).
///
/// This macro returns a [`Result`] and hints the compiler that the failure
/// case is unlikely to happen.
///
/// A custom message can be added, with [`format!`] support.
///
/// # Examples
/// ```
/// use test_eq::test_ne;
/// let a = 3;
/// let b = 1 + 2;
/// let c = b * 2;
/// test_ne!(a, c).expect("This is true");
/// println!("{:?}", test_ne!(a, b, "and c is {}", c));
/// // prints:
/// // Err([src/main.rs:5:1]: Test failed: a == b: and c is 6
/// // a: 3
/// // b: 3)
/// ```
#[macro_export]
macro_rules! test_ne {
    ($left:expr, $right:literal $(,)?) => {{
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(left_val != right_val) {
                    // "[src/main:2:5]: Test failed: a * 2 == b * 5"
                    #[cfg(feature = "line-info")]
                    let message = ::std::concat!(
                        '[', ::std::file!(), ':', ::std::line!(), ':', ::std::column!(), "]: Test failed: ",
                        ::std::stringify!($left), " == ", ::std::stringify!($right)
                    );

                    // "Test failed: a * 2 == b * 5"
                    #[cfg(not(feature = "line-info"))]
                    let message = ::std::concat!(
                        "Test failed: ", ::std::stringify!($left), " == ", ::std::stringify!($right)
                    );

                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down.
                    ::std::result::Result::Err($crate::TestFailure::test_failed_one_ident(message, ::std::stringify!($left), &*left_val, ::std::option::Option::None))
                } else {
                    ::std::result::Result::Ok(())
                }
            }
        }
    }};
    ($left:literal, $right:expr $(,)?) => {{
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(left_val != right_val) {
                    // "[src/main:2:5]: Test failed: a * 2 == b * 5"
                    #[cfg(feature = "line-info")]
                    let message = ::std::concat!(
                        '[', ::std::file!(), ':', ::std::line!(), ':', ::std::column!(), "]: Test failed: ",
                        ::std::stringify!($left), " == ", ::std::stringify!($right)
                    );

                    // "Test failed: a * 2 == b * 5"
                    #[cfg(not(feature = "line-info"))]
                    let message = ::std::concat!(
                        "Test failed: ", ::std::stringify!($left), " == ", ::std::stringify!($right)
                    );

                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down.
                    ::std::result::Result::Err($crate::TestFailure::test_failed_one_ident(message, ::std::stringify!($right), &*right_val, ::std::option::Option::None))
                } else {
                    ::std::result::Result::Ok(())
                }
            }
        }
    }};
    ($left:expr, $right:expr $(,)?) => {{
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(left_val != right_val) {
                    // "[src/main:2:5]: Test failed: a * 2 == b * 5"
                    #[cfg(feature = "line-info")]
                    let message = ::std::concat!(
                        '[', ::std::file!(), ':', ::std::line!(), ':', ::std::column!(), "]: Test failed: ",
                        ::std::stringify!($left), " == ", ::std::stringify!($right)
                    );

                    // "Test failed: a * 2 == b * 5"
                    #[cfg(not(feature = "line-info"))]
                    let message = ::std::concat!(
                        "Test failed: ", ::std::stringify!($left), " == ", ::std::stringify!($right)
                    );

                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down.
                    ::std::result::Result::Err($crate::TestFailure::test_failed_two_idents(message, ::std::stringify!($left), &*left_val, ::std::stringify!($right), &*right_val, ::std::option::Option::None))
                } else {
                    ::std::result::Result::Ok(())
                }
            }
        }
    }};
    ($left:literal, $right:expr, $($arg:tt)+) => {{
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(left_val != right_val) {
                    // "[src/main:2:5]: Test failed: a * 2 == b * 5"
                    #[cfg(feature = "line-info")]
                    let message = ::std::concat!(
                        '[', ::std::file!(), ':', ::std::line!(), ':', ::std::column!(), "]: Test failed: ",
                        ::std::stringify!($left), " == ", ::std::stringify!($right)
                    );

                    // "Test failed: a * 2 == b * 5"
                    #[cfg(not(feature = "line-info"))]
                    let message = ::std::concat!(
                        "Test failed: ", ::std::stringify!($left), " == ", ::std::stringify!($right)
                    );

                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down.
                    ::std::result::Result::Err($crate::TestFailure::test_failed_one_ident(message, ::std::stringify!($right), &*right_val, ::std::option::Option::Some(::std::format_args!($($arg)+))))
                } else {
                    ::std::result::Result::Ok(())
                }
            }
        }
    }};
    ($left:expr, $right:literal, $($arg:tt)+) => {{
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(left_val != right_val) {
                    // "[src/main:2:5]: Test failed: a * 2 == b * 5"
                    #[cfg(feature = "line-info")]
                    let message = ::std::concat!(
                        '[', ::std::file!(), ':', ::std::line!(), ':', ::std::column!(), "]: Test failed: ",
                        ::std::stringify!($left), " == ", ::std::stringify!($right)
                    );

                    // "Test failed: a * 2 == b * 5"
                    #[cfg(not(feature = "line-info"))]
                    let message = ::std::concat!(
                        "Test failed: ", ::std::stringify!($left), " == ", ::std::stringify!($right)
                    );

                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down.
                    ::std::result::Result::Err($crate::TestFailure::test_failed_one_ident(message, ::std::stringify!($left), &*left_val, ::std::option::Option::Some(::std::format_args!($($arg)+))))
                } else {
                    ::std::result::Result::Ok(())
                }
            }
        }
    }};
    ($left:expr, $right:expr, $($arg:tt)+) => {{
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(left_val != right_val) {
                    // "[src/main:2:5]: Test failed: a * 2 == b * 5"
                    #[cfg(feature = "line-info")]
                    let message = ::std::concat!(
                        '[', ::std::file!(), ':', ::std::line!(), ':', ::std::column!(), "]: Test failed: ",
                        ::std::stringify!($left), " == ", ::std::stringify!($right)
                    );

                    // "Test failed: a * 2 == b * 5"
                    #[cfg(not(feature = "line-info"))]
                    let message = ::std::concat!(
                        "Test failed: ", ::std::stringify!($left), " == ", ::std::stringify!($right)
                    );

                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down.
                    ::std::result::Result::Err($crate::TestFailure::test_failed_two_idents(message, ::std::stringify!($left), &*left_val, ::std::stringify!($right), &*right_val, ::std::option::Option::Some(::std::format_args!($($arg)+))))
                } else {
                    ::std::result::Result::Ok(())
                }
            }
        }
    }};
}

/// Tests that the left expression is any of the values in the right expression.
///
/// The right expression can be anything that results in an item that has a `.contains()` function.
/// For example, slices, [`Vec`]s, ranges, ...
///
/// This macro returns a [`Result`] and hints the compiler that the failure
/// case is unlikely to happen.
///
/// A custom message can be added, with [`format!`] support.
///
/// # Examples
/// ```
/// use test_eq::test_any;
/// let a = 3;
/// let b = a * 2;
/// test_any!(a, [1, 3, 5, 7]).expect("This is true");
/// println!("{:?}", test_any!(b, [1, 3, 5, 7], "and a is {}", a));
/// // prints:
/// // Err([src/main.rs:5:1]: Test failed: ![1, 3, 5, 7].contains(b): and a is 3
/// // b: 6)
/// ```
#[macro_export]
macro_rules! test_any {
    ($left:expr, $right:literal $(,)?) => {{
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !((right_val).contains(left_val)) {
                    // "[src/main:2:5]: Test failed: ![5, 10, 15].contains(unk1)"
                    #[cfg(feature = "line-info")]
                    let message = ::std::concat!(
                        '[', ::std::file!(), ':', ::std::line!(), ':', ::std::column!(), "]: Test failed: !",
                        ::std::stringify!($right), ".contains(", ::std::stringify!($left), ')'
                    );

                    // "Test failed: ![5, 10, 15].contains(unk1)"
                    #[cfg(not(feature = "line-info"))]
                    let message = ::std::concat!(
                        "Test failed: !", ::std::stringify!($right), ".contains(", ::std::stringify!($left), ')'
                    );

                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down.
                    ::std::result::Result::Err($crate::TestFailure::test_failed_one_ident(message, ::std::stringify!($left), &*left_val, ::std::option::Option::None))
                } else {
                    ::std::result::Result::Ok(())
                }
            }
        }
    }};
    ($left:literal, $right:expr $(,)?) => {{
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !((right_val).contains(left_val)) {
                    // "[src/main:2:5]: Test failed: ![5, 10, 15].contains(unk1)"
                    #[cfg(feature = "line-info")]
                    let message = ::std::concat!(
                        '[', ::std::file!(), ':', ::std::line!(), ':', ::std::column!(), "]: Test failed: !",
                        ::std::stringify!($right), ".contains(", ::std::stringify!($left), ')'
                    );

                    // "Test failed: ![5, 10, 15].contains(unk1)"
                    #[cfg(not(feature = "line-info"))]
                    let message = ::std::concat!(
                        "Test failed: !", ::std::stringify!($right), ".contains(", ::std::stringify!($left), ')'
                    );

                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down.
                    ::std::result::Result::Err($crate::TestFailure::test_failed_one_ident(message, ::std::stringify!($right), &*right_val, ::std::option::Option::None))
                } else {
                    ::std::result::Result::Ok(())
                }
            }
        }
    }};
    ($left:expr, $right:expr $(,)?) => {{
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !((right_val).contains(left_val)) {
                    // "[src/main:2:5]: Test failed: ![5, 10, 15].contains(unk1)"
                    #[cfg(feature = "line-info")]
                    let message = ::std::concat!(
                        '[', ::std::file!(), ':', ::std::line!(), ':', ::std::column!(), "]: Test failed: !",
                        ::std::stringify!($right), ".contains(", ::std::stringify!($left), ')'
                    );

                    // "Test failed: ![5, 10, 15].contains(unk1)"
                    #[cfg(not(feature = "line-info"))]
                    let message = ::std::concat!(
                        "Test failed: !", ::std::stringify!($right), ".contains(", ::std::stringify!($left), ')'
                    );

                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down.
                    ::std::result::Result::Err($crate::TestFailure::test_failed_two_idents(message, ::std::stringify!($left), &*left_val, ::std::stringify!($right), &*right_val, ::std::option::Option::None))
                } else {
                    ::std::result::Result::Ok(())
                }
            }
        }
    }};
    ($left:expr, $right:literal, $($arg:tt)+) => {{
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !((right_val).contains(left_val)) {
                    // "[src/main:2:5]: Test failed: ![5, 10, 15].contains(unk1)"
                    #[cfg(feature = "line-info")]
                    let message = ::std::concat!(
                        '[', ::std::file!(), ':', ::std::line!(), ':', ::std::column!(), "]: Test failed: !",
                        ::std::stringify!($right), ".contains(", ::std::stringify!($left), ')'
                    );

                    // "Test failed: ![5, 10, 15].contains(unk1)"
                    #[cfg(not(feature = "line-info"))]
                    let message = ::std::concat!(
                        "Test failed: !", ::std::stringify!($right), ".contains(", ::std::stringify!($left), ')'
                    );

                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down.
                    ::std::result::Result::Err($crate::TestFailure::test_failed_one_ident(message, ::std::stringify!($left), &*left_val, ::std::option::Option::Some(::std::format_args!($($arg)+))))
                } else {
                    ::std::result::Result::Ok(())
                }
            }
        }
    }};
    ($left:literal, $right:expr, $($arg:tt)+) => {{
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !((right_val).contains(left_val)) {
                    // "[src/main:2:5]: Test failed: ![5, 10, 15].contains(unk1)"
                    #[cfg(feature = "line-info")]
                    let message = ::std::concat!(
                        '[', ::std::file!(), ':', ::std::line!(), ':', ::std::column!(), "]: Test failed: !",
                        ::std::stringify!($right), ".contains(", ::std::stringify!($left), ')'
                    );

                    // "Test failed: ![5, 10, 15].contains(unk1)"
                    #[cfg(not(feature = "line-info"))]
                    let message = ::std::concat!(
                        "Test failed: !", ::std::stringify!($right), ".contains(", ::std::stringify!($left), ')'
                    );

                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down.
                    ::std::result::Result::Err($crate::TestFailure::test_failed_one_ident(message, ::std::stringify!($right), &*right_val, ::std::option::Option::Some(::std::format_args!($($arg)+))))
                } else {
                    ::std::result::Result::Ok(())
                }
            }
        }
    }};
    ($left:expr, $right:expr, $($arg:tt)+) => {{
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !((right_val).contains(left_val)) {
                    // "[src/main:2:5]: Test failed: ![5, 10, 15].contains(unk1)"
                    #[cfg(feature = "line-info")]
                    let message = ::std::concat!(
                        '[', ::std::file!(), ':', ::std::line!(), ':', ::std::column!(), "]: Test failed: !",
                        ::std::stringify!($right), ".contains(", ::std::stringify!($left), ')'
                    );

                    // "Test failed: ![5, 10, 15].contains(unk1)"
                    #[cfg(not(feature = "line-info"))]
                    let message = ::std::concat!(
                        "Test failed: !", ::std::stringify!($right), ".contains(", ::std::stringify!($left), ')'
                    );

                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down.
                    ::std::result::Result::Err($crate::TestFailure::test_failed_two_idents(message, ::std::stringify!($left), &*left_val, ::std::stringify!($right), &*right_val, ::std::option::Option::Some(::std::format_args!($($arg)+))))
                } else {
                    ::std::result::Result::Ok(())
                }
            }
        }
    }};
}

/// Tests that the expression is not any of the values in the slice (using `.contains()` on the right expression).
///
/// This macro returns a [`Result`] and hints the compiler that the failure
/// case is unlikely to happen.
///
/// A custom message can be added, with [`format!`] support.
///
/// # Examples
/// ```
/// use test_eq::test_not_any;
/// let a = 3;
/// let b = a * 2;
/// test_not_any!(b, [1, 3, 5, 7]).expect("This is true");
/// println!("{:?}", test_not_any!(a, [1, 3, 5, 7], "and b is {}", b));
/// // prints:
/// // [src/main.rs:5:1]: Test failed: [1, 3, 5, 7].contains(a): and b is 6
/// // a: 3
/// // [1, 3, 5, 7]: [1, 3, 5, 7]
/// ```
#[macro_export]
macro_rules! test_not_any {
    ($left:expr, $right:literal $(,)?) => {{
        match (&$left, &$right) {
            (left_val, right_val) => {
                if ((right_val).contains(left_val)) {
                    // "[src/main:2:5]: Test failed: [5, 10, 15].contains(unk1)"
                    #[cfg(feature = "line-info")]
                    let message = ::std::concat!(
                        '[', ::std::file!(), ':', ::std::line!(), ':', ::std::column!(), "]: Test failed: ",
                        ::std::stringify!($right), ".contains(", ::std::stringify!($left), ')'
                    );

                    // "Test failed: [5, 10, 15].contains(unk1)"
                    #[cfg(not(feature = "line-info"))]
                    let message = ::std::concat!(
                        "Test failed: ", ::std::stringify!($right), ".contains(", ::std::stringify!($left), ')'
                    );

                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down.
                    ::std::result::Result::Err($crate::TestFailure::test_failed_one_ident(message, ::std::stringify!($left), &*left_val, ::std::option::Option::None))
                } else {
                    ::std::result::Result::Ok(())
                }
            }
        }
    }};
    ($left:literal, $right:expr $(,)?) => {{
        match (&$left, &$right) {
            (left_val, right_val) => {
                if ((right_val).contains(left_val)) {
                    // "[src/main:2:5]: Test failed: [5, 10, 15].contains(unk1)"
                    #[cfg(feature = "line-info")]
                    let message = ::std::concat!(
                        '[', ::std::file!(), ':', ::std::line!(), ':', ::std::column!(), "]: Test failed: ",
                        ::std::stringify!($right), ".contains(", ::std::stringify!($left), ')'
                    );

                    // "Test failed: [5, 10, 15].contains(unk1)"
                    #[cfg(not(feature = "line-info"))]
                    let message = ::std::concat!(
                        "Test failed: ", ::std::stringify!($right), ".contains(", ::std::stringify!($left), ')'
                    );

                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down.
                    ::std::result::Result::Err($crate::TestFailure::test_failed_one_ident(message, ::std::stringify!($right), &*right_val, ::std::option::Option::None))
                } else {
                    ::std::result::Result::Ok(())
                }
            }
        }
    }};
    ($left:expr, $right:expr $(,)?) => {{
        match (&$left, &$right) {
            (left_val, right_val) => {
                if ((right_val).contains(left_val)) {
                    // "[src/main:2:5]: Test failed: [5, 10, 15].contains(unk1)"
                    #[cfg(feature = "line-info")]
                    let message = ::std::concat!(
                        '[', ::std::file!(), ':', ::std::line!(), ':', ::std::column!(), "]: Test failed: ",
                        ::std::stringify!($right), ".contains(", ::std::stringify!($left), ')'
                    );

                    // "Test failed: [5, 10, 15].contains(unk1)"
                    #[cfg(not(feature = "line-info"))]
                    let message = ::std::concat!(
                        "Test failed: ", ::std::stringify!($right), ".contains(", ::std::stringify!($left), ')'
                    );

                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down.
                    ::std::result::Result::Err($crate::TestFailure::test_failed_two_idents(message, ::std::stringify!($left), &*left_val, ::std::stringify!($right), &*right_val, ::std::option::Option::None))
                } else {
                    ::std::result::Result::Ok(())
                }
            }
        }
    }};
    ($left:expr, $right:literal, $($arg:tt)+) => {{
        match (&$left, &$right) {
            (left_val, right_val) => {
                if ((right_val).contains(left_val)) {
                    // "[src/main:2:5]: Test failed: [5, 10, 15].contains(unk1)"
                    #[cfg(feature = "line-info")]
                    let message = ::std::concat!(
                        '[', ::std::file!(), ':', ::std::line!(), ':', ::std::column!(), "]: Test failed: ",
                        ::std::stringify!($right), ".contains(", ::std::stringify!($left), ')'
                    );

                    // "Test failed: [5, 10, 15].contains(unk1)"
                    #[cfg(not(feature = "line-info"))]
                    let message = ::std::concat!(
                        "Test failed: ", ::std::stringify!($right), ".contains(", ::std::stringify!($left), ')'
                    );

                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down.
                    ::std::result::Result::Err($crate::TestFailure::test_failed_one_ident(message, ::std::stringify!($left), &*left_val, ::std::option::Option::Some(::std::format_args!($($arg)+))))
                } else {
                    ::std::result::Result::Ok(())
                }
            }
        }
    }};
    ($left:literal, $right:expr, $($arg:tt)+) => {{
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !((right_val).contains(left_val)) {
                    // "[src/main:2:5]: Test failed: [5, 10, 15].contains(unk1)"
                    #[cfg(feature = "line-info")]
                    let message = ::std::concat!(
                        '[', ::std::file!(), ':', ::std::line!(), ':', ::std::column!(), "]: Test failed: ",
                        ::std::stringify!($right), ".contains(", ::std::stringify!($left), ')'
                    );

                    // "Test failed: [5, 10, 15].contains(unk1)"
                    #[cfg(not(feature = "line-info"))]
                    let message = ::std::concat!(
                        "Test failed: ", ::std::stringify!($right), ".contains(", ::std::stringify!($left), ')'
                    );

                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down.
                    ::std::result::Result::Err($crate::TestFailure::test_failed_one_ident(message, ::std::stringify!($right), &*right_val, ::std::option::Option::Some(::std::format_args!($($arg)+))))
                } else {
                    ::std::result::Result::Ok(())
                }
            }
        }
    }};
    ($left:expr, $right:expr, $($arg:tt)+) => {{
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !((right_val).contains(left_val)) {
                    // "[src/main:2:5]: Test failed: [5, 10, 15].contains(unk1)"
                    #[cfg(feature = "line-info")]
                    let message = ::std::concat!(
                        '[', ::std::file!(), ':', ::std::line!(), ':', ::std::column!(), "]: Test failed: ",
                        ::std::stringify!($right), ".contains(", ::std::stringify!($left), ')'
                    );

                    // "Test failed: [5, 10, 15].contains(unk1)"
                    #[cfg(not(feature = "line-info"))]
                    let message = ::std::concat!(
                        "Test failed: ", ::std::stringify!($right), ".contains(", ::std::stringify!($left), ')'
                    );

                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down.
                    ::std::result::Result::Err($crate::TestFailure::test_failed_two_idents(message, ::std::stringify!($left), &*left_val, ::std::stringify!($right), &*right_val, ::std::option::Option::Some(::std::format_args!($($arg)+))))
                } else {
                    ::std::result::Result::Ok(())
                }
            }
        }
    }};
}

/// Tests that the left expression is smaller or equal to the right expression (using [`PartialOrd`]).
///
/// This macro returns a [`Result`] and hints the compiler that the failure
/// case is unlikely to happen.
///
/// A custom message can be added, with [`format!`] support.
///
/// # Examples
/// ```
/// use test_eq::test_le;
/// let a = 3;
/// let b = 2;
/// let c = b * 2;
/// test_le!(a, c).expect("This is true");
/// println!("{:?}", test_le!(a, b, "and c is {}", c));
/// // prints:
/// // [src/main.rs:5:1]: Test failed: a > b: and c is 6
/// // a: 3
/// // b: 2
/// ```
#[macro_export]
macro_rules! test_le {
    ($left:expr, $right:literal $(,)?) => {{
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(left_val <= right_val) {
                    // "[src/main:2:5]: Test failed: a * 2 > b * 5"
                    #[cfg(feature = "line-info")]
                    let message = ::std::concat!(
                        '[', ::std::file!(), ':', ::std::line!(), ':', ::std::column!(), "]: Test failed: ",
                        ::std::stringify!($left), " > ", ::std::stringify!($right)
                    );

                    // "Test failed: a * 2 > b * 5"
                    #[cfg(not(feature = "line-info"))]
                    let message = ::std::concat!(
                        "Test failed: ", ::std::stringify!($left), " > ", ::std::stringify!($right)
                    );

                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down.
                    ::std::result::Result::Err($crate::TestFailure::test_failed_one_ident(message, ::std::stringify!($left), &*left_val, ::std::option::Option::None))
                } else {
                    ::std::result::Result::Ok(())
                }
            }
        }
    }};
    ($left:literal, $right:expr $(,)?) => {{
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(left_val <= right_val) {
                    // "[src/main:2:5]: Test failed: a * 2 > b * 5"
                    #[cfg(feature = "line-info")]
                    let message = ::std::concat!(
                        '[', ::std::file!(), ':', ::std::line!(), ':', ::std::column!(), "]: Test failed: ",
                        ::std::stringify!($left), " > ", ::std::stringify!($right)
                    );

                    // "Test failed: a * 2 > b * 5"
                    #[cfg(not(feature = "line-info"))]
                    let message = ::std::concat!(
                        "Test failed: ", ::std::stringify!($left), " > ", ::std::stringify!($right)
                    );

                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down.
                    ::std::result::Result::Err($crate::TestFailure::test_failed_one_ident(message, ::std::stringify!($right), &*right_val, ::std::option::Option::None))
                } else {
                    ::std::result::Result::Ok(())
                }
            }
        }
    }};
    ($left:expr, $right:expr $(,)?) => {{
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(left_val <= right_val) {
                    // "[src/main:2:5]: Test failed: a * 2 > b * 5"
                    #[cfg(feature = "line-info")]
                    let message = ::std::concat!(
                        '[', ::std::file!(), ':', ::std::line!(), ':', ::std::column!(), "]: Test failed: ",
                        ::std::stringify!($left), " > ", ::std::stringify!($right)
                    );

                    // "Test failed: a * 2 > b * 5"
                    #[cfg(not(feature = "line-info"))]
                    let message = ::std::concat!(
                        "Test failed: ", ::std::stringify!($left), " > ", ::std::stringify!($right)
                    );

                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down.
                    ::std::result::Result::Err($crate::TestFailure::test_failed_two_idents(message, ::std::stringify!($left), &*left_val, ::std::stringify!($right), &*right_val, ::std::option::Option::None))
                } else {
                    ::std::result::Result::Ok(())
                }
            }
        }
    }};
    ($left:literal, $right:expr, $($arg:tt)+) => {{
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(left_val <= right_val) {
                    // "[src/main:2:5]: Test failed: a * 2 > b * 5"
                    #[cfg(feature = "line-info")]
                    let message = ::std::concat!(
                        '[', ::std::file!(), ':', ::std::line!(), ':', ::std::column!(), "]: Test failed: ",
                        ::std::stringify!($left), " > ", ::std::stringify!($right)
                    );

                    // "Test failed: a * 2 > b * 5"
                    #[cfg(not(feature = "line-info"))]
                    let message = ::std::concat!(
                        "Test failed: ", ::std::stringify!($left), " > ", ::std::stringify!($right)
                    );

                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down.
                    ::std::result::Result::Err($crate::TestFailure::test_failed_one_ident(message, ::std::stringify!($right), &*right_val, ::std::option::Option::Some(::std::format_args!($($arg)+))))
                } else {
                    ::std::result::Result::Ok(())
                }
            }
        }
    }};
    ($left:expr, $right:literal, $($arg:tt)+) => {{
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(left_val <= right_val) {
                    // "[src/main:2:5]: Test failed: a * 2 > b * 5"
                    #[cfg(feature = "line-info")]
                    let message = ::std::concat!(
                        '[', ::std::file!(), ':', ::std::line!(), ':', ::std::column!(), "]: Test failed: ",
                        ::std::stringify!($left), " > ", ::std::stringify!($right)
                    );

                    // "Test failed: a * 2 > b * 5"
                    #[cfg(not(feature = "line-info"))]
                    let message = ::std::concat!(
                        "Test failed: ", ::std::stringify!($left), " > ", ::std::stringify!($right)
                    );

                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down.
                    ::std::result::Result::Err($crate::TestFailure::test_failed_one_ident(message, ::std::stringify!($left), &*left_val, ::std::option::Option::Some(::std::format_args!($($arg)+))))
                } else {
                    ::std::result::Result::Ok(())
                }
            }
        }
    }};
    ($left:expr, $right:expr, $($arg:tt)+) => {{
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(left_val <= right_val) {
                    // "[src/main:2:5]: Test failed: a * 2 > b * 5"
                    #[cfg(feature = "line-info")]
                    let message = ::std::concat!(
                        '[', ::std::file!(), ':', ::std::line!(), ':', ::std::column!(), "]: Test failed: ",
                        ::std::stringify!($left), " > ", ::std::stringify!($right)
                    );

                    // "Test failed: a * 2 > b * 5"
                    #[cfg(not(feature = "line-info"))]
                    let message = ::std::concat!(
                        "Test failed: ", ::std::stringify!($left), " > ", ::std::stringify!($right)
                    );

                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down.
                    ::std::result::Result::Err($crate::TestFailure::test_failed_two_idents(message, ::std::stringify!($left), &*left_val, ::std::stringify!($right), &*right_val, ::std::option::Option::Some(::std::format_args!($($arg)+))))
                } else {
                    ::std::result::Result::Ok(())
                }
            }
        }
    }};
}

/// Tests that the left expression is greater or equal to the right expression (using [`PartialOrd`]).
///
/// This macro returns a [`Result`] and hints the compiler that the failure
/// case is unlikely to happen.
///
/// A custom message can be added, with [`format!`] support.
///
/// # Examples
/// ```
/// use test_eq::test_ge;
/// let a = 3;
/// let b = 2;
/// let c = b * 2;
/// test_ge!(a, b).expect("This is true");
/// println!("{:?}", test_ge!(a, c, "and b is {}", b));
/// // prints:
/// // [src/main.rs:5:1]: Test failed: a < c: and b is 2
/// // a: 3
/// // c: 4
/// ```
#[macro_export]
macro_rules! test_ge {
    ($left:expr, $right:literal $(,)?) => {{
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(left_val >= right_val) {
                    // "[src/main:2:5]: Test failed: a * 2 < b * 5"
                    #[cfg(feature = "line-info")]
                    let message = ::std::concat!(
                        '[', ::std::file!(), ':', ::std::line!(), ':', ::std::column!(), "]: Test failed: ",
                        ::std::stringify!($left), " < ", ::std::stringify!($right)
                    );

                    // "Test failed: a * 2 < b * 5"
                    #[cfg(not(feature = "line-info"))]
                    let message = ::std::concat!(
                        "Test failed: ", ::std::stringify!($left), " < ", ::std::stringify!($right)
                    );

                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down.
                    ::std::result::Result::Err($crate::TestFailure::test_failed_one_ident(message, ::std::stringify!($left), &*left_val, ::std::option::Option::None))
                } else {
                    ::std::result::Result::Ok(())
                }
            }
        }
    }};
    ($left:literal, $right:expr $(,)?) => {{
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(left_val >= right_val) {
                    // "[src/main:2:5]: Test failed: a * 2 < b * 5"
                    #[cfg(feature = "line-info")]
                    let message = ::std::concat!(
                        '[', ::std::file!(), ':', ::std::line!(), ':', ::std::column!(), "]: Test failed: ",
                        ::std::stringify!($left), " < ", ::std::stringify!($right)
                    );

                    // "Test failed: a * 2 < b * 5"
                    #[cfg(not(feature = "line-info"))]
                    let message = ::std::concat!(
                        "Test failed: ", ::std::stringify!($left), " < ", ::std::stringify!($right)
                    );

                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down.
                    ::std::result::Result::Err($crate::TestFailure::test_failed_one_ident(message, ::std::stringify!($right), &*right_val, ::std::option::Option::None))
                } else {
                    ::std::result::Result::Ok(())
                }
            }
        }
    }};
    ($left:expr, $right:expr $(,)?) => {{
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(left_val >= right_val) {
                    // "[src/main:2:5]: Test failed: a * 2 < b * 5"
                    #[cfg(feature = "line-info")]
                    let message = ::std::concat!(
                        '[', ::std::file!(), ':', ::std::line!(), ':', ::std::column!(), "]: Test failed: ",
                        ::std::stringify!($left), " < ", ::std::stringify!($right)
                    );

                    // "Test failed: a * 2 < b * 5"
                    #[cfg(not(feature = "line-info"))]
                    let message = ::std::concat!(
                        "Test failed: ", ::std::stringify!($left), " < ", ::std::stringify!($right)
                    );

                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down.
                    ::std::result::Result::Err($crate::TestFailure::test_failed_two_idents(message, ::std::stringify!($left), &*left_val, ::std::stringify!($right), &*right_val, ::std::option::Option::None))
                } else {
                    ::std::result::Result::Ok(())
                }
            }
        }
    }};
    ($left:literal, $right:expr, $($arg:tt)+) => {{
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(left_val >= right_val) {
                    // "[src/main:2:5]: Test failed: a * 2 < b * 5"
                    #[cfg(feature = "line-info")]
                    let message = ::std::concat!(
                        '[', ::std::file!(), ':', ::std::line!(), ':', ::std::column!(), "]: Test failed: ",
                        ::std::stringify!($left), " < ", ::std::stringify!($right)
                    );

                    // "Test failed: a * 2 < b * 5"
                    #[cfg(not(feature = "line-info"))]
                    let message = ::std::concat!(
                        "Test failed: ", ::std::stringify!($left), " < ", ::std::stringify!($right)
                    );

                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down.
                    ::std::result::Result::Err($crate::TestFailure::test_failed_one_ident(message, ::std::stringify!($right), &*right_val, ::std::option::Option::Some(::std::format_args!($($arg)+))))
                } else {
                    ::std::result::Result::Ok(())
                }
            }
        }
    }};
    ($left:expr, $right:literal, $($arg:tt)+) => {{
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(left_val >= right_val) {
                    // "[src/main:2:5]: Test failed: a * 2 < b * 5"
                    #[cfg(feature = "line-info")]
                    let message = ::std::concat!(
                        '[', ::std::file!(), ':', ::std::line!(), ':', ::std::column!(), "]: Test failed: ",
                        ::std::stringify!($left), " < ", ::std::stringify!($right)
                    );

                    // "Test failed: a * 2 < b * 5"
                    #[cfg(not(feature = "line-info"))]
                    let message = ::std::concat!(
                        "Test failed: ", ::std::stringify!($left), " < ", ::std::stringify!($right)
                    );

                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down.
                    ::std::result::Result::Err($crate::TestFailure::test_failed_one_ident(message, ::std::stringify!($left), &*left_val, ::std::option::Option::Some(::std::format_args!($($arg)+))))
                } else {
                    ::std::result::Result::Ok(())
                }
            }
        }
    }};
    ($left:expr, $right:expr, $($arg:tt)+) => {{
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(left_val >= right_val) {
                    // "[src/main:2:5]: Test failed: a * 2 < b * 5"
                    #[cfg(feature = "line-info")]
                    let message = ::std::concat!(
                        '[', ::std::file!(), ':', ::std::line!(), ':', ::std::column!(), "]: Test failed: ",
                        ::std::stringify!($left), " < ", ::std::stringify!($right)
                    );

                    // "Test failed: a * 2 < b * 5"
                    #[cfg(not(feature = "line-info"))]
                    let message = ::std::concat!(
                        "Test failed: ", ::std::stringify!($left), " < ", ::std::stringify!($right)
                    );

                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down.
                    ::std::result::Result::Err($crate::TestFailure::test_failed_two_idents(message, ::std::stringify!($left), &*left_val, ::std::stringify!($right), &*right_val, ::std::option::Option::Some(::std::format_args!($($arg)+))))
                } else {
                    ::std::result::Result::Ok(())
                }
            }
        }
    }};
}

/// Tests that both tests pass.
///
/// This macro returns a [`Result`] and hints the compiler that the failure
/// case is unlikely to happen.
///
/// A custom message can be added, with [`format!`] support.
///
/// # Examples
/// ```
/// use test_eq::{test_and, test_ge, test_ne};
/// let a = 5;
/// let b = 10;
/// let c = "hello";
/// let d = "world";
/// test_and!(test_ge!(b, a), test_ne!(c, d)).expect("This is true");
/// println!("{:?}", test_and!(test_ge!(a, b), test_ne!(c, d), "format args {}", a + b))
/// // prints:
/// // One of the tests failed: format args 15
/// //    [src/main.rs:5:1]: Test failed: a < b
/// //    a: 5
/// //    b: 10
/// ```
#[macro_export]
macro_rules! test_and {
    ($left:expr, $right:expr $(,)?) => {{
        match ($left, $right) {
            (::std::result::Result::Ok(_), ::std::result::Result::Ok(_)) => ::std::result::Result::Ok(()),
            (::std::result::Result::Err(first), ::std::result::Result::Err(second)) => ::std::result::Result::Err($crate::TestFailure::two_tests_failed(first, second, ::std::option::Option::None)),
            (::std::result::Result::Err(one), _) => ::std::result::Result::Err($crate::TestFailure::one_test_failed(one, ::std::option::Option::None)),
            (_, ::std::result::Result::Err(one)) => ::std::result::Result::Err($crate::TestFailure::one_test_failed(one, ::std::option::Option::None)),
        }
    }};
    ($left:expr, $right:expr, $($arg:tt)+) => {{
        match ($left, $right) {
            (::std::result::Result::Ok(_), ::std::result::Result::Ok(_)) => ::std::result::Result::Ok(()),
            (::std::result::Result::Err(first), ::std::result::Result::Err(second)) => ::std::result::Result::Err($crate::TestFailure::two_tests_failed(first, second, ::std::option::Option::Some(::std::format_args!($($arg)+)))),
            (::std::result::Result::Err(one), _) => ::std::result::Result::Err($crate::TestFailure::one_test_failed(one, ::std::option::Option::Some(::std::format_args!($($arg)+)))),
            (_, ::std::result::Result::Err(one)) => ::std::result::Result::Err($crate::TestFailure::one_test_failed(one, ::std::option::Option::Some(::std::format_args!($($arg)+)))),
        }
    }};
}

/// Tests that at least one test passes.
///
/// This macro returns a [`Result`] and hints the compiler that the failure
/// case is unlikely to happen.
///
/// A custom message can be added, with [`format!`] support.
///
/// # Examples
/// ```
/// use test_eq::{test_or, test_ge, test_eq};
/// let a = 5;
/// let b = 10;
/// let c = "hello";
/// let d = "world";
/// test_or!(test_ge!(b, a), test_eq!(c, d)).expect("This is true");
/// println!("{:?}", test_or!(test_ge!(a, b), test_eq!(c, d), "format args {}", a + b))
/// // prints:
/// // Both tests failed: format args 15
/// // 1: [src/main.rs:5:1]: Test failed: a < b
/// //    a: 5
/// //    b: 10
/// // 2: [src/main.rs:5:1]: Test failed: c != d
/// //    c: "hello"
/// //    d: "world"
/// ```
#[macro_export]
macro_rules! test_or {
    ($left:expr, $right:expr $(,)?) => {{
        match ($left, $right) {
            (::std::result::Result::Err(first), ::std::result::Result::Err(second)) => ::std::result::Result::Err($crate::TestFailure::two_tests_failed(first, second, ::std::option::Option::None)),
            _ => ::std::result::Result::Ok(()),
        }
    }};
    ($left:expr, $right:expr, $($arg:tt)+) => {{
        match ($left, $right) {
            (::std::result::Result::Err(first), ::std::result::Result::Err(second)) => ::std::result::Result::Err($crate::TestFailure::two_tests_failed(first, second, ::std::option::Option::Some(::std::format_args!($($arg)+)))),
            _ => ::std::result::Result::Ok(()),
        }
    }};
}
