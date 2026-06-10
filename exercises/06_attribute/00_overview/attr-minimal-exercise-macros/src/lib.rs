use proc_macro::TokenStream;
use syn::{ItemFn, parse_macro_input};

/// `#[make_public]` — the simplest possible *transforming* attribute macro: it
/// takes a function and re-emits it with `pub` visibility.
///
/// An attribute macro receives **two** token streams: `attr` (the tokens inside
/// the attribute's parentheses, empty here) and `item` (the annotated item). Unlike
/// a derive macro, which only ever *adds* code, an attribute macro returns the
/// replacement for the item — so whatever we return *is* the function now.
#[proc_macro_attribute]
pub fn make_public(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut func = parse_macro_input!(item as ItemFn);

    func.vis = syn::parse_quote!(pub);

    quote::quote!(#func).into()
}
