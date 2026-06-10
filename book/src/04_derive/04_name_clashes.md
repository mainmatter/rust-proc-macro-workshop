# Preventing name clashes in generated symbols

The previous section was about names your macro _refers to_. This one is about names your macro
_introduces_ — the helper functions, constants, local variables, and types you create in your
generated code. Get these wrong and your macro works in isolation but breaks the moment it's
used twice, or next to the user's own code.

## Why derive output isn't hygienic

`macro_rules!` macros are _hygienic_: a local variable a declarative macro introduces is kept
separate from the caller's variables, even if they share a name. Procedural macros mostly are
**not**. An identifier you build with `format_ident!` or `quote!` is an ordinary call-site
identifier — it lives in the same namespace as the user's code and can collide with it.

There are two failure modes:

1. **Colliding with the user.** You emit `const BUF_SIZE: usize = 1024;` at module scope; the
   user already has a `BUF_SIZE`. Now their crate doesn't compile.
2. **Colliding with yourself.** You emit a fixed-name module-level helper, and the user derives
   your macro on two types in the same module. Your own expansions clash:

   ```rust
   #[derive(Loud)] struct Cat; // emits `fn shout_impl(..)`
   #[derive(Loud)] struct Dog; // emits `fn shout_impl(..)` again -> error[E0428]
   ```

## Strategy 1: scope it away

The cleanest fix is to not put a name at module scope in the first place. Two ways:

- **Put helpers inside the `impl`.** An associated function is namespaced under the type, so
  `Cat`'s and `Dog`'s copies never collide:

  ```rust
  impl #name {
      pub fn shout(&self) -> String { Self::shout_impl(/* .. */) }
      fn shout_impl(/* .. */) -> String { /* .. */ }
  }
  ```

- **Wrap module-level items in an anonymous `const`.** `const _: () = { .. };` gives you a
  fresh, unnamable scope — perfect for things like trait-impl bound checks that can't live
  inside an `impl`:

  ```rust
  const _: () = {
      // helpers here are invisible to the outside world
  };
  ```

## Strategy 2: generate a unique name

When you genuinely need a named item at module scope, make the name unlikely to collide and
unique per type. The convention is a `__` prefix plus the type name, built with
[`format_ident!`](https://docs.rs/quote/latest/quote/macro.format_ident.html):

```rust
let helper = format_ident!("__loud_shout_for_{}", name);

quote! {
    impl #name {
        pub fn shout(&self) -> String { #helper(stringify!(#name)) }
    }
    fn #helper(s: &str) -> String { /* .. */ }
}
```

The `__` prefix keeps you clear of names a user would plausibly write, and splicing in `#name`
guarantees `Cat` and `Dog` get distinct helpers. The same `#helper` interpolation is used at
both the definition and the call site, so they always agree.

> You may have seen crates generate truly outlandish names like `_IMPL_SERIALIZE_FOR_Foo`. That's
> this exact technique — a verbose, deliberately ugly prefix chosen to never collide with
> hand-written code.

## Exercise

`Loud` above was the worked example. For the exercise you'll apply strategy 2 to a trickier case:
the `Accessors` macro generates a module-level accessor _function_ per field (emitted next to the
`impl`, not inside it). The unique name you saw for `Loud` combined a prefix with the _type_ name —
but that isn't enough here. Work out a `format_ident!` scheme that survives both clashes the test
(`examples/accessors.rs`) throws at it: two fields in one struct, and two structs that share a
field name. The functions live at module scope, so strategy 1 doesn't apply.
