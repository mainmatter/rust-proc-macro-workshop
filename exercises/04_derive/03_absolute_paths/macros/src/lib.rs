use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

/// A derive macro that generates a `type_name(&self) -> String` method returning
/// the name of the type as an owned `String`.
///
/// The generated code references `String` — a name the *user* might have
/// shadowed in their own module. To be robust, the macro must refer to standard
/// library items by their absolute path.
#[proc_macro_derive(TypeName)]
pub fn type_name(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    type_name_impl(&input).into()
}

fn type_name_impl(input: &DeriveInput) -> proc_macro2::TokenStream {
    let name = &input.ident;

    // TODO: this generated code uses the bare name `String`, which resolves to
    //   whatever `String` means at the *call site*. If the user shadowed it
    //   (see `examples/shadowed.rs`), this breaks.
    //
    //   Fix it by referring to the standard library type by its absolute path.
    //   Make both the return type and the constructor call use the absolute path.
    quote! {
        impl #name {
            pub fn type_name(&self) -> String {
                String::from(stringify!(#name))
            }
        }
    }
}
