/// Assertion helpers intended for OS/kernel contexts.
///
/// By default, assertions are active in debug builds and compiled out in release builds.
/// Enable feature `release_assertions` to keep them in release builds as well.
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
