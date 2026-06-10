use proc_macro::TokenStream;
use quote::quote;
use syn::{
    LitStr, Token,
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
};

// Custom keywords. `get` and `post` aren't real Rust keywords, so we teach `syn`
// to recognise them as tokens. Each one becomes a type we can `peek` and `parse`.
mod kw {
    syn::custom_keyword!(get);
    syn::custom_keyword!(post);
}

/// One entry of the DSL, e.g. `get "/users"`.
struct Endpoint {
    method: &'static str,
    path: LitStr,
}

impl Parse for Endpoint {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // TODO: parse one endpoint, e.g. `get "/users"`.
        //   1. Decide the HTTP method by *peeking* for a custom keyword:
        //      `input.peek(kw::get)` / `input.peek(kw::post)`. Consume whichever you
        //      matched with `input.parse::<kw::get>()?` and record "GET" / "POST".
        //      If neither keyword comes next, return an error with
        //      `input.error("expected `get` or `post`")` — the
        //      `tests/fail/unknown_method.rs` snapshot checks for exactly that message.
        //   2. Parse the path that follows as a `LitStr`.
        //   The book's custom-keyword snippet shows the peek-then-parse pattern.
        let method = if input.peek(kw::get) {
            input.parse::<kw::get>()?;
            "GET"
        } else if input.peek(kw::post) {
            input.parse::<kw::post>()?;
            "POST"
        } else {
            return Err(input.error("expected `get` or `post`"));
        };

        let path: LitStr = input.parse()?;

        Ok(Endpoint { method, path })
    }
}

/// The whole `methods! { ... }` block: a comma-separated list of endpoints.
struct Endpoints {
    entries: Punctuated<Endpoint, Token![,]>,
}

impl Parse for Endpoints {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Endpoints {
            entries: Punctuated::parse_terminated(input)?,
        })
    }
}

/// `methods! { get "/", post "/users" }` expands to a
/// `Vec<(&'static str, &'static str)>` of `(method, path)` pairs.
#[proc_macro]
pub fn methods(input: TokenStream) -> TokenStream {
    let Endpoints { entries } = parse_macro_input!(input as Endpoints);
    let methods = entries.iter().map(|e| e.method);
    let paths = entries.iter().map(|e| &e.path);
    quote! {
        ::std::vec![ #( (#methods, #paths) ),* ]
    }
    .into()
}
