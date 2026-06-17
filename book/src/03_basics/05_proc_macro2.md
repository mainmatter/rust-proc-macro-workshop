# `proc-macro2`

You've been using [`proc_macro::TokenStream`](https://doc.rust-lang.org/proc_macro/struct.TokenStream.html) so far — the type provided by the compiler.
It works, but it has a significant limitation: its types can **only** be used inside a
proc-macro crate (one with `proc-macro = true` in `Cargo.toml`) during compilation. If you try
to use them in a regular library or in a test binary, the code will fail to compile or panic.

## The problem

This limitation makes it hard to:

- **Test** macro logic in unit tests within the proc-macro crate, since [`proc_macro`](https://doc.rust-lang.org/proc_macro/) types
  can't be used in test binaries.
- **Share** code between a proc-macro crate and a regular library.
- **Write** helper functions in separate modules or crates.

## `proc-macro2` to the rescue

The [`proc-macro2`](https://docs.rs/proc-macro2) crate provides a drop-in replacement for the
`proc_macro` types ([`TokenStream`](https://docs.rs/proc-macro2/latest/proc_macro2/struct.TokenStream.html), [`TokenTree`](https://docs.rs/proc-macro2/latest/proc_macro2/enum.TokenTree.html), [`Ident`](https://docs.rs/proc-macro2/latest/proc_macro2/struct.Ident.html), [`Span`](https://docs.rs/proc-macro2/latest/proc_macro2/struct.Span.html), etc.) that works everywhere —
in proc-macro crates, regular libraries, and tests.

```rust
use proc_macro2::TokenStream;
use proc_macro2::{Ident, Span};

fn make_ident(name: &str) -> Ident {
    Ident::new(name, Span::call_site())
}
```

This function compiles in any context, not just in a proc-macro crate.

## Converting between `proc_macro` and `proc-macro2`

At the boundary of your proc-macro function, you convert between the two:

```rust
use proc_macro::TokenStream;     // The "real" type for the macro signature

#[proc_macro_derive(MyMacro)]
pub fn my_macro(input: TokenStream) -> TokenStream {
    // Convert to proc_macro2 for internal use
    let input: proc_macro2::TokenStream = input.into();

    // Do all the work with proc_macro2 types
    let output = generate(input);

    // Convert back to proc_macro for the return type
    output.into()
}

// This function can be tested!
fn generate(input: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    // ...
    # input
}
```

## `syn` and `quote` use `proc-macro2`

Both [`syn`](https://docs.rs/syn) and [`quote`](https://docs.rs/quote) use `proc_macro2` types internally. When `syn` parses a
`proc_macro::TokenStream`, it converts it to `proc_macro2` first. When [`quote!`](https://docs.rs/quote/latest/quote/macro.quote.html) produces a
token stream, it returns a [`proc_macro2::TokenStream`](https://docs.rs/proc-macro2/latest/proc_macro2/struct.TokenStream.html).

This is why you almost always need `proc-macro2` as a dependency alongside `syn` and `quote`.

## Exercise

Refactor a derive macro to move the code generation logic into a separate function that uses
`proc_macro2` types, keeping only the thin wrapper in the proc-macro entry point.
