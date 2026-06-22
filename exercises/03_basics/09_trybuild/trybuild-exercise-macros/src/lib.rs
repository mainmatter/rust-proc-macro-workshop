use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_macro_input};

/// A derive macro that generates a `field_names()` method returning the names of
/// all fields in a named struct as a `&'static [&'static str]`.
///
/// Panics if used on an enum, union, tuple struct, or unit struct.
#[proc_macro_derive(FieldNames)]
pub fn field_names(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    field_names_impl(&input).into()
}

fn field_names_impl(input: &DeriveInput) -> proc_macro2::TokenStream {
    let name = &input.ident;
    let Data::Struct(data) = &input.data else {
        panic!("FieldNames only supports structs");
    };

    let Fields::Named(fields) = &data.fields else {
        panic!("FieldNames only supports named structs");
    };

    let field_strings: Vec<String> = fields
        .named
        .iter()
        .map(|f| f.ident.as_ref().unwrap().to_string())
        .collect();

    quote! {
        impl #name {
            pub fn field_names() -> &'static [&'static str] {
                &[#(#field_strings),*]
            }
        }
    }
}
