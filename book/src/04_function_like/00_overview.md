# Function-like macros

The derive macros you've built so far had it easy: the compiler always handed your macro the
same thing — the annotated `struct` or `enum`, parsed into a `DeriveInput`. The shape of the input
never varied — it was always a type definition — and your job was to generate _extra_ code alongside
it.

Function-like macros lift that restriction. They look like a function call with a `!`:

```rust
let v = vec![1, 2, 3];
let s = format!("{name} is {age}");
let q = sqlx::query!("SELECT * FROM users WHERE id = $1", id);
```

"Function-like" describes the _call syntax_, not how the macro is implemented: `vec!` and
`format!` are declarative (`macro_rules!`), while
[`sqlx::query!`](https://docs.rs/sqlx/latest/sqlx/macro.query.html) is procedural — the
`#[proc_macro]` kind this chapter builds. [Section `03`](03_when_to_use.md) draws that line; for now
just note they all share the `name!( ... )` shape.

That last one even checks its SQL against your database at compile time. Function-like macros can
receive **any tokens at all** between their delimiters — a comma-separated list, a SQL string, even
syntax that isn't valid Rust. Whatever you write inside `name! { ... }` is handed to your macro as a
raw `TokenStream`, and it's entirely up to you what it means.

## Why is `println!` a macro?

It's a fair question — why isn't `println` just a function? Two reasons, and both are things a
plain function fundamentally _cannot_ do:

- **It's variadic over mixed types.** `println!("{} {}", 1, "two")` takes a different number of
  arguments of different types every time it's called. Rust functions have a fixed arity and fixed
  parameter types; there's no way to write `fn println(fmt, ...)` like in C.
- **It checks the format string at compile time.** `println!("{}")` is a _compile error_ —
  "1 positional argument in format string, but no arguments were given" — not a runtime panic. To
  produce that error the macro has to parse the literal `"{}"` while the program is being compiled
  and count its placeholders. A function only sees the string's _value_ at run time, far too late.

Anything that needs a variable shape of input, or needs to inspect its arguments _as source code_,
has to be a macro. `println!` is both.

## Registering a function-like macro

You already know the crate layout from chapter 2 — a `proc-macro` crate, usually re-exported
through a facade crate. The only thing that changes is the attribute on your function:

```rust
use proc_macro::TokenStream;

#[proc_macro] // not #[proc_macro_derive(...)] and not #[proc_macro_attribute]
pub fn my_macro(input: TokenStream) -> TokenStream {
    // `input` is whatever tokens the caller wrote inside `my_macro!( ... )`
    todo!()
}
```

Three kinds of procedural macro, three attributes:

| Attribute                   | Invoked as       | Input it receives                      |
| --------------------------- | ---------------- | -------------------------------------- |
| `#[proc_macro_derive(Foo)]` | `#[derive(Foo)]` | the annotated type, as a `DeriveInput` |
| `#[proc_macro]`             | `foo!( ... )`    | the raw tokens between the delimiters  |
| `#[proc_macro_attribute]`   | `#[foo] item`    | the attribute's args _and_ the item    |

A function-like macro can be invoked with any of the three delimiters — `foo!(...)`, `foo!{...}`,
or `foo![...]` — and they're interchangeable; the macro can't tell which you used (`vec![1, 2, 3]`
and `vec! {1, 2, 3}` are the same call). By convention `!{ }` is used for block-like or
statement-like DSLs and `!( )` for expression-like calls.

The output is the same `TokenStream` you've been returning all along, and it's spliced in wherever
the macro was called. That means a function-like macro can expand into an _expression_
(`let x = foo!();`) or, in item position, a set of _items_ (`foo!();` at module level) — depending
on where it's invoked and what tokens it returns. (To run several statements in expression position,
return a block expression `{ ... }` — you'll do exactly that in the next section.)

## Exercise

A warm-up from the caller's side. Use
[`serde_json::json!`](https://docs.rs/serde_json/latest/serde_json/macro.json.html) — a
function-like macro that turns a JSON-shaped block of tokens into a
[`serde_json::Value`](https://docs.rs/serde_json/latest/serde_json/enum.Value.html) — to build
the document the tests expect. You're _using_ a function-like macro here; from the next section
onwards you'll build your own.
