# Attribute macros

You've built derive macros (which _add_ code to a type) and function-like macros (which turn a
blob of tokens into code). The third and final kind is the **attribute macro** — the `#[...]` you
write _in front of_ an item to transform it.

You use them all the time:

```rust
#[test]
fn it_works() { /* ... */ }

#[tokio::main]
async fn main() { /* ... */ }

#[tracing::instrument]
fn handle(req: Request) { /* ... */ }
```

[`#[tokio::main]`](https://docs.rs/tokio/latest/tokio/attr.main.html) and
[`#[tracing::instrument]`](https://docs.rs/tracing/latest/tracing/attr.instrument.html) are
attribute macros. Unlike a derive, which leaves the original item untouched and only appends to it,
an attribute macro **receives the item and returns its replacement** — so it can rewrite the body,
change the signature, wrap it, or emit extra items alongside it.

> `#[test]` uses the same `#[...]` syntax and feels identical, but it's built into the compiler's
> test harness rather than being a procedural macro — much as `#[derive(Debug)]` is built in
> (chapter 2). Third-party attributes like `#[tokio::main]` _are_ procedural macros, and work
> exactly as described here.

## Two token streams in

The registering attribute is `#[proc_macro_attribute]`, and the function it marks takes **two**
`TokenStream`s, not one:

```rust
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn my_attr(attr: TokenStream, item: TokenStream) -> TokenStream {
    // `attr` = the tokens *inside* the attribute's parentheses
    // `item` = the annotated item itself
    item
}
```

For an invocation like

```rust
#[my_attr(every = "other", thing = 3)]
fn target() {}
```

- `attr` is `every = "other", thing = 3` — just the arguments, without the surrounding `#[...]`.
- `item` is `fn target() {}` — the whole function.

The macro above is the **identity** attribute: it returns `item` unchanged, so `target` compiles
exactly as written. That's the smallest possible attribute macro, and a useful baseline — a real
one parses `item` (usually into a [`syn::ItemFn`](https://docs.rs/syn/latest/syn/struct.ItemFn.html)),
modifies it, and returns the modified version.

> **Tip: see what you're given.** When you're unsure what's in `attr` or `item`, add
> `eprintln!("{item}")` (a `TokenStream` implements `Display`) at the top of your macro and read it
> during `cargo build`. This shows the macro's _input_; to inspect its _output_, use
> [`cargo expand`](https://github.com/dtolnay/cargo-expand) as you did in chapter 2. The `eprintln!`
> trick still works even when the macro panics or produces invalid tokens — handy when `cargo expand`
> can't render the result.

## Returning the modified item

Because the return value _replaces_ the item, the usual shape is: parse, tweak, re-emit. Here's a
minimal transforming example — `#[auto_inline]`, which tags a function with `#[inline]`:

```rust
use quote::quote;
use syn::{ItemFn, parse_macro_input, parse_quote};

#[proc_macro_attribute]
pub fn auto_inline(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut func = parse_macro_input!(item as ItemFn);
    func.attrs.push(parse_quote!(#[inline])); // add an attribute to the function
    quote!(#func).into() // re-emit the whole function
}
```

`parse_macro_input!(item as ItemFn)` parses the function; `func` is now a mutable syntax tree you
can edit (`func.attrs`, `func.sig`, `func.vis`, `func.block`, …); and `quote!(#func)` turns it back
into tokens — `ItemFn` implements `ToTokens`, so it round-trips cleanly.

> `quote!`, like any macro, accepts `()`, `[]`, or `{}` delimiters. We've used `quote! { ... }` so
> far; the parenthesized `quote!(#func)` just reads nicely when you're re-emitting a single item.

## Exercise

Write `#[make_public]`: an attribute macro that takes a function and re-emits it with `pub`
visibility. It's the smallest useful transform — parse the `ItemFn`, overwrite its visibility, and
return it — and it gets you comfortable with the parse-tweak-re-emit loop before the bigger
transforms in the rest of the chapter.
