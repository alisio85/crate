# os_dev_toolkit Manual

## 1. Overview

`os_dev_toolkit` is a dependency-free, `no_std`-first crate that provides small building blocks typically needed while developing operating systems in Rust.

This crate intentionally avoids:

- boot code / bootloader integration
- paging / memory allocators
- scheduler / drivers / arch-specific primitives

Instead, it focuses on *developer experience primitives* that are useful in any kernel layout.

## 2. Integration philosophy

- You implement **where bytes go** by providing a `LogSink`.
- The crate provides **how bytes are formatted and structured**.
- Every component is:
  - deterministic
  - allocation-free by default
  - testable on the host

## 3. Modules

### 3.1 `log`

#### Core types

- `LogSink`: minimal interface.
- `Logger`: provides level filtering and formatting.
- Log macros: `kerror!`, `kwarn!`, `kinfo!`, `kdebug!`, `ktrace!`.

#### Typical kernel usage

1. Implement a sink (serial/VGA/etc.).
2. Create a `Logger` early.
3. Pass the logger (or a mutable reference to it) to subsystems.

### 3.2 `buffer`

- `RingBuffer<N>`: fixed-capacity FIFO byte buffer with overwrite-on-full semantics.
- `FixedStr<N>`: fixed-capacity string builder implementing `core::fmt::Write`.

### 3.3 `fmt`

- `HexSlice`: stable debug rendering for byte slices.
- `hexdump_to_sink`: deterministic hex dump output with configurable columns.

### 3.4 `status`

- `Status`: small OS-style status codes.
- `KResult<T>`: `Result<T, Status>` alias.

### 3.5 `assert`

- `kassert!`, `kassert_eq!`, `kassert_ne!`.
- Feature `release_assertions` keeps assertions in release builds.

### 3.6 `panic`

- `halt()`: portable spin-loop.
- `panic_to_sink()`: formats a panic into a sink and halts.

## 4. Testing strategy

Even if your kernel is `no_std`, these components are designed so that their logic can be tested on the host via `cargo test`.

## 5. Stability

- Public APIs are kept small.
- Semver: breaking changes only in major releases.
