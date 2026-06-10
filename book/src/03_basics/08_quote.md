# Generating code with `quote`

Building token streams from strings is fragile and unreadable. The
[`quote`](https://docs.rs/quote) crate provides a macro that lets you write code generation
that looks almost like regular Rust.

## The `quote!` macro

`quote!` takes Rust-like syntax and produces a `proc_macro2::TokenStream`:

```rust
use quote::quote;

let tokens = quote! {
    fn hello() -> &'static str {
        "hello!"
    }
};
```

## Interpolation with `#`

The real power of `quote!` is interpolation. Use `#variable` to splice a value into the
generated code:

```rust
use quote::quote;
use syn::Ident;

let name = Ident::new("Foo", proc_macro2::Span::call_site());
let tokens = quote! {
    impl #name {
        pub fn type_name() -> &'static str {
            stringify!(#name)
        }
    }
};
```

[`stringify!`](https://doc.rust-lang.org/std/macro.stringify.html) is a built-in macro that
turns its argument into a string literal at compile time — `stringify!(Foo)` becomes `"Foo"`.
It's useful in generated code because it works with any identifier without needing to build
string literals yourself.

This generates:

```rust
impl Foo {
    pub fn type_name() -> &'static str {
        stringify!(Foo) // expands to "Foo"
    }
}
```

Any type that implements the [`ToTokens`](https://docs.rs/quote/latest/quote/trait.ToTokens.html)
trait can be interpolated. `syn` types like `Ident`, `Type`, and `Path` all implement it, so
you can splice parsed values directly into generated code.

## Repetition with `#( ... )*`

When you need to generate code for each item in a collection, `quote!` supports repetition
syntax similar to `macro_rules!`:

```rust
use quote::quote;

let field_names = vec!["x", "y", "z"];
let field_idents: Vec<syn::Ident> = field_names
    .iter()
    .map(|n| syn::Ident::new(n, proc_macro2::Span::call_site()))
    .collect();

let tokens = quote! {
    fn field_names() -> &'static [&'static str] {
        &[#(stringify!(#field_idents)),*]
    }
};
```

Let's break down `#(stringify!(#field_idents)),*`:

- `#( ... ),*` — the repetition operator. It iterates over `field_idents`, producing each
  element separated by commas (`,`). The `*` means "zero or more times."
- `stringify!(#field_idents)` — the body repeated for each element. `#field_idents` is replaced
  with the current `Ident` on each iteration.

So for `field_idents = [x, y, z]`, this expands to:

```rust
stringify!(x), stringify!(y), stringify!(z)
```

which at compile time becomes `"x", "y", "z"`.

This is the same pattern you'll use to generate code for each field in a struct.

## `quote!` vs string formatting

Compare these two approaches:

```rust
// String formatting — fragile, no syntax highlighting, easy to get wrong
let code = format!(
    "impl {} {{ fn count() -> usize {{ {} }} }}",
    name, count
);
let tokens: TokenStream = code.parse().unwrap();

// quote! — looks like Rust, catches many errors at compile time
let tokens = quote! {
    impl #name {
        fn count() -> usize { #count }
    }
};
```

`quote!` is not just more readable — it catches many mistakes at compile time rather than
producing confusing errors for macro users.

## `format_ident!`

The [`format_ident!`](https://docs.rs/quote/latest/quote/macro.format_ident.html) macro lets you
construct new identifiers programmatically, similar to how `format!` builds strings:

```rust
use quote::format_ident;

let field = "name";
let getter = format_ident!("get_{field}");
// `getter` is the identifier `get_name`
```

This is useful when your macro needs to generate new method or type names based on the input.

## `quote_spanned!`

`quote!` has a variant, [`quote_spanned!`](https://docs.rs/quote/latest/quote/macro.quote_spanned.html),
that lets you attach span information to the generated tokens. This controls where error
messages point when the generated code fails to compile. We'll use it in later chapters when
we cover error reporting and spans.

## Exercise

The exercise provides a working derive macro that generates a `field_names()` method using
string formatting. Rewrite it to use `quote!` instead — the logic stays the same, only the
code generation changes.
