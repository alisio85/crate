//! Formatting helpers for diagnostics in `no_std` environments.
//!
//! The goal of this module is to provide deterministic, allocation-free formatting helpers that
//! are frequently useful during OS development:
//!
//! - [`crate::fmt::HexSlice`] for stable, compact `Debug` printing of byte slices.
//! - [`crate::fmt::hexdump_to_sink`] for a classic hex dump layout (offset/hex/ascii).
//! - [`crate::fmt::ByteFmt`] for human-friendly binary unit formatting.
//! - [`crate::fmt::Addr`] for consistent pointer/address formatting.

use core::fmt;
use core::fmt::Write;

use crate::log::LogSink;

/// Wrapper type providing a stable [`Debug`](core::fmt::Debug) representation for `&[u8]`.
///
/// This is intentionally compact and deterministic:
///
/// ```text
/// [00 01 02 ff]
/// ```
///
/// This is handy in kernel logs where `{:?}` output should be predictable.
pub struct HexSlice<'a>(pub &'a [u8]);

impl fmt::Debug for HexSlice<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for (i, b) in self.0.iter().enumerate() {
            if i != 0 {
                write!(f, " ")?;
            }
            write!(f, "{:02x}", b)?;
        }
        write!(f, "]")
    }
}

/// Writes a classic hex dump to a [`LogSink`].
///
/// Each line is formatted as:
///
/// ```text
/// 0x00000000: 01 02 03 ... |....|
/// ```
///
/// - `bytes`: input data to render.
/// - `sink`: output destination.
/// - `columns`: number of bytes per line. If `0`, defaults to `16`.
///
/// This function is best-effort: it ignores formatting errors because the underlying sink is
/// infallible by contract.
pub fn hexdump_to_sink(bytes: &[u8], sink: &mut dyn LogSink, columns: usize) {
    let cols = if columns == 0 { 16 } else { columns };

    let mut offset = 0usize;
    while offset < bytes.len() {
        let end = core::cmp::min(offset + cols, bytes.len());
        let chunk = &bytes[offset..end];

        sink.write_str("0x");
        {
            struct Adapter<'a>(&'a mut dyn LogSink);
            impl fmt::Write for Adapter<'_> {
                fn write_str(&mut self, s: &str) -> fmt::Result {
                    self.0.write_str(s);
                    Ok(())
                }
            }
            let mut a = Adapter(sink);
            let _ = write!(a, "{:08x}: ", offset);
            for b in chunk {
                let _ = write!(a, "{:02x} ", b);
            }

            if end - offset < cols {
                for _ in 0..(cols - (end - offset)) {
                    let _ = write!(a, "   ");
                }
            }

            let _ = write!(a, "|");
            for &b in chunk {
                let c = if (0x20..=0x7e).contains(&b) {
                    b as char
                } else {
                    '.'
                };
                let _ = write!(a, "{c}");
            }
            if end - offset < cols {
                for _ in 0..(cols - (end - offset)) {
                    let _ = write!(a, " ");
                }
            }
            let _ = write!(a, "|");
        }
        sink.write_str("\n");
        offset = end;
    }
}

/// Formats a byte count using binary units (KiB, MiB, GiB).
///
/// The output keeps three decimals for larger units and rounds down (truncation), which is often
/// preferable in diagnostics.
///
/// Examples:
///
/// - `ByteFmt(999)` -> `"999 B"`
/// - `ByteFmt(1024)` -> `"1.000 KiB"`
/// - `ByteFmt(1024*1024)` -> `"1.000 MiB"`
pub struct ByteFmt(pub u64);

impl fmt::Display for ByteFmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KIB: u64 = 1024;
        const MIB: u64 = 1024 * 1024;
        const GIB: u64 = 1024 * 1024 * 1024;

        let b = self.0;
        if b >= GIB {
            write!(f, "{}.{:03} GiB", b / GIB, (b % GIB) * 1000 / GIB)
        } else if b >= MIB {
            write!(f, "{}.{:03} MiB", b / MIB, (b % MIB) * 1000 / MIB)
        } else if b >= KIB {
            write!(f, "{}.{:03} KiB", b / KIB, (b % KIB) * 1000 / KIB)
        } else {
            write!(f, "{b} B")
        }
    }
}

/// Formats an address as lower-hex with `0x` prefix.
///
/// This is mainly a readability wrapper for kernel logs.
pub struct Addr(pub usize);

impl fmt::LowerHex for Addr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{:x}", self.0)
    }
}
