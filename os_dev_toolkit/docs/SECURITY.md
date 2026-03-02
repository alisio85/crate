# Security

## Reporting

If you find a security issue, open a private report via GitHub security advisories (recommended) or open an issue with minimal details.

## Scope

This crate is low-level and intended for OS development. Typical issues would be:

- memory safety bugs
- data corruption in ring buffers
- unexpected panics in `no_std` usage
