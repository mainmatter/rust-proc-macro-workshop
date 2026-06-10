# Parsing arbitrary token input

A derive macro never has to wonder what its input looks like — `parse_macro_input!(input as DeriveInput)` always works, because the compiler only ever hands it a type definition. A
function-like macro has no such guarantee. The tokens between `foo!( ... )` are whatever the caller
typed, and **you** decide how to interpret them.

`syn` still does the heavy lifting; you just have to tell it _what_ to parse. For input that's a
list of ordinary Rust fragments, the tools you already know are enough.

## A worked example: `min!`

Let's build `min!(a, b, c, ...)` — a macro that expands to the minimum of its arguments, where each
argument is an arbitrary expression:

```rust
let smallest = min!(3 + 4, x, y, 0);
```

The input is a comma-separated list of expressions. `syn` models exactly that with
[`Punctuated<T, P>`](https://docs.rs/syn/latest/syn/punctuated/struct.Punctuated.html) — a sequence
of `T` separated by the punctuation token `P`:

```rust
use proc_macro::TokenStream;
use quote::quote;
use syn::{Expr, Token, parse_macro_input, punctuated::Punctuated};

#[proc_macro]
pub fn min(input: TokenStream) -> TokenStream {
    // Parse the input as a comma-separated list of expressions.
    let args = parse_macro_input!(input with Punctuated::<Expr, Token![,]>::parse_terminated);

    let mut iter = args.into_iter();
    let first = iter.next().expect("min! needs at least one argument");
    let rest = iter; // the remaining expressions

    quote! {
        {
            let mut __min = #first;
            #( __min = ::core::cmp::min(__min, #rest); )*
            __min
        }
    }
    .into()
}
```

Two new pieces compared to a derive macro:

- **`parse_macro_input!(input with <parser>)`.** The `as Type` form needs a type that implements
  [`Parse`](https://docs.rs/syn/latest/syn/parse/trait.Parse.html). When you instead want to call a
  specific _parser function_, use the `with` form.
  [`Punctuated::parse_terminated`](https://docs.rs/syn/latest/syn/punctuated/struct.Punctuated.html#method.parse_terminated)
  parses a list with an optional trailing comma — so `min!(a, b,)` is accepted, just like Rust
  accepts trailing commas everywhere else. (`Token![,]` is `syn`'s way of naming the `,` token
  type; there's a `Token![...]` for every bit of Rust punctuation and every keyword.)
- **`#( ... )*` over a runtime iterator.** You met `quote!`'s repetition in
  [chapter 2](../02_basics/08_quote.md#repetition-with---) when expanding struct fields. It works on anything iterable, including the `rest` iterator here:
  `quote!` advances it once per repetition and splices each expression in turn.

That's the whole macro. Note that `min!` _generates_ a chain of `min` calls; it doesn't compute
anything itself. The actual comparisons happen at run time, on whatever values the expressions
evaluate to — the macro only assembles the code. (The `.expect(..)` on empty input is a shortcut; a
production macro would report that with a `syn::Error`, as you did in chapter 3; the exercise's
`avg!` already has that error wired up for you.)

## Generating code that uses what you parsed

A subtle but important point: the generated code can depend not just on the _expressions_ you
parsed but on facts _about_ them — how many there are, their positions, and so on. The number of
arguments is known at compile time, so you can bake it straight into the output as a literal.
That's the new muscle the exercise asks you to flex.

## Exercise

Build a sibling of `min!`: a macro `avg!(a, b, c, ...)` that expands to the **average** of its
comma-separated arguments, as an `f64`. Parse the arguments exactly as `min!` does, then generate
code that sums them and divides by the count.

The twist over `min!` is that you need the _number_ of arguments in your generated code — it's the
divisor. `Punctuated` is a collection, so its length is a method call away; emit that count as part
of the output. Make `tests/pass/basic.rs` pass. (The empty-input case, `avg!()`, is already handled
for you with a `compile_error!` and checked by `tests/fail/empty.rs`.)
