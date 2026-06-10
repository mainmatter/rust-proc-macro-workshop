use proc_macro::TokenStream;
use syn::{Data, DeriveInput, Fields, parse_macro_input};

/// A derive macro that generates a `field_count()` method returning the number
/// of fields in a named struct as a `usize`.
///
/// This macro only needs to support named structs (e.g. `struct Foo { x: i32 }`).
/// It panics if used on an enum, union, tuple struct, or unit struct.
#[proc_macro_derive(FieldCount)]
pub fn field_count(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    field_count_impl(&input).into()
}

fn field_count_impl(input: &DeriveInput) -> proc_macro2::TokenStream {
    let name = &input.ident;

    let Data::Struct(data) = &input.data else {
        panic!("FieldCount only supports structs");
    };

    let Fields::Named(fields) = &data.fields else {
        panic!("FieldCount only supports named structs");
    };

    let count = fields.named.len();

    format!(
        "impl {name} {{
            pub fn field_count() -> usize {{ {count} }}
        }}"
    )
    .parse()
    .unwrap()
}
