# Field attributes

Container attributes configure the whole type; **field attributes** configure a single field —
`serde`'s `#[serde(rename = "...")]`, `#[serde(skip)]`, and friends. The mechanics are the same
as the previous section, with one difference: you read the attributes off each `Field` instead
of off the `DeriveInput`.

## One declaration, two positions

You still declare the helper attribute once, on the derive:

```rust
#[proc_macro_derive(Labelled, attributes(label))]
```

That single declaration covers the attribute wherever it legally appears — on the container _and_
on fields. Which positions you actually _read_ is up to your macro. Here we read it per field, so
we look at `field.attrs` rather than `input.attrs`:

```rust
fn field_label(field: &syn::Field) -> syn::Result<String> {
    for attr in &field.attrs {
        if attr.path().is_ident("label") {
            // ... read this field's label
        }
    }
    // default: the field's own name
    Ok(field.ident.as_ref().unwrap().to_string())
}
```

## `parse_args` for a single value

The previous section used `parse_nested_meta` for a comma-separated `key = value` list. When the
attribute holds a _single_ value — `#[label("Email address")]` — there's a more direct tool:
[`Attribute::parse_args`](https://docs.rs/syn/latest/syn/struct.Attribute.html#method.parse_args),
which parses the tokens inside the parentheses as any `Parse` type:

```rust
let s: syn::LitStr = attr.parse_args()?;
let label = s.value();
```

`parse_args` works with any type implementing `syn::parse::Parse` — `LitStr`, `LitInt`, `Ident`,
`Type`, or your own custom-parsed struct. It's the right reach whenever an attribute carries one
self-contained argument.

Rust offers three attribute _shapes_, and syn has a tool for each:

- **bare path** — `#[label]` — check `attr.path().is_ident(..)`
- **name-value** — `#[label = "x"]` — read it with `attr.meta.require_name_value()`
- **list** — `#[label(...)]` — parsed with one of two tools, depending on its contents:
  - one self-contained argument — `#[label("x")]` — `attr.parse_args()`
  - a `key = value` list — `#[label(a = 1, b = 2)]` — `attr.parse_nested_meta(..)`

Picking the shape is a design choice; following one ecosystem's convention — usually serde's —
keeps your macro familiar to users.

## Reading a name-value attribute

`parse_args` is the easy path, but it only applies to the _list_ shape. The name-value shape
`#[label = "x"]` doesn't go through a parser — `require_name_value()` hands you a
[`MetaNameValue`](https://docs.rs/syn/latest/syn/struct.MetaNameValue.html) whose `.value` is a
plain [`syn::Expr`](https://docs.rs/syn/latest/syn/enum.Expr.html), and you dig the literal out
yourself. A string literal arrives as `Expr::Lit` wrapping a `Lit::Str`, so you match those two
layers and reject anything else with a spanned error:

```rust
let nv = attr.meta.require_name_value()?;
match &nv.value {
    syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Str(s), .. }) => s.value(),
    other => return Err(syn::Error::new_spanned(other, "expected a string literal")),
}
```

Spanning the error on `other` (the offending expression) makes the diagnostic point at exactly
what the user wrote after the `=`, the same span discipline from the error-reporting sections.

## Collecting fallible results

Computing one label per field is a `map` that can fail, so collect into a `Result` to
short-circuit on the first bad field:

```rust
let labels = fields
    .iter()
    .map(field_label)
    .collect::<syn::Result<Vec<String>>>()?;
```

This is a handy idiom: `collect()` turns an iterator of `Result`s into a `Result` of a `Vec`,
propagating the first `Err`.

## Exercise

`Labelled` above used the _list_ shape `#[label("...")]` with `parse_args`. For the exercise
you'll build `Renamed`, which uses the _name-value_ shape instead: `#[rename = "..."]`. It
generates a `column_names()` method returning each field's name, overridden by `#[rename]` when
present. The iteration and `require_name_value` call are wired up; your job is to pull the string
out of the attribute's value expression — a different job from `parse_args`, since here you
inspect a `syn::Expr` yourself (see the three-shapes aside above).
