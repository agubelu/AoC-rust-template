# Advent of Code project template
A Rust template for Advent of Code that I made to easily run any day or combination of days and measure the execution time.

Each day has a `solve()` function that returns a pair of `Solution`. The type `Solution` is an enum that can contain any integer or a string.

You can create a `Solution` by specifying its type, for example `Solution::U32(value)`, or by using the From trait which is implemented for all supported types, for example, `Solution::from(value)`.

To run: `cargo run --release [days...]`