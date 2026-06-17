# Parsing Rust code with `syn`

We've used `syn` briefly to parse the input of a derive macro. Let's look more closely at what
`syn` offers and how to navigate the parsed result.

## `DeriveInput`

When you write a derive macro, `syn`'s
[`DeriveInput`](https://docs.rs/syn/latest/syn/struct.DeriveInput.html) type gives you a
structured view of the annotated item:

```rust
pub struct DeriveInput {
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub ident: Ident,
    pub generics: Generics,
    pub data: Data,
}
```

The `data` field tells you whether you're looking at a struct, an enum, or a union:

```rust
pub enum Data {
    Struct(DataStruct),
    Enum(DataEnum),
    Union(DataUnion),
}
```

## Struct fields

For structs, [`DataStruct`](https://docs.rs/syn/latest/syn/struct.DataStruct.html) contains
a `fields` value which can be one of three kinds:

- **Named fields** — `struct Foo { x: i32, y: i32 }`
- **Unnamed fields** (tuple structs) — `struct Foo(i32, i32)`
- **Unit structs** — `struct Foo;`

```rust
use syn::{parse_macro_input, DeriveInput, Data, Fields};

#[proc_macro_derive(FieldCount)]
pub fn field_count(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    // Match on `data` to get the struct body, then on `fields` to
    // handle the three possible field layouts.
    let count = match &input.data {
        Data::Struct(data) => match &data.fields {
            // struct Foo { x: i32, y: i32 }
            Fields::Named(fields) => fields.named.len(),
            // struct Foo(i32, i32)
            Fields::Unnamed(fields) => fields.unnamed.len(),
            // struct Foo;
            Fields::Unit => 0,
        },
        _ => panic!("FieldCount only supports structs"),
    };

    let name = &input.ident;
    let expanded = format!(
        "impl {name} {{ pub fn field_count() -> usize {{ {count} }} }}"
    );
    expanded.parse().unwrap()
}
```

## Inspecting fields

[`FieldsNamed`](https://docs.rs/syn/latest/syn/struct.FieldsNamed.html) contains a `named` list
of [`Field`](https://docs.rs/syn/latest/syn/struct.Field.html) values. Each `Field` has an
`ident` (the field name, as an `Option<Ident>`) and a `ty` (the type). For named fields,
`ident` is always `Some` — it's `None` only for tuple struct fields:

```rust
if let Data::Struct(data) = &input.data {
    if let Fields::Named(fields) = &data.fields {
        // `fields.named` is a punctuated list of `Field` values.
        // Each `Field` has an `ident` and a `ty`.
        for field in &fields.named {
            // Safe to unwrap: named fields always have an ident.
            let name = field.ident.as_ref().unwrap();
            // `ty` is a `syn::Type` — the parsed representation of the field's type.
            let ty = &field.ty;
            eprintln!("field: {name} has type {ty:?}");
        }
    }
}
```

We'll use this extensively when building derive macros that need to generate code for each
field — which is most of them.

## Feature flags

`syn` is a large crate. To keep compile times down, most of its functionality is behind
feature flags. The most commonly used ones are:

- **`full`** — enables parsing of all Rust syntax (statements, expressions, patterns, etc.)
- **`derive`** — enables parsing of `DeriveInput` (this is on by default)
- **`parsing`** — enables the parsing infrastructure
- **`printing`** — enables converting `syn` types back to tokens

For derive macros, the `derive` feature (on by default) is usually sufficient. We enabled
`full` in this workshop so you have access to everything.

## Exercise

Write a derive macro that reads the fields of a named struct and generates a
`field_count()` method returning the number of fields.
