# Buffer Types

## RingBuffer

`RingBuffer<N>` is a fixed-capacity FIFO for bytes.

### Overwrite semantics

When full, pushing a new byte overwrites the oldest byte.

### Slicing

`as_slices()` returns up to two slices representing the logical FIFO order.

## FixedStr

`FixedStr<N>` is a fixed-capacity UTF-8 string builder.

- It implements `core::fmt::Write`.
- `as_str()` is safe because only valid `&str` is appended.
