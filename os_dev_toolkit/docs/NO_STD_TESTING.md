# no_std Testing

Even if your kernel is `no_std`, you can test most logic on the host.

## Strategy

- Keep logic in `no_std` modules.
- Write `#[test]` in `tests/` which runs under `std`.
- Avoid architecture-specific instructions in the crate.

## What this crate tests

- ring buffer wraparound and overwrite semantics
- fixed-capacity formatting boundaries
- deterministic hexdump output shape
