use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, GenericParam, parse_macro_input, parse_quote};

/// A derive macro that generates an `empty() -> Self` constructor, setting every
/// field to its `Default::default()`.
///
/// Like the book's generic `DebugFields`, two things make it work on a generic
/// struct like `struct Wrapper<T> { value: T }`:
/// - the generated `impl` must repeat the type's generics, via `split_for_impl`;
/// - because the body calls `Default::default()` on each field, every type
///   parameter needs a `Default` bound, which the macro adds itself.
#[proc_macro_derive(Empty)]
pub fn empty(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    empty_impl(&input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

fn empty_impl(input: &DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let name = &input.ident;

    let Data::Struct(data) = &input.data else {
        return Err(syn::Error::new_spanned(
            input,
            "Empty only supports structs",
        ));
    };

    let Fields::Named(fields) = &data.fields else {
        return Err(syn::Error::new_spanned(
            input,
            "Empty only supports structs with named fields",
        ));
    };

    // Start from the type's own generics.
    let mut generics = input.generics.clone();

    // TODO: the generated `empty()` calls `Default::default()` on each field, so
    //   every *type* parameter needs a `::core::default::Default` bound — or the
    //   code won't compile for `struct Wrapper<T>`. The book added a `Debug` bound
    //   the same way: iterate `generics.params`, match the type-parameter case,
    //   and push the bound onto it. Leave lifetimes and const parameters alone.

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let inits = fields.named.iter().map(|f| {
        let ident = f.ident.as_ref().unwrap();
        quote! { #ident: ::core::default::Default::default() }
    });

    Ok(quote! {
        #[automatically_derived]
        impl #impl_generics #name #ty_generics #where_clause {
            pub fn empty() -> Self {
                Self { #(#inits),* }
            }
        }
    })
}
