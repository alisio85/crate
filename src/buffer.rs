use core::fmt;

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
    pub const fn new() -> Self {
        Self {
            buf: [0u8; N],
            head: 0,
            len: 0,
        }
    }

    pub const fn capacity(&self) -> usize {
        N
    }

    pub const fn len(&self) -> usize {
        self.len
    }

    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub const fn is_full(&self) -> bool {
        self.len == N
    }

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

    pub fn pop(&mut self) -> Option<u8> {
        if self.len == 0 || N == 0 {
            return None;
        }

        let b = self.buf[self.head];
        self.head = (self.head + 1) % N;
        self.len -= 1;
        Some(b)
    }

    pub fn clear(&mut self) {
        self.head = 0;
        self.len = 0;
    }

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

pub struct FixedStr<const N: usize> {
    buf: [u8; N],
    len: usize,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct FixedStrError;

impl<const N: usize> Default for FixedStr<N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const N: usize> FixedStr<N> {
    pub const fn new() -> Self {
        Self {
            buf: [0u8; N],
            len: 0,
        }
    }

    pub const fn capacity(&self) -> usize {
        N
    }

    pub const fn len(&self) -> usize {
        self.len
    }

    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn clear(&mut self) {
        self.len = 0;
    }

    pub fn as_str(&self) -> &str {
        // Safe because we only ever append valid UTF-8 from &str.
        unsafe { core::str::from_utf8_unchecked(&self.buf[..self.len]) }
    }

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
