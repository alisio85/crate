//! Fixed-capacity buffers for allocation-free environments.
//!
//! The types in this module are designed for early boot and kernel contexts where dynamic
//! allocation is unavailable or undesirable.
//!
//! - [`crate::buffer::RingBuffer`] is a byte FIFO with overwrite-on-full semantics.
//! - [`crate::buffer::FixedStr`] is a small fixed-capacity string builder that implements [`core::fmt::Write`].

use core::fmt;

/// A fixed-capacity byte ring buffer.
///
/// ## Semantics
///
/// - Pushing into a full buffer will **overwrite** the oldest byte.
/// - Popping removes bytes in FIFO order.
/// - `as_slices()` returns up to two slices representing the logical content in FIFO order.
///
/// ## `N == 0`
///
/// For convenience in generic code, `N` is allowed to be zero. In that case:
///
/// - `push` becomes a no-op.
/// - `pop` always returns `None`.
/// - `as_slices()` returns `(&[], &[])`.
pub struct RingBuffer<const N: usize> {
    buf: [u8; N],
    head: usize,
    len: usize,
}

impl<const N: usize> Default for RingBuffer<N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const N: usize> RingBuffer<N> {
    /// Creates an empty ring buffer.
    pub const fn new() -> Self {
        Self {
            buf: [0u8; N],
            head: 0,
            len: 0,
        }
    }

    /// Returns the fixed capacity (`N`).
    pub const fn capacity(&self) -> usize {
        N
    }

    /// Returns the number of stored bytes.
    pub const fn len(&self) -> usize {
        self.len
    }

    /// Returns `true` if the buffer is empty.
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns `true` if the buffer is full.
    pub const fn is_full(&self) -> bool {
        self.len == N
    }

    /// Pushes a byte into the buffer.
    ///
    /// If the buffer is full, the oldest byte is overwritten.
    pub fn push(&mut self, byte: u8) {
        if N == 0 {
            return;
        }

        let idx = (self.head + self.len) % N;
        self.buf[idx] = byte;
        if self.len < N {
            self.len += 1;
        } else {
            self.head = (self.head + 1) % N;
        }
    }

    /// Pops the oldest byte from the buffer.
    pub fn pop(&mut self) -> Option<u8> {
        if self.len == 0 || N == 0 {
            return None;
        }

        let b = self.buf[self.head];
        self.head = (self.head + 1) % N;
        self.len -= 1;
        Some(b)
    }

    /// Clears the buffer without modifying the underlying bytes.
    pub fn clear(&mut self) {
        self.head = 0;
        self.len = 0;
    }

    /// Returns the current content as one or two slices.
    ///
    /// The returned slices must be read in order (`slice0` then `slice1`) to obtain the logical
    /// FIFO content.
    pub fn as_slices(&self) -> (&[u8], &[u8]) {
        if self.len == 0 || N == 0 {
            return (&[], &[]);
        }

        let end = (self.head + self.len) % N;
        if self.head < end {
            (&self.buf[self.head..end], &[])
        } else {
            (&self.buf[self.head..N], &self.buf[0..end])
        }
    }
}

/// A fixed-capacity UTF-8 string builder.
///
/// This type is useful for building diagnostic messages without allocation.
///
/// - Appends are bounded by `N`.
/// - On overflow, [`try_push_str`](FixedStr::try_push_str) returns [`FixedStrError`].
/// - [`as_str`](FixedStr::as_str) returns the current content.
///
/// ## UTF-8 invariant
///
/// `FixedStr` stores raw bytes internally and uses `from_utf8_unchecked` in `as_str()`.
/// This is safe because the only mutation API appends bytes from a `&str`, preserving UTF-8.
pub struct FixedStr<const N: usize> {
    buf: [u8; N],
    len: usize,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
/// Error returned when attempting to append beyond capacity.
pub struct FixedStrError;

impl<const N: usize> Default for FixedStr<N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const N: usize> FixedStr<N> {
    /// Creates an empty fixed string.
    pub const fn new() -> Self {
        Self {
            buf: [0u8; N],
            len: 0,
        }
    }

    /// Returns the fixed capacity (`N`).
    pub const fn capacity(&self) -> usize {
        N
    }

    /// Returns the current length in bytes.
    pub const fn len(&self) -> usize {
        self.len
    }

    /// Returns `true` if empty.
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Clears the string.
    pub fn clear(&mut self) {
        self.len = 0;
    }

    /// Returns the current content as `&str`.
    pub fn as_str(&self) -> &str {
        // Safe because we only ever append valid UTF-8 from &str.
        unsafe { core::str::from_utf8_unchecked(&self.buf[..self.len]) }
    }

    /// Attempts to append `s`.
    ///
    /// Returns [`FixedStrError`] if the resulting length would exceed `N`.
    pub fn try_push_str(&mut self, s: &str) -> Result<(), FixedStrError> {
        let bytes = s.as_bytes();
        if self.len + bytes.len() > N {
            return Err(FixedStrError);
        }
        self.buf[self.len..self.len + bytes.len()].copy_from_slice(bytes);
        self.len += bytes.len();
        Ok(())
    }
}

impl<const N: usize> fmt::Write for FixedStr<N> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.try_push_str(s).map_err(|_| fmt::Error)
    }
}
