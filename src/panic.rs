//! Panic and halt helpers.
//!
//! In many kernels you want a consistent strategy for panic handling:
//!
//! - Format panic information to a device (serial/VGA/etc.).
//! - Halt the CPU (spin) to stop further damage.
//!
//! This module provides the building blocks to implement that pattern.

use core::fmt;
use core::fmt::Write;

use crate::log::LogSink;

/// Busy-wait forever.
///
/// This uses [`core::hint::spin_loop`] and never returns.
pub fn halt() -> ! {
    loop {
        core::hint::spin_loop();
    }
}

/// Formats a [`core::panic::PanicInfo`] to `sink` and then halts.
///
/// This function is intended to be called from a `#[panic_handler]` implementation in a kernel.
/// It is infallible: formatting failures are ignored because the sink is considered infallible.
pub fn panic_to_sink(info: &core::panic::PanicInfo<'_>, sink: &mut dyn LogSink) -> ! {
    struct Adapter<'a>(&'a mut dyn LogSink);
    impl fmt::Write for Adapter<'_> {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.0.write_str(s);
            Ok(())
        }
    }

    let mut a = Adapter(sink);
    let _ = writeln!(a, "PANIC: {info}");
    halt()
}
