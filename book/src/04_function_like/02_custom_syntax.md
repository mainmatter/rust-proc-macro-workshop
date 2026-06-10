# Custom syntax

So far the tokens inside your macro have been ordinary Rust — a list of expressions. The real
reason to reach for a function-like macro is that the input _doesn't have to be valid Rust at all_.
You can invent your own little language and parse it however you like. This is how
[`html!`](https://docs.rs/maud/latest/maud/macro.html.html) accepts `<div>` tags and
the declarative [`quote!`](https://docs.rs/quote/latest/quote/macro.quote.html) accepts
`#interpolation` — syntax the Rust grammar has never heard of.

To parse custom syntax you implement [`Parse`](https://docs.rs/syn/latest/syn/parse/trait.Parse.html)
for your own types. A `Parse` impl is handed a
[`ParseStream`](https://docs.rs/syn/latest/syn/parse/type.ParseStream.html) — a cursor over the
remaining tokens — and pulls items off it one at a time, in the order your grammar dictates.

## A worked example: `hashmap!`

Let's parse a small DSL: a map literal whose entries are written `key => value`.

```rust
let m = hashmap! {
    "one" => 1,
    "two" => 1 + 1,
};
```

First, a type for a single entry, with a hand-written `Parse` impl that spells out the grammar
"an expression, then `=>`, then another expression". Both the key and the value are parsed as
`Expr` — not just string literals — so any expression works on either side, exactly like the
keys and values a real [`HashMap`](https://doc.rust-lang.org/std/collections/struct.HashMap.html)
accepts:

```rust
use syn::{
    Expr, Token,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
};

struct Entry {
    key: Expr,
    value: Expr,
}

impl Parse for Entry {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let key: Expr = input.parse()?;     // parse the key expression
        input.parse::<Token![=>]>()?;       // expect and consume `=>`
        let value: Expr = input.parse()?;   // parse the value expression
        Ok(Entry { key, value })
    }
}
```

`input.parse::<T>()` parses one `T` and advances the cursor; `Token![=>]` names the `=>` token
type. If the `=>` isn't there, `parse` returns an `Err` and `syn` turns it into a compiler error
pointing at the offending token — you get good diagnostics for free.

The whole macro is then a comma-separated list of `Entry`, parsed with the same
`parse_terminated` you saw for `min!` (it works for any `T: Parse`, not just `Expr`):

```rust
struct Map {
    entries: Punctuated<Entry, Token![,]>,
}

impl Parse for Map {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Map { entries: Punctuated::parse_terminated(input)? })
    }
}

#[proc_macro]
pub fn hashmap(input: TokenStream) -> TokenStream {
    let Map { entries } = parse_macro_input!(input as Map);

    let inserts = entries.iter().map(|Entry { key, value }| {
        quote! { __map.insert(#key, #value); }
    });

    quote! {
        {
            let mut __map = ::std::collections::HashMap::new();
            #( #inserts )*
            __map
        }
    }
    .into()
}
```

## Custom keywords

`Entry` reused Rust's own `Expr` and `=>`. Often your DSL needs words that _look_ like keywords but
aren't real ones — `asc`, `desc`, `state`, `on`. `syn` won't recognise them out of the box, so you
declare them with [`custom_keyword!`](https://docs.rs/syn/latest/syn/macro.custom_keyword.html),
conventionally inside a `mod kw`. Say you're parsing a sort direction:

```rust
mod kw {
    syn::custom_keyword!(asc);
    syn::custom_keyword!(desc);
}
```

Each call generates a token _type_ (`kw::asc`) you can parse and, crucially, **peek**. Peeking lets
you look at the next token _without consuming it_, so you can branch on what comes next:

```rust
// inside a `Parse` impl, deciding which keyword (if any) comes next:
let direction = if input.peek(kw::asc) {
    input.parse::<kw::asc>()?; // consume it now that we know it's there
    "ASC"
} else if input.peek(kw::desc) {
    input.parse::<kw::desc>()?;
    "DESC"
} else {
    // Nothing we recognise — produce a clear error at the current position.
    return Err(input.error("expected `asc` or `desc`"));
};
```

`input.error(msg)` builds a `syn::Error` spanned at the cursor; returning it makes misuse a
compile error that points exactly where parsing got stuck. This is the parsing-side equivalent of
the error reporting you did with `syn::Error` in chapter 3.

## Exercise

Build a `methods!` macro for declaring HTTP endpoints:

```rust
let routes = methods! {
    get "/",
    post "/users",
};
// -> vec![("GET", "/"), ("POST", "/users")]
```

Each entry starts with a custom keyword — `get` or `post` — followed by a string-literal path. The
macro expands to a `Vec<(&'static str, &'static str)>` of `(method, path)` pairs. The list
parsing, the keyword declarations, and the code generation are written for you; your job is the
`Parse` impl for a single entry: peek for `get`/`post`, consume the right keyword, parse the path,
and reject anything else with a helpful error. The `tests/fail` case checks that a bad method is a
clean compile error, so don't forget the `else` branch.
