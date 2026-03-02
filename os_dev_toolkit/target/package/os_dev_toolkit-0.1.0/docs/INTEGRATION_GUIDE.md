# Integration Guide

This guide shows recommended integration patterns for kernels.

## Logger ownership

Preferred pattern: create a `Logger` early and pass `&mut Logger` to subsystems.

- Avoid global mutable state.
- Keep the sink implementation in your kernel crate.

## Panic routing

If you want panics to end up in your sink:

1. Implement a sink that is safe to use in panic paths.
2. Use `os_dev_toolkit::panic::panic_to_sink` in your panic handler.

## Assertions strategy

- In debug builds, `kassert!` checks are enabled.
- In release builds, enable feature `release_assertions` if you want invariant checks to remain active.

## Status codes

Use `Status` for cross-module results where you want an OS-style error contract.

- Keep conversions at module boundaries.
- Avoid mixing rich errors and status codes deep in the call graph.
