use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_macro_input};

/// A derive macro that generates a `field_names()` method returning the names of
/// all fields in a named struct as a `&'static [&'static str]`.
///
/// The current implementation uses string formatting. Your task is to rewrite
/// `field_names_impl` to use `quote!` instead.
///
/// Hint: look at the `quote!` repetition syntax covered in the book section.
#[proc_macro_derive(FieldNames)]
pub fn field_names(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    field_names_impl(&input).into()
}

fn field_names_impl(input: &DeriveInput) -> proc_macro2::TokenStream {
    let name = &input.ident;

    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => panic!("FieldNames only supports named structs"),
        },
        _ => panic!("FieldNames only supports structs"),
    };

    // TODO: Rewrite this function to use `quote!` instead of `format!`.
    let field_names_str = fields
        .iter()
        .map(|f| format!("\"{}\"", f.ident.as_ref().unwrap()))
        .collect::<Vec<_>>()
        .join(", ");

    format!(
        "impl {name} {{
            pub fn field_names() -> &'static [&'static str] {{
                &[{field_names_str}]
            }}
        }}"
    )
    .parse()
    .unwrap()
}
