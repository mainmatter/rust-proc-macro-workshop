use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_macro_input};

/// A derive macro for enums that generates an `ordinal(&self) -> usize` method,
/// returning the 0-based position of the current variant in declaration order:
/// the first variant is `0`, the second `1`, and so on.
///
/// Like the book's `as_str` example, it builds one match arm per variant — but
/// here the arm body is the variant's *index*, not its name, so you'll need
/// `enumerate`. You still have to ignore each variant's payload, which means
/// matching its shape (unit / tuple / struct variant).
///
/// Panics if used on a struct or union.
#[proc_macro_derive(Ordinal)]
pub fn ordinal(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    ordinal_impl(&input).into()
}

fn ordinal_impl(input: &DeriveInput) -> proc_macro2::TokenStream {
    let name = &input.ident;

    let Data::Enum(data) = &input.data else {
        panic!("Ordinal only supports enums");
    };

    // One match arm per variant: `<pattern> => <index>,`.
    let arms =
        data.variants
            .iter()
            .enumerate()
            .map(|(index, variant)| -> proc_macro2::TokenStream {
                let vname = &variant.ident;

                let pattern = match &variant.fields {
                    Fields::Unit => quote! { #name::#vname },
                    Fields::Unnamed(_) => quote! { #name::#vname(..) },
                    Fields::Named(_) => quote! { #name::#vname { .. } },
                };
                quote! { #pattern => #index, }
            });

    quote! {
        #[automatically_derived]
        impl #name {
            pub fn ordinal(&self) -> usize {
                match self {
                    #(#arms)*
                }
            }
        }
    }
}
