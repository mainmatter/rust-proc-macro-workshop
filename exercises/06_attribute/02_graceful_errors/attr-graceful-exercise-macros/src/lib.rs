use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{ItemFn, parse_macro_input};

/// `#[describe]` — only valid on a function that takes **no** arguments. For
/// `fn ping() { .. }` it re-emits the function and adds a companion
/// `fn describe_ping() -> &'static str` returning the function's name.
///
/// The interesting part isn't the happy path — it's the error path. An attribute
/// macro replaces the item it annotates, so a careless error path (a panic, or
/// dropping the item with no diagnostic) produces baffling errors. The habit to
/// build: report the error as a `compile_error!` *and* re-emit the original item.
#[proc_macro_attribute]
pub fn describe(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let func = parse_macro_input!(item as ItemFn);

    match describe_impl(&func) {
        Ok(tokens) => tokens.into(),
        Err(err) => {
            let compile_error = err.to_compile_error();
            quote! {
                #compile_error
                #func
            }
            .into()
        }
    }
}

fn describe_impl(func: &ItemFn) -> syn::Result<proc_macro2::TokenStream> {
    if !func.sig.inputs.is_empty() {
        return Err(syn::Error::new_spanned(
            &func.sig.ident,
            "#[describe] only works on functions that take no arguments",
        ));
    }

    let name = &func.sig.ident;
    let describe_fn = format_ident!("describe_{}", name);
    let name_str = name.to_string();

    Ok(quote! {
        #func

        fn #describe_fn() -> &'static str {
            #name_str
        }
    })
}
