# Error reporting: panic vs `compile_error!`

Every macro you've written so far reaches for `panic!` when the input is wrong — deriving on an
enum, a tuple struct, and so on. That's fine while you're prototyping, but it's a poor
experience for the people _using_ your macro. This section is the first of four on doing better.

## What the user sees when you panic

When a proc macro panics, the compiler catches the panic and turns it into an error like this:

```text
error: proc-macro derive panicked
 --> src/main.rs:3:10
  |
3 | #[derive(Getters)]
  |          ^^^^^^^
  |
  = help: message: Getters can only be derived for structs with named fields
```

The message is there, but it's buried under "proc-macro derive panicked," it's tagged as a
_help_ note, and the span points only at the derive name. It reads like your macro crashed —
because, technically, it did.

## `compile_error!` — a real diagnostic

[`compile_error!`](https://doc.rust-lang.org/std/macro.compile_error.html) is a built-in macro
that emits a compiler error with your message when it's expanded. Instead of _panicking_, your
macro can _return_ a token stream that invokes it:

```rust
return quote! {
    compile_error!("Getters can only be derived for structs with named fields");
};
```

Now the user sees a clean, top-level error:

```text
error: Getters can only be derived for structs with named fields
 --> src/main.rs:3:10
  |
3 | #[derive(Getters)]
  |          ^^^^^^^
```

No "panicked," no "help: message:" — just an error, exactly like one the compiler itself would
produce. You can even emit a `compile_error!` _alongside_ valid code, so the rest of your
expansion still type-checks and you avoid a cascade of follow-on errors. (More on that idea when
we cover attribute macros.)

## Why not stop here?

`compile_error!` is a big step up from a panic, but building the message by hand with `quote!`
gets awkward as soon as you have more than one error site, and the span still points at the
whole derive rather than at the _specific_ thing that's wrong. The next two sections introduce
[`syn::Error`](https://docs.rs/syn/latest/syn/struct.Error.html), which packages a message
together with a span and converts to a `compile_error!` for you — and lets you point the error
exactly where it belongs.

## Exercise

The `Getters` derive macro currently `panic!`s when used on anything other than a named struct.
Replace both panics with returned `compile_error!` invocations. The unit tests in the macro
crate check that misuse produces a `compile_error!` rather than a panic.
