# FAQ

## Why no dependencies?

Because kernels often need absolute control over code size, initialization order, and safety boundaries.

## Why no global logger?

Global mutable state is tricky in early boot and concurrent contexts.

This crate prefers explicit ownership (`&mut Logger`).

## Why overwrite-on-full ring buffer?

For logging/diagnostics, keeping the most recent data is typically more valuable than failing writes.
