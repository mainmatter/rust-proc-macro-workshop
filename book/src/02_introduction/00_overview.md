# Introduction

Rust has three kinds of procedural macros. You've almost certainly used all three,
even if you didn't realize it at the time.

## Derive macros

Derive macros generate **additional** code for a struct or enum.
They don't modify the original item — they _extend_ it.

```rust
#[derive(Debug, Clone)]
struct Point {
    x: f64,
    y: f64,
}
```

[`Debug`](https://doc.rust-lang.org/std/fmt/trait.Debug.html) and [`Clone`](https://doc.rust-lang.org/std/clone/trait.Clone.html) are derive macros. They look at the fields of `Point` and generate
implementations of the `Debug` and `Clone` traits, respectively.

> `Debug` and `Clone` are actually built into the compiler, not implemented as procedural macros —
> but they use the same `#[derive(...)]` syntax. Third-party derive macros like
> [`serde::Serialize`](https://docs.rs/serde/latest/serde/derive.Serialize.html) or
> [`thiserror::Error`](https://docs.rs/thiserror/latest/thiserror/derive.Error.html) _are_
> procedural macros, and they work exactly the same way from a user's perspective.

Derive macros are the most common kind of procedural macro. They are also the simplest to write,
which is why we'll start with them in this workshop.

## Function-like macros

Function-like macros look like function calls, but they operate on tokens rather than values.
The `!` suffix is a syntactic marker that distinguishes macro invocations from regular
function calls.

```rust
let query = sqlx::query!("SELECT * FROM users WHERE id = $1", user_id);
```

[`sqlx::query!`](https://docs.rs/sqlx/latest/sqlx/macro.query.html) is a procedural
function-like macro. It parses a SQL string at compile time, verifies it against your database
schema, and generates type-safe Rust code.

> You may be more familiar with `vec!` or `println!` — those are also function-like macros,
> but they are _declarative_ macros (`macro_rules!`), not procedural ones.
> We'll cover the difference in a [later section](04_declarative_vs_procedural.md).

## Attribute macros

Attribute macros are attached to an item (a function, a struct, a module, etc.) and can
**transform** it — replacing, wrapping, or augmenting the original code.
They can also emit _additional_ items alongside the original.

```rust
#[tokio::main]
async fn main() {
    // ...
}

#[tracing::instrument]
fn process(input: &str) -> Result<(), Error> {
    // ...
}
```

[`#[tokio::main]`](https://docs.rs/tokio/latest/tokio/attr.main.html) and
[`#[tracing::instrument]`](https://docs.rs/tracing/latest/tracing/attr.instrument.html) are
attribute macros. They receive the annotated item as input and produce a modified version of it
as output.

## What they have in common

All three types share the same fundamental mechanism: they are **Rust functions** that receive
tokens as input and produce tokens as output. They run at **compile time**, and the tokens they
produce are spliced into the program as if you had written them by hand.

The difference lies in _how_ they are invoked and _what_ they receive as input.

By the end of this workshop, you'll have built all three kinds from scratch — starting with
derive macros, then moving on to function-like macros and attribute macros.
