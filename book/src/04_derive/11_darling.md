# Parsing attributes with `darling`

The last two sections parsed attributes by hand: loop over `attrs`, match on `path()`, call
`parse_nested_meta`, pull out values, raise errors for anything unexpected. It works, but it's a
lot of fiddly, repetitive code — and it grows fast as you add options. [`darling`](https://docs.rs/darling)
is a crate that generates all of that for you from a plain struct, the same way `serde` generates
(de)serialization.

## Describe the attributes as a struct

With `darling` you _declare_ the shape of your attributes as a Rust struct and derive the
parsing. There are derives mirroring each level of the input:

- [`FromDeriveInput`](https://docs.rs/darling/latest/darling/trait.FromDeriveInput.html) — the
  whole item, including container attributes.
- [`FromField`](https://docs.rs/darling/latest/darling/trait.FromField.html) — a single field.
- [`FromVariant`](https://docs.rs/darling/latest/darling/trait.FromVariant.html) — an enum variant.
- [`FromMeta`](https://docs.rs/darling/latest/darling/trait.FromMeta.html) — a single value, so
  you can parse attributes into your own enums and types.

```rust
use darling::{FromDeriveInput, FromField, ast, util::Ignored};

#[derive(FromDeriveInput)]
#[darling(attributes(model), supports(struct_named))]
struct ModelOpts {
    ident: syn::Ident,
    data: ast::Data<Ignored, FieldOpts>,
    #[darling(default = "default_table")]
    table: String,
}

fn default_table() -> String {
    "items".to_string()
}

#[derive(FromField)]
#[darling(attributes(model))]
struct FieldOpts {
    ident: Option<syn::Ident>,
    rename: Option<String>, // `Option<T>` is optional automatically
}
```

A few things `darling` is doing here:

- **Magic fields.** `ident`, `data`, `attrs`, `generics`, `vis` are recognised by name and filled
  from the input — `ident` is the type name, `data` is the body.
- **`ast::Data<V, F>`** captures the body generically: `V` is the per-variant type (here `Ignored`,
  since we only support structs), `F` is the per-field type. Later you call `.take_struct()` to get
  the fields back as `FieldOpts` values.
- **`supports(struct_named)`** rejects enums, tuple structs, and unit structs _for you_, with a
  proper error — no hand-written `match` on `Data`.
- **Optional options.** An `Option<T>` field (like `rename`) is optional automatically — it
  defaults to `None` with no annotation. For a _non-`Option`_ field you opt in with
  `#[darling(default)]` (falls back to the type's `Default`) or `#[darling(default = "...")]` (calls
  a named function). `table` uses the latter: `#[darling(default = "default_table")]` calls
  `default_table()`, which returns `"items"`, when the user omits `#[model(table = "...")]`.
  Unknown keys and type mismatches become precise `darling::Error`s automatically.

## Wire it into the macro

Parsing is then a single fallible call, and `darling::Error` renders itself straight to
`compile_error!` tokens:

```rust
#[proc_macro_derive(Model, attributes(model))]
pub fn model(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    model_impl(&input)
        .unwrap_or_else(|err| err.write_errors())
        .into()
}

fn model_impl(input: &DeriveInput) -> darling::Result<proc_macro2::TokenStream> {
    let opts = ModelOpts::from_derive_input(input)?;
    // ... use opts.ident, opts.table, opts.data.take_struct(), ...
}
```

Compare this to section 9: the same configurability, but the parsing, defaulting, validation, and
error reporting are all generated. For anything beyond one or two trivial options, `darling` is
what real macro crates reach for.

## Exercise

Complete the `Model` derive macro. The `darling` schema (`ModelOpts` and `FieldOpts`) is written
for you — study it. Your job is the code generation: call `from_derive_input`, then emit
`table_name()` and `columns()` from the parsed options, using each field's `rename` when present.
