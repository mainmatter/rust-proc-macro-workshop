use darling::{FromMeta, ast::NestedMeta};
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{ItemFn, parse_macro_input};

#[derive(FromMeta)]
struct EndpointArgs {
    path: String,
    #[darling(default = "default_method")]
    method: String,
}

fn default_method() -> String {
    "GET".to_string()
}

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

    let meta = match NestedMeta::parse_meta_list(attr.into()) {
        Ok(meta) => meta,
        Err(err) => return TokenStream::from(darling::Error::from(err).write_errors()),
    };
    let args = match EndpointArgs::from_list(&meta) {
        Ok(args) => args,
        Err(err) => return TokenStream::from(err.write_errors()),
    };

    endpoint_impl(&func, &args.path, &args.method).into()
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
