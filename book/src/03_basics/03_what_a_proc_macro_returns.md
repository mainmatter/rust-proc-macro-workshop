# What a proc macro returns

A derive macro receives the annotated item as a [`TokenStream`](https://doc.rust-lang.org/proc_macro/struct.TokenStream.html) and returns a `TokenStream`.
But what exactly should that returned stream contain?

## Derive macros _add_ code

Unlike attribute macros, a derive macro **does not replace** the original item. The struct or
enum you annotate with `#[derive(...)]` is preserved as-is. The token stream you return is
**appended** after the original item.

This means:

- If you return an **empty** token stream, the item is unchanged. This is valid — the macro
  simply has no effect.
- If you return an `impl` block, that implementation is added alongside the original item.
- If you return malformed tokens, compilation fails.

```rust
use proc_macro::TokenStream;

#[proc_macro_derive(NoOp)]
pub fn no_op(_input: TokenStream) -> TokenStream {
    // Return nothing — the item is preserved, no code is added.
    TokenStream::new()
}
```

## The returned tokens must be valid Rust

The compiler will try to parse and compile whatever tokens you return. If they aren't valid
Rust, you'll get a compilation error — but the error will point at the `#[derive(...)]`
invocation, not at the generated code. This can be confusing.

[`cargo expand`](https://github.com/dtolnay/cargo-expand) is your best friend here. When the
generated code doesn't compile, expand it to see exactly what was produced.

## A working example

Here's a derive macro that generates a simple method:

```rust
use proc_macro::TokenStream;

#[proc_macro_derive(Hello)]
pub fn hello(_input: TokenStream) -> TokenStream {
    // For now, hardcode the struct name.
    // We'll learn how to read it from the input later.
    "impl Greeting { fn hello() -> &'static str { \"hello!\" } }"
        .parse()
        .unwrap()
}
```

This works, but it's fragile — the struct name is hardcoded. In the next exercises, you'll
learn how to parse the input to extract the actual name.

## Returning tokens from a string

The simplest way to produce a `TokenStream` is to parse it from a string using the
[`str::parse()`](https://doc.rust-lang.org/std/primitive.str.html#method.parse) method, as shown above. This is fine for quick experiments, but it gives you
no compile-time checks on the generated code — any mistake in the string only shows up
when someone uses the macro.

Later, we'll use the [`quote!`](https://docs.rs/quote/latest/quote/macro.quote.html) macro, which provides a much better way to construct token
streams. We'll also see how to report errors properly using [`compile_error!`](https://doc.rust-lang.org/std/macro.compile_error.html) and [`syn::Error`](https://docs.rs/syn/latest/syn/struct.Error.html),
instead of relying on confusing error messages from malformed tokens.

## Exercise

Write a derive macro that generates a method returning the name of the type as a string.
For now, you can hardcode the name.
