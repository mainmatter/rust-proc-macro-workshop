# Error reporting: `syn::Error` and `Span`

Returning a hand-built `compile_error!` works, but it doesn't scale. As soon as a macro has
several things that can go wrong — an unsupported shape here, a malformed attribute there — you
want a uniform way to _raise_ an error from deep inside your logic and _report_ it once at the
top. That's what [`syn::Error`](https://docs.rs/syn/latest/syn/struct.Error.html) gives you.

## `syn::Error` and `syn::Result`

A `syn::Error` bundles two things: a message and a **span** (a location in the user's source).
`syn` also defines `type Result<T> = std::result::Result<T, syn::Error>`, the standard return type
for fallible macro logic.

Refactor your implementation function to return a `syn::Result`:

```rust
fn getters_impl(input: &DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    // ...
    return Err(syn::Error::new_spanned(
        input,
        "Getters can only be derived for structs with named fields",
    ));
    // ...
    Ok(quote! { /* ... */ })
}
```

## Creating an error: where does the span come from?

There are two constructors:

- [`Error::new(span, message)`](https://docs.rs/syn/latest/syn/struct.Error.html#method.new) —
  you pass a [`Span`](https://docs.rs/proc-macro2/latest/proc_macro2/struct.Span.html) explicitly.
- [`Error::new_spanned(tokens, message)`](https://docs.rs/syn/latest/syn/struct.Error.html#method.new_spanned)
  — you pass anything that implements `ToTokens`, and `syn` uses _its_ span. This is usually the
  more convenient one: pass the `DeriveInput`, a `Field`, an `Ident`, etc., and the error points
  at that piece of syntax.

The span is what makes the error land in the right place — point at `input` and the error
underlines the whole item; point at a single field (next section) and it underlines just that
field.

## Reporting the error

A `syn::Error` converts to a `compile_error!` token stream with
[`into_compile_error`](https://docs.rs/syn/latest/syn/struct.Error.html#method.into_compile_error)
(or [`to_compile_error`](https://docs.rs/syn/latest/syn/struct.Error.html#method.to_compile_error),
which borrows). The idiomatic top-level wiring is:

```rust
#[proc_macro_derive(Getters)]
pub fn getters(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    getters_impl(&input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
```

`unwrap_or_else` either takes the `Ok` token stream or, on `Err`, turns the error into a
`compile_error!`. Your `_impl` function stays focused on the happy path and can use `?` to
bubble errors up — exactly like ordinary Rust error handling.

> `syn::Error` can also accumulate multiple errors: build several and
> [`combine`](https://docs.rs/syn/latest/syn/struct.Error.html#method.combine) them, and the
> compiler reports them all at once instead of one-at-a-time. Handy for validating every field
> before giving up.

## Exercise

The exercise starts from `Getters` exactly as the previous section left it — errors reported with
a hand-written `compile_error!`, and `getters_impl` returning a plain `TokenStream`. Refactor it
into the `syn::Error` / `syn::Result` form shown above, without changing what the user sees:
return a `syn::Result` from `getters_impl`, turn each `compile_error!` return into an `Err` built
with `syn::Error::new_spanned`, wrap the success path in `Ok`, and convert the error back to tokens
in the entry point. The unit tests call `.unwrap_err()` / `.is_ok()` on `getters_impl`, so they
only compile once it returns a `Result` — that's how you'll know you've threaded the type through.
