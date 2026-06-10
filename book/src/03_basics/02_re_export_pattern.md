# The re-export pattern

In the previous exercise, the code that uses a derive macro had to depend directly on the
proc-macro crate. This works, but there's a problem: proc-macro crates are limited in what
they can export.

A `proc-macro = true` crate can **only** export procedural macros. It cannot export structs,
traits, functions, or any other items. If your macro generates code that references a trait
or helper type, that trait must live somewhere else.

## The problem

Say you're building a `Describe` derive macro that implements a `Describe` trait:

```rust
pub trait Describe {
    fn describe(&self) -> &'static str;
}
```

The trait definition can't go in the proc-macro crate. So where does it go?

## The facade pattern

The standard approach is a **facade crate** that re-exports the macro alongside any traits,
types, or functions that the macro's generated code needs:

```text
my-lib/                  ← facade crate (what users depend on)
├── Cargo.toml
├── src/lib.rs           ← defines the trait, re-exports the macro
│
└── my-lib-macros/       ← proc-macro crate (implementation detail)
    ├── Cargo.toml
    └── src/lib.rs       ← #[proc_macro_derive(Describe)]
```

The facade crate (`my-lib`) depends on the proc-macro crate and re-exports the macro:

```rust
// my-lib/src/lib.rs
pub use my_lib_macros::Describe;

pub trait Describe {
    fn describe(&self) -> &'static str;
}
```

Users only depend on `my-lib`:

```toml
[dependencies]
my-lib = "1.0"
```

And they get both the trait and the macro from a single import:

```rust
use my_lib::Describe;

#[derive(Describe)]
struct Point { x: f64, y: f64 }
```

## Real-world examples

This pattern is everywhere:

- [**`serde`**](https://docs.rs/serde) re-exports macros from `serde_derive`.
- [**`thiserror`**](https://docs.rs/thiserror) re-exports macros from `thiserror-impl`.
- [**`clap`**](https://docs.rs/clap) re-exports macros from `clap_derive`.
- [**`tokio`**](https://docs.rs/tokio) re-exports macros from `tokio-macros`.

If you've ever used `#[derive(Serialize)]` from `serde` without adding `serde_derive` to your
dependencies, it's because `serde` re-exports it for you.

## Exercise

The `macros/` subdirectory already contains a working `Describe` derive macro that implements
a `SelfDescribe` trait. Your job is to wire up the facade pattern:

1. Add the `macros` crate as a path dependency in `Cargo.toml`.
2. Re-export the `Describe` derive macro with `pub use` in `src/lib.rs`, so that downstream
   code can import both the trait and the macro from a single crate.
