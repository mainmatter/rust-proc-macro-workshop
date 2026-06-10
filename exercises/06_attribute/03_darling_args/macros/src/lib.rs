use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{ItemFn, parse_macro_input};

/// `#[endpoint(path = "/users", method = "POST")]` — attaches routing metadata to a
/// function. `path` is required; `method` is optional and defaults to `"GET"`. The
/// macro re-emits the function and generates `<name>_path()` / `<name>_method()`
/// accessors returning the parsed values.
///
/// The new skill here is reading the **attribute arguments** — the tokens inside the
/// parentheses, which arrive in the first `TokenStream`. Parse them declaratively
/// with `darling`'s `FromMeta`, the attribute-macro counterpart to the
/// `FromDeriveInput` you used in chapter 4.
#[proc_macro_attribute]
pub fn endpoint(attr: TokenStream, item: TokenStream) -> TokenStream {
    let func = parse_macro_input!(item as ItemFn);

    // TODO: parse the `#[endpoint(...)]` arguments with darling's `FromMeta`.
    //   1. Declare a struct that derives `darling::FromMeta` with:
    //        - a required `path: String`;
    //        - an optional `method: String` that defaults to "GET" — use
    //          `#[darling(default = "...")]` pointing at a helper fn, just like the
    //          `#[darling(default = ...)]` you used in chapter 4's darling section.
    //   2. Parse it out of `attr` (you'll want
    //      `use darling::{FromMeta, ast::NestedMeta};`):
    //        - `NestedMeta::parse_meta_list(attr.into())` splits the tokens into a list
    //          of nested-meta items. It fails with a `syn::Error`, so convert that with
    //          `darling::Error::from(..)`.
    //        - `YourArgs::from_list(&meta)` then parses your struct and fails with a
    //          `darling::Error`.
    //      On either failure, return `err.write_errors()` as the output `TokenStream`
    //      (that's how darling renders attribute mistakes as compiler errors).
    //   Replace these two placeholders with the values parsed from `attr`.
    let _ = &attr;
    let path: String = String::new();
    let method: String = String::new();

    endpoint_impl(&func, &path, &method).into()
}

fn endpoint_impl(func: &ItemFn, path: &str, method: &str) -> proc_macro2::TokenStream {
    let name = &func.sig.ident;
    let path_fn = format_ident!("{}_path", name);
    let method_fn = format_ident!("{}_method", name);

    quote! {
        #func

        fn #path_fn() -> &'static str {
            #path
        }

        fn #method_fn() -> &'static str {
            #method
        }
    }
}
