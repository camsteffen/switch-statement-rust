# Switch Statement For Rust

*A simple macro to emulate a `switch` statement in Rust.*

[![crates.io](https://img.shields.io/crates/v/switch_statement.svg)](https://crates.io/crates/switch_statement)
[![Docs](https://docs.rs/switch_statement/badge.svg)](https://docs.rs/switch_statement/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

## Description

The `switch!` macro looks similar to `match`. But instead of pattern matching,
each left-hand-side expression is interpreted as an expression instead of a
pattern. One use case for this is to match against constants joined with
bitwise OR. The output of the macro is a `match` with `if` guards.

## Example

```rust
const A: u32 = 1 << 0;
const B: u32 = 1 << 1;
const C: u32 = 1 << 2;

fn flag_string(input: u32) -> &'static str {
    switch! { input;
        A => "A",
        // bitwise OR
        A | B => "A and B",
        A | B | C => "A and B and C",
        B | C => "B and C",
        _ => "other"
    }
}
```

The above code expands to:

```rust
const A: u32 = 1 << 0;
const B: u32 = 1 << 1;
const C: u32 = 1 << 2;

fn flag_string(input: u32) -> &'static str {
    match input {
        v if v == A => "A",
        v if v == A | B => "A and B",
        v if v == A | B | C => "A and B and C",
        v if v == B | C => "B and C",
        _ => "other"
    }
}
```
