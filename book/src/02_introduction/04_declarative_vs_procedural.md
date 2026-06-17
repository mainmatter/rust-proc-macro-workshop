# Declarative vs procedural macros

Rust has two macro systems: **declarative macros** (also called "macros by example") and
**procedural macros**. Both generate code at compile time, but they work very differently.

## Declarative macros (`macro_rules!`)

Declarative macros use pattern matching on token trees. You define patterns and their
corresponding expansions:

```rust
macro_rules! max {
    ($a:expr, $b:expr) => {
        if $a > $b { $a } else { $b }
    };
}

let m = max!(3, 7); // expands to: if 3 > 7 { 3 } else { 7 }
```

Declarative macros are:

- Defined inline in your crate, no separate crate needed.
- Limited to pattern matching on tokens — you can't run Rust code during expansion.
- Good for simple syntactic transformations and reducing repetition.

## Procedural macros

Procedural macros are **Rust functions** that receive a token stream and return a token stream.
They can run arbitrary Rust code to decide what to generate:

```rust
use proc_macro::TokenStream;

#[proc_macro_derive(MyDerive)]
pub fn my_derive(input: TokenStream) -> TokenStream {
    // Parse the input, inspect it, generate code
    // — full power of Rust available here
    todo!()
}
```

Procedural macros are:

- Defined in a **dedicated crate** with `proc-macro = true` in its `Cargo.toml`.
- Able to run arbitrary Rust code: parse syntax trees, read attributes, use external libraries.
- Required for derive macros, attribute macros, and complex function-like macros.

## When to use which

|                           | Declarative | Procedural |
| ------------------------- | ----------- | ---------- |
| Simple token substitution | Yes         | Overkill   |
| Variadic arguments        | Yes         | Yes        |
| Inspecting struct fields  | No          | Yes        |
| Custom attributes         | No          | Yes        |
| Derive implementations    | No          | Yes        |
| Needs a separate crate    | No          | Yes        |

A good rule of thumb: start with `macro_rules!`. If you hit its limits — you need to parse
Rust syntax, inspect types, or use attributes — switch to a procedural macro.[^decl-macros-2]

## Exercise

Write a `hashmap!` macro using `macro_rules!` that creates a `HashMap` from key-value pairs.

[^decl-macros-2]: Some of these limits are being lifted. The long-running
["declarative macros 2.0"](https://github.com/rust-lang/rust/issues/39412) effort modernizes
`macro_rules!`, and on top of it [RFC 3698, "Declarative `macro_rules!` derive
macros"](https://github.com/rust-lang/rust/issues/143549) lets you write a `derive` in
`macro_rules!` — inspecting a type's fields and generating an `impl` without a separate
proc-macro crate. The aim is to cover the simple derive cases declaratively; the heavy cases
will still want procedural macros.
