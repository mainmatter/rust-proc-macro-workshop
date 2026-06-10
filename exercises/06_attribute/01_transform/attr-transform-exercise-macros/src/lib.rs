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
    let mut func = parse_macro_input!(item as ItemFn);

    let block = &func.block;
    let new_block: syn::Block = syn::parse_quote! {{
        let result: String = #block;
        result.trim().to_string()
    }};
    func.block = Box::new(new_block);

    quote::quote!(#func).into()
}
