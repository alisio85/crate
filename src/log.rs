use core::fmt;
use core::fmt::Write;

use crate::buffer::RingBuffer;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub enum Level {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

/// A minimal sink for kernel/OS logging.
///
/// The sink decides where bytes go (serial, VGA, hypervisor console, ring buffer, etc.).
pub trait LogSink {
    fn write_str(&mut self, s: &str);
    fn flush(&mut self) {}
}

pub struct RingLog<const N: usize> {
    buf: RingBuffer<N>,
}

impl<const N: usize> Default for RingLog<N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const N: usize> RingLog<N> {
    pub const fn new() -> Self {
        Self {
            buf: RingBuffer::new(),
        }
    }

    pub fn clear(&mut self) {
        self.buf.clear();
    }

    pub fn as_slices(&self) -> (&[u8], &[u8]) {
        self.buf.as_slices()
    }

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
    pub fn new(sink: &'a mut S, level: Level) -> Self {
        Self { sink, level }
    }

    pub fn level(&self) -> Level {
        self.level
    }

    pub fn sink_mut(&mut self) -> &mut S {
        self.sink
    }

    pub fn enabled(&self, level: Level) -> bool {
        level <= self.level
    }

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
macro_rules! kerror {
    ($logger:expr, $($arg:tt)*) => {{
        $logger.log($crate::log::Level::Error, core::format_args!($($arg)*));
    }};
}

#[macro_export]
macro_rules! kwarn {
    ($logger:expr, $($arg:tt)*) => {{
        $logger.log($crate::log::Level::Warn, core::format_args!($($arg)*));
    }};
}

#[macro_export]
macro_rules! kinfo {
    ($logger:expr, $($arg:tt)*) => {{
        $logger.log($crate::log::Level::Info, core::format_args!($($arg)*));
    }};
}

#[macro_export]
macro_rules! kdebug {
    ($logger:expr, $($arg:tt)*) => {{
        $logger.log($crate::log::Level::Debug, core::format_args!($($arg)*));
    }};
}

#[macro_export]
macro_rules! ktrace {
    ($logger:expr, $($arg:tt)*) => {{
        $logger.log($crate::log::Level::Trace, core::format_args!($($arg)*));
    }};
}
