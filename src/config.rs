//! Small compile-time configuration helpers.
//!
//! This module is intended for lightweight invariants that you may want to enforce even in
//! `no_std` contexts.

/// Enforces a required condition.
///
/// This is primarily intended for configuration/invariant checks.
///
/// - If `condition` is `true`, this function does nothing.
/// - If `condition` is `false`, it panics.
///
/// The `_message` parameter is kept for API ergonomics; it can be used by higher-level wrappers
/// or future improvements.
pub const fn require(condition: bool, _message: &str) {
    if !condition {
        panic!();
    }
}
