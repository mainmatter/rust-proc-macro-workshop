# Graceful error handling: re-emitting the original item

Attribute macros have an error-reporting hazard that derive macros don't.

A derive macro only ever _adds_ code, so if it bails out the original type is still there. An
attribute macro **replaces** the item it annotates — whatever you return _becomes_ the item. That
makes the error path easy to get wrong, in two ways that both produce baffling diagnostics:

- **Panicking.** A `panic!`, an `unwrap`, or a stray `todo!()` in your macro surfaces as
  `error: custom attribute panicked` with your message but **no span** pointing at the user's code.
  Never report errors by panicking.
- **Dropping the item silently.** If a branch returns empty or partial tokens _without_ a
  diagnostic, the annotated function simply vanishes — and the user gets
  `error[E0425]: cannot find function 'greet'` at every call site, with nothing explaining _why_
  it's missing.

## The fix

On the error path, do two things: produce a real `compile_error!`, and re-emit the original item
next to it.

```rust
#[proc_macro_attribute]
pub fn no_args(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let func = parse_macro_input!(item as ItemFn);
    if !func.sig.inputs.is_empty() {
        let error = syn::Error::new_spanned(&func.sig.ident, "expected no arguments")
            .to_compile_error();
        return quote! { #error #func }.into(); // diagnostic + the untouched item
    }
    quote!(#func).into()
}
```

`compile_error!` and a normal item coexist happily: the `compile_error!` produces a clean, spanned
diagnostic, and the re-emitted `#func` keeps the function present. Two things are worth knowing
about why this is the right habit:

- **The `compile_error!` is what tames the cascade.** Modern rustc, once it sees your
  `compile_error!`, suppresses the follow-on "cannot find …" errors — so even returning _only_ the
  error reads cleanly in a batch build. The genuinely bad version is the _silent_ drop above, which
  emits no `compile_error!` at all and leaves the user with an unexplained "cannot find function".
- **Re-emitting the item is still best practice.** It keeps the function intact for `rust-analyzer`
  and incremental tooling — which want to see the item even while it's erroneous — and it's robust
  in the cases the cascade _isn't_ suppressed. `quote! { #error #item }` is the idiom to memorize:
  error first, original item second.

This is the attribute-macro counterpart to the spanned `syn::Error` work from chapter 4 — same
`syn::Error`, but now you're careful to keep the item too.

## Exercise

Write `#[describe]`. On a function that takes **no arguments**, it re-emits the function and adds a
companion `fn describe_<name>() -> &'static str` returning the function's name. On a function that
_does_ take arguments, it must report the error **gracefully** — turning the `syn::Error` into a
`compile_error!` (never a panic) and re-emitting the original function alongside it.

The companion-generation and validation are written for you; your job is the error branch. The
`tests/fail` snapshot checks that misuse produces your clean, spanned diagnostic rather than the
`todo!()` panic — `error: custom attribute panicked` — that the stub starts with.
