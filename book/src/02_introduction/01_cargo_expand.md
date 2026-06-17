# Exploring macros with `cargo expand`

Macros generate code, but that code is invisible by default — the compiler sees it, you don't.
This can make macros feel like black boxes.

[`cargo expand`](https://github.com/dtolnay/cargo-expand) is a tool that shows you the code
a macro generates. It's invaluable for understanding what existing macros do, and later for
debugging the macros you write yourself.

## Installation

The fastest way is via [`cargo-binstall`](https://github.com/cargo-bins/cargo-binstall), which
downloads a prebuilt binary instead of compiling from source:

```bash
# Install cargo-binstall first if you don't have it already.
cargo install cargo-binstall

# Then use it to fetch a prebuilt cargo-expand.
cargo binstall cargo-expand
```

Alternatively, compile `cargo-expand` from source directly. This is slower, but needs no extra
tooling:

```bash
cargo install cargo-expand
```

## Editor integration

You don't strictly need the command-line tool: most editors backed by
[rust-analyzer](https://rust-analyzer.github.io/) can expand macros for you. In
[Zed](https://zed.dev/), place your cursor over a macro invocation and run **`editor: expand macro recursively`** from the command palette. VS Code (with the rust-analyzer extension) offers the same
under **`rust-analyzer: Expand macro recursively`**, and JetBrains IDEs have an equivalent
**Expand macro** action. These are handy for a quick look, while `cargo expand` is better when you
want the whole crate or to pipe the output elsewhere.

## Usage

Given this code:

```rust
#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}
```

Running `cargo expand` will show the full `Debug` implementation that the derive macro generates:

```rust
struct Point {
    x: f64,
    y: f64,
}

impl ::core::fmt::Debug for Point {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f, "Point", "x", &self.x, "y", &&self.y,
        )
    }
}
```

The exact output may vary between Rust versions — the compiler is free to change the
internal helpers it uses. The important points remain the same, though:

- The original struct is preserved unchanged.
- All paths are **fully qualified** (`::core::fmt::Debug` instead of `Debug`). This is a
  deliberate choice: generated code must not break if the user has a different item named `Debug`
  in scope.
- The generated code is valid Rust — you could have written it by hand.

## Tips

You can expand a specific item instead of the entire crate by passing its name:

```bash
cargo expand --package cargo-expand-exercise --lib -- Rgb
```

This is useful when a crate contains many items and you only care about one.

## Exercise

Run `cargo expand --package cargo-expand-exercise --lib` to see what the `Clone` derive generates
for a simple struct. Then write the same implementation by hand for a second struct.
