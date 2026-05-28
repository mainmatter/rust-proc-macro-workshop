# A minimal derive macro

So far we've either returned an empty token stream or hardcoded the output. To write a
useful derive macro, we need to read the input — at minimum, we need to know the name of the
type we're deriving for.

## Parsing the input (the hard way)

The `TokenStream` you receive is a sequence of tokens. You _could_ convert it to a string
and try to extract the struct name manually:

```rust
use proc_macro::TokenStream;

#[proc_macro_derive(TypeName)]
pub fn type_name(input: TokenStream) -> TokenStream {
    let input = input.to_string();
    // Crude: find the word after "struct"
    let name = input
        .split_whitespace()
        .skip_while(|t| *t != "struct")
        .nth(1)
        .expect("expected a struct");

    format!(
        "impl {name} {{ pub fn type_name() -> &'static str {{ \"{name}\" }} }}"
    )
    .parse()
    .unwrap()
}
```

This works for `struct Foo {}`, but it's fragile. It breaks on generics (`struct Foo<T>`),
visibility modifiers (`pub struct Foo`), attributes, and many other valid Rust constructs.

## Parsing the input with `syn`

The [`syn`](https://docs.rs/syn) crate provides a full Rust parser. It can parse a
`TokenStream` into a structured representation that's easy to work with:

```rust
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(TypeName)]
pub fn type_name(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    format!(
        "impl {name} {{ pub fn type_name() -> &'static str {{ \"{name}\" }} }}"
    )
    .parse()
    .unwrap()
}
```

[`DeriveInput`](https://docs.rs/syn/latest/syn/struct.DeriveInput.html) is a `syn` type that
represents the input to a derive macro. It gives you:

- `ident` — the name of the type
- `generics` — any generic parameters
- `data` — the body (struct fields or enum variants)
- `attrs` — attributes on the type
- `vis` — the visibility (`pub`, `pub(crate)`, etc.)

For now, we only need `ident`. We'll use the other fields in later chapters.

## `parse_macro_input!`

The [`parse_macro_input!`](https://docs.rs/syn/latest/syn/macro.parse_macro_input.html) macro
from `syn` parses a `TokenStream` into any type that implements
[`syn::parse::Parse`](https://docs.rs/syn/latest/syn/parse/trait.Parse.html). If parsing fails,
it automatically emits a compiler error and returns early from the function.

The `as DeriveInput` part tells `parse_macro_input!` which type to parse the tokens into.
A `TokenStream` is just a sequence of tokens — it has no structure. By specifying
`as DeriveInput`, you're saying "parse these tokens as a struct/enum/union definition."
`syn` provides other target types too — for example, you could parse `as syn::ItemFn` to parse
a function definition ([`ItemFn`](https://docs.rs/syn/latest/syn/struct.ItemFn.html)). The `as` syntax here is specific to `parse_macro_input!`; it's not
the same as Rust's type-casting `as` operator.

## Exercise

Write a derive macro that reads the type name from the input using `syn` and generates an
`impl` block. This time, the generated code must work for any struct name, not just a
hardcoded one.
