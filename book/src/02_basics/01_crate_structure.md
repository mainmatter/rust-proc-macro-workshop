# Crate structure

A procedural macro must live in its own crate, separate from the code that uses it. This isn't
a convention — it's a hard requirement enforced by the compiler.

## Why a separate crate?

Procedural macros run at **compile time**. They are compiled separately and loaded by the
compiler while building the crate that uses the macro. This means the macro code and the
code that uses it cannot be in the same crate — the macro needs to be fully compiled before the
compiler can process the code that invokes it.

## Creating a proc-macro crate

A proc-macro crate looks like a regular library crate with one key difference in `Cargo.toml`:

```toml
[package]
name = "my-macros"
version = "0.1.0"
edition = "2024"

[lib]
proc-macro = true
```

The `proc-macro = true` flag tells the compiler that this crate exports procedural macros.
Without it, attempting to use `#[proc_macro_derive]` or any other proc-macro attribute will
produce a compilation error.

## The `proc_macro` crate

When you set `proc-macro = true`, your crate automatically gets access to the
[`proc_macro`](https://doc.rust-lang.org/proc_macro/) crate from the standard library.
You don't need to add it to `[dependencies]` — it's provided by the compiler.

This crate gives you the core type you'll work with: `TokenStream`. Every procedural macro
is a function that takes a `TokenStream` as input and returns a `TokenStream` as output.

```rust
use proc_macro::TokenStream;

#[proc_macro_derive(MyMacro)]
pub fn my_macro(_input: TokenStream) -> TokenStream {
    // Return nothing — the derive macro adds no extra code for now.
    TokenStream::new()
}
```

## Wiring it up

To use your proc-macro crate from another crate, add it as a dependency:

```toml
[dependencies]
my-macros = { path = "../my-macros" }
```

Then you can use the derive macro:

```rust
use my_macros::MyMacro;

#[derive(MyMacro)]
struct Foo;
```

## Workspace layout

In a workspace, proc-macro crates are just regular members. This workshop uses a workspace
with exercises as members, and some exercises will include their own proc-macro crate as a
path dependency.

## Exercise

Create a proc-macro crate from scratch and wire it up. You'll need to create the `Cargo.toml`
(with `proc-macro = true`), write a derive macro that returns an empty token stream, and add
it as a dependency. Then uncomment the provided code to verify everything compiles.
