use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Ident, LitStr, Token,
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
};

/// One route of the DSL: `GET "/path" => handler`.
struct Route {
    method: Ident,
    path: LitStr,
    handler: Ident,
}

impl Parse for Route {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // TODO: parse one route, `METHOD "path" => handler`, by pulling its parts off
        //   the stream in grammar order:
        //     1. the method as an `Ident` (e.g. `GET`) — `input.parse()?`,
        //     2. the path as a `LitStr`,
        //     3. the `=>` arrow with `input.parse::<Token![=>]>()?` (this consumes it;
        //        a missing arrow is reported automatically — that's the compile error
        //        `tests/fail/missing_arrow.rs` checks for),
        //     4. the handler as an `Ident`,
        //   then build the `Route`. Section 02's `Entry` `Parse` impl shows the same
        //   read-tokens-in-order shape.
        let _ = input;
        todo!()
    }
}

/// The whole `routes! { ... }` block: a comma-separated list of routes.
struct Routes {
    routes: Punctuated<Route, Token![,]>,
}

impl Parse for Routes {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Routes {
            routes: Punctuated::parse_terminated(input)?,
        })
    }
}

/// A tiny routing DSL:
///
/// ```ignore
/// let router = routes! {
///     GET  "/"      => index,
///     POST "/users" => create_user,
/// };
/// router("GET", "/"); // -> Some(index())
/// ```
///
/// It expands to a closure `|method: &str, path: &str| -> Option<String>` that
/// dispatches to the first matching handler (a `fn() -> String`), or returns `None`.
#[proc_macro]
pub fn routes(input: TokenStream) -> TokenStream {
    let Routes { routes } = parse_macro_input!(input as Routes);
    routes_impl(&routes).into()
}

fn routes_impl(routes: &Punctuated<Route, Token![,]>) -> proc_macro2::TokenStream {
    // TODO: generate the router closure — the heart of the macro. The book's
    //   "Generating the dispatcher" section shows the closure skeleton to aim for.
    //   For each parsed `Route`, emit an arm that returns the handler's result wrapped
    //   in `Some(...)` when the runtime method and path both match that route's method
    //   (an `Ident` — you'll need it as a string) and its path (a `LitStr`). Map over
    //   `routes.iter()` to build the arms, splice them into the closure body with
    //   `quote!`'s `#( ... )*` repetition, and fall through to `None`.
    //   Reach for absolute paths (chapter 3) so the output compiles in any context.
    let _ = routes;
    todo!()
}
