//! Minimal logging primitives for `no_std` kernels.
//!
//! This module is intentionally small:
//!
//! - You provide a [`crate::log::LogSink`] that decides *where bytes go* (serial/VGA/hypervisor console/etc.).
//! - This crate provides a [`crate::log::Logger`] that decides *how bytes are formatted* and implements
//!   level filtering.
//! - All APIs are allocation-free and usable in early boot.
//!
//! ## Typical usage
//!
//! ```rust
//! use os_dev_toolkit::log::{Level, LogSink, Logger};
//!
//! struct Sink;
//! impl LogSink for Sink {
//!     fn write_str(&mut self, s: &str) {
//!         let _ = s;
//!     }
//! }
//!
//! fn demo() {
//!     let mut sink = Sink;
//!     let mut logger = Logger::new(&mut sink, Level::Info);
//!     os_dev_toolkit::kinfo!(logger, "boot ok");
//!     os_dev_toolkit::kdebug!(logger, "this will be filtered out");
//! }
//! ```

use core::fmt;
use core::fmt::Write;

use crate::buffer::RingBuffer;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
/// Log level used by [`Logger`] for filtering.
///
/// Ordering is from most important to least important:
///
/// - [`Level::Error`] is the highest priority.
/// - [`Level::Trace`] is the lowest priority.
///
/// A [`Logger`] configured with a given level will accept any message whose level is
/// `<=` its configured level.
pub enum Level {
    /// Something is wrong and likely requires immediate attention.
    Error,
    /// Something unexpected happened, but execution may continue.
    Warn,
    /// High-level informational messages (boot progress, state transitions).
    Info,
    /// Debug-level messages (may be verbose).
    Debug,
    /// Extremely verbose tracing.
    Trace,
}

/// A minimal sink for kernel/OS logging.
///
/// The sink decides where bytes go (serial, VGA, hypervisor console, ring buffer, etc.).
///
/// ## Contract
///
/// - `write_str` is expected to be *best-effort* and should not panic.
/// - Implementations should handle being called many times with small fragments.
/// - The default [`LogSink::flush`] is a no-op; override it if your device benefits from it.
pub trait LogSink {
    /// Writes a string fragment to the output device.
    fn write_str(&mut self, s: &str);
    /// Flushes buffered output if applicable.
    fn flush(&mut self) {}
}

/// A fixed-capacity in-memory log sink.
///
/// This is useful when you don't have a device early during boot, or when you want to keep the
/// last `N` bytes of logs for later inspection.
///
/// Internally it uses [`RingBuffer`], so it has **overwrite-on-full** semantics.
pub struct RingLog<const N: usize> {
    buf: RingBuffer<N>,
}

impl<const N: usize> Default for RingLog<N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const N: usize> RingLog<N> {
    /// Creates an empty ring log.
    ///
    /// If `N == 0`, all writes become no-ops and `as_slices()` returns empty slices.
    pub const fn new() -> Self {
        Self {
            buf: RingBuffer::new(),
        }
    }

    /// Clears the stored bytes.
    pub fn clear(&mut self) {
        self.buf.clear();
    }

    /// Returns the current content as two slices.
    ///
    /// Because this is a ring buffer, the data may wrap. The returned slices represent the
    /// content in logical FIFO order (`slice0` then `slice1`).
    pub fn as_slices(&self) -> (&[u8], &[u8]) {
        self.buf.as_slices()
    }

    /// Pushes a single byte into the ring.
    ///
    /// This can be used by device drivers that already stream bytes.
    pub fn push_byte(&mut self, b: u8) {
        self.buf.push(b);
    }
}

impl<const N: usize> LogSink for RingLog<N> {
    fn write_str(&mut self, s: &str) {
        for &b in s.as_bytes() {
            self.buf.push(b);
        }
    }
}

/// A tiny logger with level filtering.
///
/// `Logger` formats each line as:
///
/// ```text
/// [Level] message\n
/// ```
///
/// It is intentionally synchronous and allocation-free.
pub struct Logger<'a, S: LogSink> {
    sink: &'a mut S,
    level: Level,
}

struct SinkWriter<'a, S: LogSink>(&'a mut S);

impl<S: LogSink> fmt::Write for SinkWriter<'_, S> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.0.write_str(s);
        Ok(())
    }
}

impl<'a, S: LogSink> Logger<'a, S> {
    /// Creates a new logger writing to `sink` and filtering at `level`.
    pub fn new(sink: &'a mut S, level: Level) -> Self {
        Self { sink, level }
    }

    /// Returns the current configured level.
    pub fn level(&self) -> Level {
        self.level
    }

    /// Returns a mutable reference to the underlying sink.
    ///
    /// This is useful when you need to flush or reconfigure the device.
    pub fn sink_mut(&mut self) -> &mut S {
        self.sink
    }

    /// Returns `true` if messages of `level` would be emitted.
    pub fn enabled(&self, level: Level) -> bool {
        level <= self.level
    }

    /// Writes a log line with the given level and pre-formatted arguments.
    ///
    /// This is the core routine used by the `k*` macros.
    pub fn log(&mut self, level: Level, args: fmt::Arguments<'_>) {
        if !self.enabled(level) {
            return;
        }

        let mut w = SinkWriter(self.sink);
        let _ = write!(w, "[{:?}] ", level);
        let _ = w.write_fmt(args);
        let _ = w.write_str("\n");
    }
}

#[macro_export]
/// Emits an error-level log line.
macro_rules! kerror {
    ($logger:expr, $($arg:tt)*) => {{
        $logger.log($crate::log::Level::Error, core::format_args!($($arg)*));
    }};
}

#[macro_export]
/// Emits a warning-level log line.
macro_rules! kwarn {
    ($logger:expr, $($arg:tt)*) => {{
        $logger.log($crate::log::Level::Warn, core::format_args!($($arg)*));
    }};
}

#[macro_export]
/// Emits an info-level log line.
macro_rules! kinfo {
    ($logger:expr, $($arg:tt)*) => {{
        $logger.log($crate::log::Level::Info, core::format_args!($($arg)*));
    }};
}

#[macro_export]
/// Emits a debug-level log line.
macro_rules! kdebug {
    ($logger:expr, $($arg:tt)*) => {{
        $logger.log($crate::log::Level::Debug, core::format_args!($($arg)*));
    }};
}

#[macro_export]
/// Emits a trace-level log line.
macro_rules! ktrace {
    ($logger:expr, $($arg:tt)*) => {{
        $logger.log($crate::log::Level::Trace, core::format_args!($($arg)*));
    }};
}
