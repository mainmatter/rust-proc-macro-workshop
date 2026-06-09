# Token and TokenStream

Now that you've used `syn` to parse input conveniently, let's look under the hood at the
types that make it all work.

## What is a token?

When the Rust compiler reads your source code, the first thing it does is **lexing** (also
called tokenization): breaking the source text into a sequence of tokens. Each token is the
smallest meaningful unit of the language.

For example, this Rust code:

```rust
fn add(a: i32) -> i32 { a + 1 }
```

is broken into a tree of tokens:

```text
Ident("fn")
Ident("add")
Group(Parenthesis)
├── Ident("a")
├── Punct(':')
└── Ident("i32")
Punct('-')
Punct('>')
Ident("i32")
Group(Brace)
├── Ident("a")
├── Punct('+')
└── Literal(1)
```

Notice that this isn't a flat list — parenthesized and braced portions form nested **groups**.
Each token also has a **span** (its location in the source file).

## `TokenTree`

The [`proc_macro`](https://doc.rust-lang.org/proc_macro/) crate represents tokens using the
[`TokenTree`](https://doc.rust-lang.org/proc_macro/enum.TokenTree.html) enum:

```rust
pub enum TokenTree {
    Group(Group),     // delimited group: (...), [...], {...}
    Ident(Ident),     // identifier or keyword: foo, struct, pub
    Punct(Punct),     // punctuation: +, :, #
    Literal(Literal), // literal value: 42, "hello", 3.14
}
```

Notice that delimiters (`(`, `)`, `[`, `]`, `{`, `}`) are not individual tokens.
Instead, everything inside a matching pair of delimiters is grouped into a
[`Group`](https://doc.rust-lang.org/proc_macro/struct.Group.html).
This means the token tree is actually a **tree**, not a flat list — groups contain other
token trees.

## `TokenStream`

A [`TokenStream`](https://doc.rust-lang.org/proc_macro/struct.TokenStream.html) is a sequence
of `TokenTree` values. It's the type that every procedural macro receives and returns.

You can iterate over a `TokenStream` to inspect its tokens:

```rust
use proc_macro::TokenStream;

#[proc_macro_derive(Inspect)]
pub fn inspect(input: TokenStream) -> TokenStream {
    for tree in input {
        eprintln!("{tree:?}");
    }
    TokenStream::new()
}
```

We use [`eprintln!`](https://doc.rust-lang.org/std/macro.eprintln.html) rather than
[`println!`](https://doc.rust-lang.org/std/macro.println.html) here. Both format and print
text, but `println!` writes to **stdout** while `eprintln!` writes to **stderr**. Both show up
during compilation, so either works for debugging — but by convention proc macros use
`eprintln!`, keeping debug noise on stderr and separate from anything a build might emit on
stdout. This makes `eprintln!` a handy debugging tool alongside `cargo expand`.

Keep in mind that a proc macro only runs when the crate that invokes it is actually
(re)compiled. If a build is cached, the macro doesn't run and you'll see no output — touch the
source or run `cargo clean` to force a rebuild.

## Descending into groups

Iterating with `for tree in input` only visits the **top-level** tokens. Remember that a
`TokenStream` is a tree: anything inside `(...)`, `[...]`, or `{...}` is wrapped in a
[`Group`](https://doc.rust-lang.org/proc_macro/struct.Group.html), and its contents are a
nested `TokenStream`. For a struct like `struct Pair { x: i32 }`, the top level is just
`Ident("struct")`, `Ident("Pair")`, and a single `Group` — the fields `x` and `i32` live
_inside_ that group.

To reach them, call [`Group::stream()`](https://doc.rust-lang.org/proc_macro/struct.Group.html#method.stream)
to get the inner stream and recurse.

## Building a `TokenStream`

You can build a `TokenStream` from individual `TokenTree` values:

```rust
use proc_macro::{TokenStream, TokenTree, Ident, Span};

let ident = TokenTree::Ident(Ident::new("hello", Span::call_site()));
let stream: TokenStream = [ident].into_iter().collect();
```

[`Ident`](https://doc.rust-lang.org/proc_macro/struct.Ident.html) represents an identifier or
keyword, and [`Span::call_site()`](https://doc.rust-lang.org/proc_macro/struct.Span.html#method.call_site)
gives it the location of the macro invocation.

Or parse it from a string:

```rust
let stream: TokenStream = "fn foo() {}".parse().unwrap();
```

Building token streams by hand is tedious. That's why we'll soon introduce `quote!`, which
makes this much more ergonomic.

## Exercise

Write a derive macro that inspects the token stream it receives and counts the number of
`Ident` tokens in it — including those nested inside groups, which you'll need to recurse into
with `Group::stream()`. The macro should generate a method that returns this count.
