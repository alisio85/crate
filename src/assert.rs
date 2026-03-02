//! Assertion macros intended for OS/kernel contexts.
//!
//! The standard `assert!` macros are fine in many situations, but OS development often benefits
//! from:
//!
//! - a distinct naming convention (`kassert!`) to avoid confusion with userspace code;
//! - an explicit feature to keep assertions in release builds.
//!
//! ## Build behavior
//!
//! - In `debug_assertions` builds, the macros behave like standard assertions and panic on failure.
//! - In release builds, the macros are compiled out by default.
//! - Enabling the `release_assertions` feature keeps them active in release builds.
//!
//! When compiled out, the macros still type-check their arguments (to avoid unused warnings and
//! preserve formatting validation), but they perform no runtime checks.

/// Kernel assertion.
///
/// In enabled builds, panics if `$cond` is false.
#[cfg(any(debug_assertions, feature = "release_assertions"))]
#[macro_export]
macro_rules! kassert {
    ($cond:expr $(,)?) => {{
        if !$cond {
            panic!("kassert failed: {}", stringify!($cond));
        }
    }};
    ($cond:expr, $($arg:tt)+) => {{
        if !$cond {
            panic!($($arg)+);
        }
    }};
}

#[cfg(not(any(debug_assertions, feature = "release_assertions")))]
#[macro_export]
macro_rules! kassert {
    ($cond:expr $(,)?) => {{
        let _ = &$cond;
    }};
    ($cond:expr, $($arg:tt)+) => {{
        let _ = &$cond;
        let _ = core::format_args!($($arg)+);
    }};
}

/// Kernel equality assertion.
///
/// In enabled builds, panics if the two expressions are not equal.
#[cfg(any(debug_assertions, feature = "release_assertions"))]
#[macro_export]
macro_rules! kassert_eq {
    ($left:expr, $right:expr $(,)?) => {{
        let l = &$left;
        let r = &$right;
        if *l != *r {
            panic!("kassert_eq failed: left={:?}, right={:?}", l, r);
        }
    }};
}

#[cfg(not(any(debug_assertions, feature = "release_assertions")))]
#[macro_export]
macro_rules! kassert_eq {
    ($left:expr, $right:expr $(,)?) => {{
        let _ = &$left;
        let _ = &$right;
    }};
}

/// Kernel inequality assertion.
///
/// In enabled builds, panics if the two expressions are equal.
#[cfg(any(debug_assertions, feature = "release_assertions"))]
#[macro_export]
macro_rules! kassert_ne {
    ($left:expr, $right:expr $(,)?) => {{
        let l = &$left;
        let r = &$right;
        if *l == *r {
            panic!("kassert_ne failed: left={:?}, right={:?}", l, r);
        }
    }};
}

#[cfg(not(any(debug_assertions, feature = "release_assertions")))]
#[macro_export]
macro_rules! kassert_ne {
    ($left:expr, $right:expr $(,)?) => {{
        let _ = &$left;
        let _ = &$right;
    }};
}
