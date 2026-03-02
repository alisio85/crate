use core::fmt;
use core::fmt::Write;

use crate::log::LogSink;

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

pub struct Addr(pub usize);

impl fmt::LowerHex for Addr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{:x}", self.0)
    }
}
