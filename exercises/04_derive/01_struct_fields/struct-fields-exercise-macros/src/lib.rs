use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_macro_input};

/// A derive macro that generates a `debug_fields(&self) -> Vec<String>` method,
/// returning the `{:?}` representation of every field.
///
/// It must work for all three struct layouts:
/// - named structs:   `struct Foo { x: i32 }`   -> access via `self.x`
/// - tuple structs:   `struct Foo(i32, i32)`    -> access via `self.0`
/// - unit structs:    `struct Foo;`             -> no fields
///
/// Panics if used on an enum or union.
#[proc_macro_derive(DebugFields)]
pub fn debug_fields(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    debug_fields_impl(&input).into()
}

fn debug_fields_impl(input: &DeriveInput) -> proc_macro2::TokenStream {
    let name = &input.ident;

    let fields = match &input.data {
        Data::Struct(data) => &data.fields,
        _ => panic!("DebugFields only supports structs"),
    };

    // Build one `format!("{:?}", <field access>)` expression per field. The
    // field-access expression differs between named and tuple structs.
    let entries: Vec<proc_macro2::TokenStream> = match fields {
        // struct Foo { x: i32, y: i32 } -> access via field name: `self.x`
        Fields::Named(fields) => fields
            .named
            .iter()
            .map(|f| {
                let ident = f.ident.as_ref().unwrap();
                quote! { format!("{:?}", &self.#ident) }
            })
            .collect(),
        // TODO: tuple struct — `struct Foo(i32, i32)`. The fields have no names,
        //   so each is accessed by position (`self.0`, `self.1`). Mirror the
        //   named branch above, but build the access from each field's *index*
        //   rather than its ident. Watch the gotcha the book flags: a plain
        //   integer renders with a type suffix (`0usize`) and is invalid as a
        //   field index — reach for the quote-aware index type `syn` provides.
        Fields::Unnamed(fields) => fields
            .unnamed
            .iter()
            .enumerate()
            .map(|(i, _)| {
                let index = syn::Index::from(i);
                quote! { format!("{:?}", &self.#index) }
            })
            .collect(),
        // unit struct — `struct Foo;` — has no fields at all.
        Fields::Unit => Vec::new(),
    };

    quote! {
        #[automatically_derived]
        impl #name {
            pub fn debug_fields(&self) -> Vec<String> {
                vec![ #(#entries),* ]
            }
        }
    }
}
