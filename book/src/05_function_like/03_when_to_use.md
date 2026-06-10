# When to use function-like over declarative macros

There are two ways to write a macro that's _invoked_ like `foo!( ... )`:

- a **declarative** macro, written with `macro_rules!`, and
- a **procedural** function-like macro, the `#[proc_macro]` kind you've been building.

They're called identically, so from the caller's side you often can't tell which one you're using —
`vec!` is declarative, [`sqlx::query!`](https://docs.rs/sqlx/latest/sqlx/macro.query.html) is
procedural, both look the same. The question for _you_, the
author, is which to write. Procedural macros are far more powerful, but that power isn't free, so
the honest default is: **reach for `macro_rules!` first, and only escalate when it can't do the
job.**

## What `macro_rules!` is good at

`macro_rules!` is a pattern-matching templating engine. It matches the input against
_fragment specifiers_ — `$e:expr`, `$t:ty`, `$i:ident`, `$($xs:expr),*` for repetition — and
expands a fixed template. For anything that's "take some Rust fragments and stamp out a fixed shape
of code," it's the right tool, and it needs no extra crate, no `syn`, no compile step of its own:

```rust
macro_rules! my_vec {
    ($($x:expr),* $(,)?) => {{
        let mut v = Vec::new();
        $( v.push($x); )*
        v
    }};
}
```

That's the whole comma-separated-list family — `vec!`, `min!`, `hashset!` — expressions in, a
template out. If `macro_rules!` can express it, prefer it.

## What only a procedural macro can do

You _need_ `#[proc_macro]` when the input or the work exceeds simple templating:

- **Syntax that isn't Rust fragments.** `macro_rules!` can only match things that fit a fragment
  specifier. The moment your DSL contains tokens that aren't valid Rust expressions/types/etc. —
  `<div>` in `html!`, `GET "/" => handler` in a router — `macro_rules!` can't describe it, but a
  hand-written `Parse` impl can read any token at all.
- **Inspecting the input as data.** Counting arguments and emitting the count, validating a format
  or SQL string against a real schema, deriving identifiers from the input — that's arbitrary Rust
  code running at compile time. `macro_rules!` only rearranges tokens; it can't _compute_ over them.
- **Real diagnostics.** A procedural macro can emit a `syn::Error` spanned at the exact offending
  token (you did this in chapter 4). `macro_rules!` errors are notoriously hard to aim.

The cost is real: a separate `proc-macro` crate, a parser to write and maintain, and slower
compiles. So the rule of thumb is a ladder — climb only as high as you have to:

- **A plain function** if there's no code generation at all.
- **`macro_rules!`** if it's token templating.
- **A procedural macro** only when you genuinely need to parse custom syntax or compute over the
  input.

## Exercise

No macro to write this time — just judgement calls. For each scenario in the exercise, decide
whether a **procedural function-like macro** is warranted, or whether `macro_rules!` (or even a
plain function) would do. The test messages explain the reasoning for each.
