# Handling struct fields

In chapter 3 you wrote macros that read a struct's fields, but only for **named** structs —
`struct Foo { x: i32 }`. Real derive macros have to cope with all three field layouts Rust
allows. Let's look at each, and at how you generate code that _accesses_ a field once you've
found it.

## The three field layouts

`syn` models the body of a struct as a [`Fields`](https://docs.rs/syn/latest/syn/enum.Fields.html)
enum with one variant per layout:

```rust
pub enum Fields {
    Named(FieldsNamed),     // struct Foo { x: i32, y: i32 }
    Unnamed(FieldsUnnamed), // struct Foo(i32, i32)
    Unit,                   // struct Foo;
}
```

- **Named** — `fields.named` is a list of `Field`s, each with an `ident` (`Some`) and a `ty`.
- **Unnamed** (tuple struct) — `fields.unnamed` is a list of `Field`s, each with `ident == None`
  and a `ty`. The fields are addressed by position, not by name.
- **Unit** — no fields at all.

A derive macro that ignores tuple and unit structs will simply panic (or, worse, generate
broken code) the first time a user reaches for one. Handling all three is the baseline for a
well-behaved macro.

## Accessing fields in generated code

Reading the field list is only half the job — your generated code usually needs to _access_
each field through `self`. How you write that access depends on the layout:

- For a **named** field, you use its identifier: `self.x`.
- For a **tuple** field, you use its position: `self.0`.

For named fields, you already have the `Ident`, so interpolation is straightforward:

```rust
let ident = field.ident.as_ref().unwrap();
quote! { self.#ident } // -> self.x
```

For tuple fields there is no identifier, so you build the index from the field's position. Use
[`syn::Index`](https://docs.rs/syn/latest/syn/struct.Index.html) rather than a plain integer:

```rust
let index = syn::Index::from(i); // i is the field's position
quote! { self.#index } // -> self.0
```

Why `syn::Index` and not a bare `usize`? Because `quote!` renders a `usize` literal with its
type suffix — `0usize` — so `self.#index` would expand to the invalid `self.0usize`.
`syn::Index` is `quote`-aware and renders as a plain `0`. This is a small but classic papercut.

## Putting it together

Iterating over the fields and emitting one snippet per field is the bread-and-butter of derive
macros. A sketch that builds one expression per field, for both named and tuple structs:

```rust
let accessors: Vec<proc_macro2::TokenStream> = match fields {
    Fields::Named(fields) => fields
        .named
        .iter()
        .map(|f| {
            let ident = f.ident.as_ref().unwrap();
            quote! { self.#ident }
        })
        .collect(),
    Fields::Unnamed(fields) => fields
        .unnamed
        .iter()
        .enumerate()
        .map(|(i, _)| {
            let index = syn::Index::from(i);
            quote! { self.#index }
        })
        .collect(),
    Fields::Unit => Vec::new(),
};

// ... then splice them with repetition:
quote! { vec![ #(#accessors),* ] };
```

This sketch collects the bare accessors (`self.x`, `self.0`); in the exercise you'll wrap each one
in `format!("{:?}", &…)` instead, but the per-field iteration and the repetition splice are
exactly the same shape.

## Exercise

Complete the `DebugFields` derive macro so it works for named, tuple, **and** unit structs.
It generates a `debug_fields(&self) -> Vec<String>` method returning the `{:?}` representation
of each field. The named-struct case is done for you; fill in the tuple and unit cases.
