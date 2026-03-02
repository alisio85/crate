# Status Codes

`Status` provides a compact OS-style error contract.

## Guidelines

- Return `Status::Ok`/`Ok(T)` for success.
- Use `Status::InvalidArgument` for API misuse.
- Use `Status::NotSupported` for feature gaps.
- Use `Status::OutOfMemory` when allocation fails (if you map from an allocator).

## `KResult<T>`

Use `KResult<T>` at subsystem boundaries to keep signatures consistent.

## Conversions

Prefer conversions only at boundaries:

- rich errors in internal code (if you use them)
- map to `Status` at kernel ABI boundaries
