# Advent of Code project template
A Rust template for Advent of Code that I made to easily run any day or combination of days and measure the execution time.

Each day has a `solve()` function that returns a pair of `Solution`. The type `Solution` is an enum that can contain any integer or a string.

You can create a `Solution` by specifying its type, for example `Solution::U32(value)`, or by using the From trait which is implemented for all supported types, for example, `Solution::from(value)`.

To run: `cargo run --release [days...]`

## Utilities

The project has a few extra utilities and goodies available in the `etc` module:

- `DOUBLE_NEWLINE`: A platform-specific constant to allow splitting input strings on double newlines.
- `Grid`: A two-dimensional matrix with some utilities to check for bounds, provide a default for OOB access, find and enumerate elements, etc.
- `Point`: A representation of a discrete point in 2D space. It includes utilities for distance calculation, moving in the cardinal directions, listing neighboring points, and so on. A `Point` can also be used to index a `Grid`.