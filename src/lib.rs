//! # os_dev_toolkit
//!
//! Dependency-free, `no_std`-first building blocks intended to improve the *developer experience*
//! while writing operating systems and kernels in Rust.
//!
//! ## Design goals
//!
//! - **`no_std` by default**: usable in kernels without a standard library.
//! - **No external dependencies**: keeps builds deterministic and simplifies bring-up.
//! - **Deterministic formatting/output**: logging and diagnostics avoid allocation by default.
//! - **Small, composable primitives**: you decide integration details (serial/VGA/etc.).
//!
//! ## Features
//!
//! - `alloc`: reserved for future optional allocation-backed utilities.
//! - `release_assertions`: keeps assertion macros enabled in release builds.
//!
//! ## High-level usage
//!
//! Most consumers will start with the [`crate::log`] module:
//!
//! ```rust
//! use os_dev_toolkit::log::{Level, LogSink, Logger};
//!
//! struct MySink;
//! impl LogSink for MySink {
//!     fn write_str(&mut self, s: &str) {
//!         let _ = s;
//!         // forward to serial/vga/etc.
//!     }
//! }
//!
//! fn demo() {
//!     let mut sink = MySink;
//!     let mut logger = Logger::new(&mut sink, Level::Info);
//!     os_dev_toolkit::kinfo!(logger, "hello from the kernel side: {}", 123);
//! }
//! ```
//!
//! ## Safety and concurrency
//!
//! This crate provides synchronization primitives such as [`sync::SpinLock`] and [`sync::Once`].
//! These are intentionally minimal and do **not** attempt to be fair or preemption-safe; you must
//! apply the right interrupt/preemption masking strategy for your kernel.
#![no_std]
#![deny(warnings)]

/// Assertion macros suitable for kernel/OS environments.
pub mod assert;
/// Fixed-capacity data structures (`RingBuffer`, `FixedStr`) intended for allocation-free code.
pub mod buffer;
/// Deterministic formatting helpers such as hexdumps and byte-size formatters.
pub mod fmt;
/// Minimal logging traits and a small `Logger` with level filtering.
pub mod log;
/// Panic helpers for routing panic output to a sink and halting.
pub mod panic;
/// OS-style status codes and small conversion helpers.
pub mod status;
/// Simple synchronization primitives (`SpinLock`, `Once`) for `no_std` environments.
pub mod sync;

/// Compile-time configuration helpers.
pub mod config;
