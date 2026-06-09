use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Type, parse_macro_input};

/// A complete `Getters` macro, carrying all the error handling from the previous
/// sections: whole-item errors for unsupported shapes, and a per-field error
/// (spanned on the field) for `()`-typed fields.
///
/// You won't change this file — the exercise is to *test* the errors it produces
/// with `trybuild`.
#[proc_macro_derive(Getters)]
pub fn getters(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    getters_impl(&input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

fn getters_impl(input: &DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let name = &input.ident;

    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => {
                return Err(syn::Error::new_spanned(
                    input,
                    "Getters can only be derived for structs with named fields",
                ));
            }
        },
        _ => {
            return Err(syn::Error::new_spanned(
                input,
                "Getters can only be derived for structs",
            ));
        }
    };

    for field in fields {
        if is_unit_type(&field.ty) {
            let fname = field.ident.as_ref().unwrap();
            return Err(syn::Error::new_spanned(
                &field.ty,
                format!("Getters can't generate a getter for the `()`-typed field `{fname}`"),
            ));
        }
    }

    let getters = fields.iter().map(|f| {
        let fname = f.ident.as_ref().unwrap();
        let ty = &f.ty;
        quote! {
            pub fn #fname(&self) -> &#ty {
                &self.#fname
            }
        }
    });

    Ok(quote! {
        impl #name {
            #(#getters)*
        }
    })
}

fn is_unit_type(ty: &Type) -> bool {
    matches!(ty, Type::Tuple(t) if t.elems.is_empty())
}
