# Quickstart

## 1. Add the crate

```toml
[dependencies]
os_dev_toolkit = "0.1"
```

## 2. Use `no_std`

In your kernel crate:

```rust
#![no_std]
```

`os_dev_toolkit` is `no_std`-first.

## 3. Provide a log sink

Implement `LogSink` to route output to your environment.

```rust
use os_dev_toolkit::log::{Level, LogSink, Logger};

struct MySink;
impl LogSink for MySink {
    fn write_str(&mut self, s: &str) {
        let _ = s;
    }
}

fn early_init() {
    let mut sink = MySink;
    let mut logger = Logger::new(&mut sink, Level::Info);
    os_dev_toolkit::kinfo!(logger, "booting...");
}
```

## 4. Use fixed buffers for diagnostics

```rust
use os_dev_toolkit::buffer::FixedStr;
use core::fmt::Write;

fn make_message() -> FixedStr<64> {
    let mut s: FixedStr<64> = FixedStr::new();
    let _ = write!(&mut s, "cpu={}", 0);
    s
}
```
