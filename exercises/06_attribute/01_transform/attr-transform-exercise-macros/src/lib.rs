use proc_macro::TokenStream;
use syn::{ItemFn, parse_macro_input};

/// `#[trimmed]` — wraps a `String`-returning function so the value it returns is
/// trimmed of surrounding whitespace.
///
/// The technique is the one you'll use again and again with attribute macros:
/// keep the function's signature, but replace its **body** with a new block that
/// runs the original code and post-processes the result.
#[proc_macro_attribute]
pub fn trimmed(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let func = parse_macro_input!(item as ItemFn);

    // TODO: replace the function's body so the `String` it returns comes back trimmed.
    //   - the original body is `func.block` (a `syn::Block`, i.e. `{ ... }`).
    //   - build a NEW block that runs the old one, binds its `String` result, and
    //     returns `result.trim()` as an owned `String`. `syn::parse_quote! {{ ... }}`
    //     builds a `syn::Block` from `quote`-style tokens, and you can interpolate the
    //     original body with `#block` (the braces come along, so `let r = #block;`
    //     runs it as a block expression).
    //   - assign your new block to `func.block` (it's a `Box<syn::Block>`), then
    //     re-emit the function with `quote::quote!(#func)`. (You'll need `func` to be
    //     `mut`.)
    //   See the book's `#[timed]` example for the wrap-the-body shape.
    let _ = &func;
    todo!()
}
