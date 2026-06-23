use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_macro_input};

/// **Exercise — refactor.** This is `Getters` exactly as you left it in the
/// previous section: misuse is reported with a hand-written `compile_error!`,
/// and `getters_impl` returns a plain `TokenStream`.
///
/// That style doesn't scale — every error site needs its own `quote!`, and you
/// can't use `?` to bubble a failure up out of a helper. Refactor it to the
/// `syn::Error` / `syn::Result` approach this section's book page walks through,
/// keeping the user-visible behaviour identical:
///
/// - make `getters_impl` return a `syn::Result<_>`;
/// - turn each `compile_error!` return into an `Err` carrying a `syn::Error`
///   (built the way the book shows, anchored on the right span);
/// - wrap the success path in `Ok`;
/// - in `getters` (below), convert a returned error back into tokens.
///
/// The unit tests call `getters_impl(..).unwrap_err()` / `.is_ok()`, so they only
/// compile once `getters_impl` returns a `Result` — that's your signal you've
/// threaded the new type through everything.
#[proc_macro_derive(Getters)]
pub fn getters(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    getters_impl(&input).into()
}

fn getters_impl(input: &DeriveInput) -> proc_macro2::TokenStream {
    let name = &input.ident;

    let Data::Struct(data) = &input.data else {
        return quote! {
            compile_error!("Getters can only be derived for structs");
        };
    };

    let Fields::Named(fields) = &data.fields else {
        return quote! {
            compile_error!("Getters can only be derived for structs with named fields");
        };
    };

    let getters = fields.named.iter().map(|f| {
        let fname = f.ident.as_ref().unwrap();
        let ty = &f.ty;
        quote! {
            pub fn #fname(&self) -> &#ty {
                &self.#fname
            }
        }
    });

    quote! {
        #[automatically_derived]
        impl #name {
            #(#getters)*
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;

    #[test]
    fn rejects_enums_with_an_error() {
        let input: DeriveInput = parse_quote! { enum E { A, B } };
        let err = getters_impl(&input).unwrap_err();
        assert!(
            err.to_string()
                .contains("Getters can only be derived for structs"),
            "unexpected message: {err}"
        );
    }

    #[test]
    fn accepts_named_structs() {
        let input: DeriveInput = parse_quote! { struct S { x: i32 } };
        assert!(getters_impl(&input).is_ok());
    }
}
