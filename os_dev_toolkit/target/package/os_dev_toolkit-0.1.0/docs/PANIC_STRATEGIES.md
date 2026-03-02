# Panic Strategies

## Goals

- Preserve a readable panic message.
- Avoid allocations.
- End in a deterministic halt.

## `panic_to_sink`

`panic_to_sink(info, sink)` formats `PanicInfo` into your sink and then halts via a spin loop.

## Recommended kernel panic handler pattern

- Ensure the sink is in a known-good state.
- Avoid re-entrancy.
- Consider disabling interrupts before writing.

## `panic = abort`

If you set `panic = "abort"` you may lose unwinding behavior; for kernels this is typically fine.

This crate remains compatible either way because it relies only on formatting the panic info.
