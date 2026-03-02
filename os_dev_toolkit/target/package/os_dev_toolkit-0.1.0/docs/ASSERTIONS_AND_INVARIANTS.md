# Assertions and Invariants

## Why not always-on assertions?

In kernels, assertions can be expensive or unsafe in certain contexts.

## Macros

- `kassert!`
- `kassert_eq!`
- `kassert_ne!`

## Release builds

By default, assertions are disabled in release builds.

Enable feature `release_assertions` to keep them enabled.

## Recommended usage

- Use assertions for invariants that indicate a programmer error.
- Use `Status` for runtime/expected failures.
