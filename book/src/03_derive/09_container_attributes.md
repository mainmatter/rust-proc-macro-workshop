# Container attributes

This is what made `serde` feel magic back in the overview: attributes that let the _user_ tune
what your macro generates. We'll start with **container attributes** — the ones that sit on the
struct or enum itself, like `#[serde(rename_all = "camelCase")]`.

## Declaring a helper attribute

By default, an attribute like `#[greeting(prefix = "Hi")]` is unknown to the compiler — it would
reject it before your macro ever ran. A derive macro has to _declare_ the helper attributes it
intends to read, in the `attributes(...)` list of its `proc_macro_derive`:

```rust
#[proc_macro_derive(Greeting, attributes(greeting))]
pub fn greeting(input: TokenStream) -> TokenStream { /* ... */ }
```

Now `#[greeting(...)]` is recognised (but inert): the compiler leaves it attached to the item for
your derive to inspect, and ignores it for every other purpose.

## Finding your attribute

Attributes on the container live in `DeriveInput::attrs`, a `Vec<Attribute>`. It contains _every_
attribute on the item — `#[derive(..)]`, `#[doc = ".."]`, and yours — so the first step is always
to filter down to the ones you own with
[`Attribute::path`](https://docs.rs/syn/latest/syn/struct.Attribute.html#method.path):

```rust
for attr in &input.attrs {
    if !attr.path().is_ident("greeting") {
        continue;
    }
    // ... this one is ours
}
```

## Parsing the contents with `parse_nested_meta`

The arguments inside `greeting( ... )` are _your_ mini-syntax to parse. For the common
`key = value, key = value` shape, syn provides
[`Attribute::parse_nested_meta`](https://docs.rs/syn/latest/syn/struct.Attribute.html#method.parse_nested_meta),
which calls your closure once per comma-separated entry:

```rust
attr.parse_nested_meta(|meta| {
    if meta.path.is_ident("prefix") {
        let value = meta.value()?;          // parse stream after the `=`
        let s: syn::LitStr = value.parse()?; // expect a string literal
        prefix = s.value();                  // assign to the `let mut prefix` above — not `let`
        Ok(())
    } else {
        Err(meta.error("unsupported greeting attribute; expected `prefix`"))
    }
})?;
```

Things to notice:

- `meta.path` is the key (`prefix`); `meta.value()` returns a parse stream for the value _after_
  the `=`. Parse it into whatever you expect — `LitStr`, `LitInt`, a type, a path.
- An unrecognised key returns `meta.error(..)`, a `syn::Error` already spanned on the offending
  key. This is the error machinery from earlier sections paying off: malformed attributes get a
  precise diagnostic for free.
- `parse_nested_meta` also handles flags (`#[greeting(loud)]`, where `meta.path` is set but there's
  no value) and nested lists — it's the general-purpose tool for hand-written attribute parsing.
- The closure can fail (it returns a `Result`), so it can't own the result. It instead _assigns_ to
  a `let mut prefix` declared before the loop and captured by the closure — `prefix = …`, not
  `let prefix = …`. A `let` would make a throwaway local and leave the outer variable untouched.

## Exercise

`Greeting` above read a _string_ attribute. For the exercise you'll build the `Repeat` macro,
which reads an _integer_ container attribute — `#[repeat(times = N)]` — and generates a
`repeated()` method returning the type's name repeated `N` times (default `1`). The attribute is
declared and the `parse_nested_meta` loop is wired up; your job is the value extraction. It's the
same shape as the worked example, but the value isn't a string this time — find the right literal
type and convert it to a number.
