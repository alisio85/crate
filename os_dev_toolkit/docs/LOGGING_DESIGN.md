# Logging Design

## Design goals

- Minimal API surface.
- No external dependencies.
- No allocation required.
- Works in early boot.

## Sink-based architecture

You provide a `LogSink`.

- Serial sink: send bytes to UART registers.
- VGA sink: write to text buffer.
- Hypervisor sink: use debug port.

`Logger` takes a mutable reference to your sink and writes formatted strings via `core::fmt`.

## Level filtering

Levels are ordered:

`Error < Warn < Info < Debug < Trace`

A logger configured with `Info` will emit `Error`, `Warn`, and `Info`.

## In-memory ring logging (`RingLog`)

`RingLog<N>` is an in-memory sink that stores the most recent log bytes in a fixed-capacity ring buffer.

Typical usage:

1. Use `RingLog` as your `LogSink`.
2. On panic or error, retrieve the buffered bytes using `as_slices()`.
3. Forward them to a real output device (serial/VGA/etc.) if available.

The overwrite policy keeps the *most recent* bytes when the buffer is full.

## Concurrency

This crate does not impose a locking strategy.

- If your sink is shared across CPUs/interrupt contexts, wrap it with your own synchronization.
- Keep sinks small and side-effect controlled, especially in panic paths.
