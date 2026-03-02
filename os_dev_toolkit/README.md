# os_dev_toolkit

Dependency-free developer tooling primitives for Rust OS development: minimal logging, diagnostics formatting, fixed-capacity buffers, status codes, and assertion helpers.

## License

MIT.

**Attribution**: Created by an AI assistant (Cascade) based on an idea by **alisio85**.

## Goals

- `no_std`-first
- zero external dependencies
- deterministic behavior (no allocation by default)
- strict CI: no warnings, clippy clean

## Quick example (logging)

```rust
use os_dev_toolkit::log::{Level, LogSink, Logger};

struct MySink;
impl LogSink for MySink {
    fn write_str(&mut self, s: &str) {
        let _ = s;
        // forward to serial/vga/etc
    }
}

fn demo() {
    let mut sink = MySink;
    let mut logger = Logger::new(&mut sink, Level::Info);
    os_dev_toolkit::kinfo!(logger, "hello from the kernel side: {}", 123);
}
```

## Documentation

See `docs/MANUAL.md` and the other documents in `docs/`.
