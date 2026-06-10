# Spans for better error messages

A `syn::Error` carries a **span** — a pointer into the user's source code that tells the
compiler where to draw the `^^^^` underline. Choosing a good span is the difference between an
error the user can act on immediately and one that makes them hunt.

## What a span is

A [`Span`](https://docs.rs/proc-macro2/latest/proc_macro2/struct.Span.html) is a region of
source text. Every token `syn` parses remembers the span it came from, so when you have a piece
of the parsed input — a `Field`, an `Ident`, a `Type` — you also have the location where the
user wrote it. `Error::new_spanned(thing, message)` borrows that location.

## Point at the smallest relevant thing

Compare two ways of rejecting a struct that has a `()`-typed field:

```rust
// Span = the whole struct. Underlines everything; the user has to find the bad field.
return Err(syn::Error::new_spanned(input, "fields can't be `()`"));
```

```text
error: fields can't be `()`
 --> src/main.rs:3:1
  |
3 | / struct Config {
4 | |     name: String,
5 | |     marker: (),
6 | | }
  | |_^
```

```rust
// Span = just the offending field's type. Underlines exactly what's wrong.
return Err(syn::Error::new_spanned(&field.ty, "fields can't be `()`"));
```

```text
error: fields can't be `()`
 --> src/main.rs:5:13
  |
5 |     marker: (),
  |             ^^
```

The second one points the user straight at `marker`'s type. As a rule of thumb: **span the
error on the most specific token that's actually at fault** — a field's `ty` or `ident`, a
variant, an attribute — rather than the whole `DeriveInput`. Whole-item spans are right only
when the whole item is the problem (e.g. "this derive doesn't support enums").

## Where does a good span come from?

You almost never construct a `Span` by hand. Instead you reach for the relevant parsed node and
let `new_spanned` take its span:

- a bad field → `new_spanned(&field.ty, ..)` or `new_spanned(&field.ident, ..)`
- a bad variant → `new_spanned(&variant, ..)`
- a bad attribute → `new_spanned(&attr, ..)`

It's also good practice to put the offending item's name into the message text.

> `Span::call_site()` resolves to wherever the macro was invoked — the `#[derive(...)]` site —
> rather than to any particular token in the input. It's the right span for identifiers your macro
> _invents_ (it's what you pass to `Ident::new`), but the wrong one for an _error_: spanned that
> way, the error underlines the whole derive instead of the offending field, so it doesn't tell the
> user where to look. Span errors on a real input node with `new_spanned` instead.

## Exercise

The `Getters` macro now rejects fields of the unit type `()`. The per-field check is wired up;
your job is to return the error and span it so the compiler underlines just the offending field.
A shipped compile-fail snapshot, `tests/fail/unit_field.stderr`, is the spec: it pins both the
message and the span, so getting the span wrong — pointing at the whole struct, say — fails the
test even when the message is right. (A plain unit test can't catch that: it never sees where the
`^^^^` lands. Authoring these snapshots yourself is the next section.)
