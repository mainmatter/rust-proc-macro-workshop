# Handling generics

Every macro so far assumed the type was non-generic. The moment a user writes
`struct Wrapper<T> { value: T }`, a naive `impl Wrapper { .. }` is wrong — it's missing the `<T>`,
and the body probably needs `T` to satisfy some bound. Handling generics correctly is the last
piece before you can write a macro people actually depend on.

## What an impl header needs

For `struct Wrapper<'a, T: Clone>`, the impl you generate has to look like:

```rust
impl<'a, T: Clone> Wrapper<'a, T> { /* ... */ }
//  ^^^^^^^^^^^^^^^         ^^^^^^^
//  impl_generics           ty_generics
```

Three distinct pieces, and they're _not_ identical: the `impl<...>` list carries bounds and
lifetimes, while the `Wrapper<...>` list is just the parameter names. Plus any `where` clause.
Getting these by hand is error-prone.

## `split_for_impl`

`syn`'s [`Generics::split_for_impl`](https://docs.rs/syn/latest/syn/struct.Generics.html#method.split_for_impl)
hands you all three, ready to interpolate:

```rust
let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

quote! {
    impl #impl_generics #name #ty_generics #where_clause {
        // ...
    }
}
```

Each piece knows how to render itself in the right spot: `#impl_generics` becomes `<'a, T: Clone>`,
`#ty_generics` becomes `<'a, T>`, and `#where_clause` becomes a `where ...` (or nothing). For a
non-generic type all three are empty, so the same code handles both cases.

## Adding your own bounds

`split_for_impl` faithfully reproduces the _user's_ generics — but your generated code often needs
_more_. If your body calls `format!("{:?}", ..)` on a field of type `T`, then `T: Debug` must hold,
and the user didn't write that. The fix is to add the bound before splitting: clone the generics,
push a bound onto each type parameter, then split the modified copy.

```rust
let mut generics = input.generics.clone();
for param in &mut generics.params {
    if let syn::GenericParam::Type(type_param) = param {
        type_param.bounds.push(syn::parse_quote!(::std::fmt::Debug));
    }
}
let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
```

Notes:

- [`parse_quote!`](https://docs.rs/syn/latest/syn/macro.parse_quote.html) is like `quote!`, but it
  _parses_ the tokens into a `syn` type — here a `TypeParamBound`. It's the easiest way to build
  small `syn` values from real syntax.
- Match only on `GenericParam::Type`; lifetimes (`GenericParam::Lifetime`) and const generics
  (`GenericParam::Const`) take no such bound.
- Pushing onto a parameter's `bounds` _adds_ to any bound the user already wrote (`T: Clone`
  becomes `T: Clone + Debug`) rather than replacing it.

> This blanket "add `Debug` to every type parameter" is the same heuristic `#[derive(Debug)]` uses.

## Exercise

The worked example above added a `Debug` bound for `DebugFields`. For the exercise you'll build a
different generic-aware macro, `Empty`, which generates an `empty() -> Self` constructor setting
every field to `Default::default()`. `split_for_impl` is already wired into the generated `impl`;
your job is the bound. Work out which trait bound the generated body now requires, and add it to
each type parameter the same way the example did. The `plain` test already passes — get `generic`
(which has type parameters) to pass too.
