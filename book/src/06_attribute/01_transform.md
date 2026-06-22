# Transforming a function

The last section tweaked a function's _attributes_ and _visibility_. The real workhorse of
attribute macros is rewriting the **body** — wrapping the original code so something extra happens
around it. That's how `#[tracing::instrument]` adds a span, how a `#[test]`-style harness reports
results, and how the `#[retry]` macro you'll build at the end of the chapter works.

## A worked example: `#[timed]`

Let's wrap a function so it prints how long it took to run. From the caller's side, you just slap
the attribute on a function and use it as normal:

```rust
#[timed]
fn slow_add(a: u64, b: u64) -> u64 {
    std::thread::sleep(std::time::Duration::from_millis(10));
    a + b
}

fn main() {
    let sum = slow_add(1, 2);
    // prints something like: `slow_add took 10.1ms` to stderr
    assert_eq!(sum, 3);
}
```

The function keeps its signature, arguments, and return value — `#[timed]` only adds the timing
message around the original body. Here's the macro that makes it work:

```rust
use quote::quote;
use syn::{ItemFn, parse_macro_input, parse_quote};

#[proc_macro_attribute]
pub fn timed(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut func = parse_macro_input!(item as ItemFn);

    let name = func.sig.ident.to_string(); // the function's name, for the message
    let block = &func.block;               // the original body: `{ ... }`

    // A new body that runs the old one, then reports the elapsed time.
    let new_body: syn::Block = parse_quote! {{
        let __start = ::std::time::Instant::now();
        let __result = #block;
        ::std::eprintln!("{} took {:?}", #name, __start.elapsed());
        __result
    }};

    func.block = ::std::boxed::Box::new(new_body);
    quote!(#func).into()
}
```

The key moves:

- **Grab the original body.** [`func.block`](https://docs.rs/syn/latest/syn/struct.ItemFn.html#structfield.block)
  is a [`syn::Block`](https://docs.rs/syn/latest/syn/struct.Block.html) — the braces and everything inside them.
- **Build a replacement body.** `parse_quote! {{ ... }}` parses tokens into a `syn::Block` (the
  outer braces are the macro's `{{`/`}}` delimiter; the inner braces are the block itself). Interpolating
  `#block` splices the original body in _with_ its braces, so `let __result = #block;` runs it as a
  block expression and captures its value.
- **Preserve the result.** The wrapper stashes the body's value in `__result`, does its extra work,
  then yields `__result` as the block's final expression — so the function still returns what it
  always did. The signature is untouched; only the body changed.
- **Put it back.** [`func.block`](https://docs.rs/syn/latest/syn/struct.ItemFn.html#structfield.block)
  wants a `Box<syn::Block>`, then `quote!(#func)` re-emits the whole function.

> **A note on `return`.** Because the original body becomes a block _expression_ — as in `#[timed]`
> above — an early `return` inside it returns from the whole function and skips the wrapper's
> trailing code. For timing that's
> usually fine; when it isn't (as with `#[retry]` later), the trick is to run the body inside a
> closure instead, so `return` exits the closure rather than the function.

## Exercise

Apply the same wrap-the-body technique to _transform the result_. Write `#[trimmed]`: it takes a
`String`-returning function and rewrites the body so the returned string is trimmed of surrounding
whitespace. Capture the original body's value, post-process it, and return the new value — leaving
the function's signature and arguments exactly as they were. Make `examples/trim.rs` pass.
