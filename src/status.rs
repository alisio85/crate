//! Small OS-style status codes.
//!
//! Many kernels prefer returning small integer status codes rather than rich error types.
//! This module provides:
//!
//! - [`crate::status::Status`]: a compact `repr(i32)` enum.
//! - [`crate::status::KResult`]: a `Result<T, Status>` alias.
//! - Helper traits to convert common types into status codes.

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
#[repr(i32)]
/// Compact status codes commonly used in OS code.
///
/// The numeric values are chosen to be stable and FFI-friendly.
pub enum Status {
    /// Success.
    Ok = 0,
    /// An argument did not satisfy preconditions.
    InvalidArgument = 1,
    /// Allocation failed or memory is exhausted.
    OutOfMemory = 2,
    /// Operation is not supported on this platform/configuration.
    NotSupported = 3,
    /// Requested item was not found.
    NotFound = 4,
    /// I/O failed (device error, bus error, etc.).
    IoError = 5,
    /// Resource is currently busy.
    Busy = 6,
    /// Operation timed out.
    Timeout = 7,
    /// Catch-all unknown error.
    Unknown = 255,
}

/// Kernel-style result type.
pub type KResult<T> = core::result::Result<T, Status>;

impl Status {
    /// Returns `true` if this status is [`Status::Ok`].
    pub const fn is_ok(self) -> bool {
        matches!(self, Status::Ok)
    }
}

/// Converts a value into [`Status`] using a caller-provided error code.
///
/// This is especially useful for converting boolean checks into a status return.
pub trait IntoStatus {
    /// Returns [`Status::Ok`] if `self` represents success, otherwise returns `err`.
    fn into_status(self, err: Status) -> Status;
}

impl IntoStatus for bool {
    fn into_status(self, err: Status) -> Status {
        if self { Status::Ok } else { err }
    }
}

/// Converts an [`Option`] into a [`KResult`].
///
/// This is similar to `Option::ok_or`, but takes a [`Status`] instead of a custom error type.
pub trait OptionIntoStatus<T> {
    /// Returns `Ok(v)` if `Some(v)`, otherwise returns `Err(err)`.
    fn ok_or_status(self, err: Status) -> KResult<T>;
}

impl<T> OptionIntoStatus<T> for Option<T> {
    fn ok_or_status(self, err: Status) -> KResult<T> {
        match self {
            Some(v) => Ok(v),
            None => Err(err),
        }
    }
}
