# Handling enums

Structs aren't the only thing you can derive on. Enums are just as common — [`serde`](https://serde.rs),
[`strum`](https://docs.rs/strum), and [`thiserror`](https://docs.rs/thiserror) all derive heavily on them. Where a struct has _fields_, an enum has
_variants_, and each variant can itself carry named, unnamed, or no fields.

## `DataEnum` and variants

When the input is an enum, `DeriveInput::data` is `Data::Enum(DataEnum)`. The piece you care
about is the list of variants:

```rust
pub struct DataEnum {
    pub variants: Punctuated<Variant, Comma>,
    // ...
}
```

Each [`Variant`](https://docs.rs/syn/latest/syn/struct.Variant.html) has:

- `ident` — the variant's name (`Circle`, `Point`, …)
- `fields` — a `syn::Fields`, exactly the same enum you met for structs
- `discriminant` — the `= 1` part of `enum E { A = 1 }`, if present

The key realisation: a variant's `fields` is the _same_ `Fields` type as a struct's body. A
variant can be a unit (`Point`), a tuple (`Circle(f64)`), or a struct variant
(`Rectangle { w: f64, h: f64 }`) — and you handle them with the same three-way match.

## Generating a match arm per variant

Most enum derive macros generate a `match self { .. }` with one arm per variant. The tricky
part is writing a pattern that binds — or ignores — the variant's payload. If you only need to
know _which_ variant you're looking at, you can ignore the payload entirely:

```rust
let name = &input.ident; // the enum's name

let arms = data.variants.iter().map(|variant| {
    let vname = &variant.ident;
    let vstr = vname.to_string();

    // A pattern that matches the variant and ignores any data it carries.
    let pattern = match &variant.fields {
        Fields::Unit => quote! { #name::#vname },
        Fields::Unnamed(_) => quote! { #name::#vname(..) },
        Fields::Named(_) => quote! { #name::#vname { .. } },
    };

    quote! { #pattern => #vstr, }
});

quote! {
    impl #name {
        pub fn as_str(&self) -> &'static str {
            match self {
                #(#arms)*
            }
        }
    }
}
```

Note the three pattern shapes — `Foo`, `Foo(..)`, and `Foo { .. }` — must match the variant's
declared shape exactly, or the generated `match` won't compile. `..` is doing the work of
ignoring the payload in the tuple and struct cases.

If instead you needed to _use_ the payload — say, to forward a field — you'd bind it: `Foo(x)`
or `Foo { w, h }`, and reference those bindings in the arm body. We'll do exactly that when we
build more advanced macros later.

## Exercise

The `as_str` macro above is your worked example. For the exercise you'll write a _sibling_ macro,
`Ordinal`, that generates an `ordinal(&self) -> usize` method returning the variant's 0-based
position in declaration order (first variant `0`, second `1`, …).

The outer `match`/`impl` scaffolding is provided; you fill in each arm. Two things to work out:
the arm body is now the variant's _index_ (reach for `enumerate`), and you still have to build a
payload-ignoring pattern whose shape depends on the variant — apply the same `Fields` matching the
`as_str` example uses.
