# Parsing attribute arguments with darling's `FromMeta`

So far the `attr` token stream has been empty — `#[make_public]`, `#[trimmed]`, `#[describe]` take
no arguments. Now we'll use it. Configurable attribute macros read settings from inside the
parentheses:

```rust
#[retry(times = 3, delay_ms = 100)]
#[route(method = "POST", path = "/users")]
#[serde(rename = "id", default)]
```

Everything between the parentheses arrives as the **first** `TokenStream`. You _could_ parse it by
hand with a `syn` `Parse` impl, but you already met the better tool in chapter 4:
[`darling`](https://docs.rs/darling). There you used `FromDeriveInput` to read a derive's
attributes; for an attribute macro's arguments the trait is
[`FromMeta`](https://docs.rs/darling/latest/darling/trait.FromMeta.html), and it works the same
declarative way — describe the arguments as a struct, let `darling` generate the parser and the
error messages.

## A worked example: `#[greeting(...)]`

```rust
use darling::{FromMeta, ast::NestedMeta};
use quote::quote;
use syn::{ItemFn, parse_macro_input};

#[derive(FromMeta)]
struct GreetingArgs {
    message: String,
    #[darling(default)] // optional; `bool::default()` is `false`
    loud: bool,
}

#[proc_macro_attribute]
pub fn greeting(attr: TokenStream, item: TokenStream) -> TokenStream {
    let func = parse_macro_input!(item as ItemFn);

    // 1. Split the attribute tokens into a list of nested-meta items.
    let meta = match NestedMeta::parse_meta_list(attr.into()) {
        Ok(meta) => meta,
        Err(err) => return darling::Error::from(err).write_errors().into(),
    };

    // 2. Parse our schema out of that list.
    let args = match GreetingArgs::from_list(&meta) {
        Ok(args) => args,
        Err(err) => return err.write_errors().into(),
    };

    // ... use `args.message` (a String) and `args.loud` (a bool) to generate code ...
    quote!(#func).into()
}
```

Three things to note:

- **[`NestedMeta::parse_meta_list`](https://docs.rs/darling/latest/darling/ast/enum.NestedMeta.html)**
  turns the raw attribute tokens into the `Vec<NestedMeta>` that
  `FromMeta` consumes. The `attr` parameter is a `proc_macro::TokenStream`, so `attr.into()` converts
  it to the `proc_macro2::TokenStream` that `parse_meta_list` expects. It can fail on malformed input
  — and it fails with a `syn::Error`, so we wrap it with `darling::Error::from(..)` to get a uniform
  error type.
- **`from_list`** is the `FromMeta` entry point, mirroring `from_derive_input` from chapter 4. It
  reports unknown keys, missing required fields, and wrong value types for you.
- **`write_errors()`** renders a `darling::Error` as `compile_error!` tokens. Returning that _is_
  your error path — no `panic!`, a real diagnostic pointing at the bad argument.

Required vs optional works exactly as it did with `FromDeriveInput`: a bare field is required, and
`#[darling(default)]` (or `#[darling(default = "some_fn")]` for a custom default) makes it optional.

## Exercise

Write `#[endpoint(path = "...", method = "...")]`. It re-emits the annotated function and generates
`<name>_path()` and `<name>_method()` accessors returning the parsed values. `path` is **required**;
`method` is **optional and defaults to `"GET"`**.

The code generation is provided — your job is the argument parsing: define the `FromMeta` struct
(with the right required/optional fields) and wire up the two-step parse. The pass test omits
`method` on one endpoint, so you'll need the default to make it compile; the `tests/fail` snapshot
checks that a missing `path` produces a clean darling error.
