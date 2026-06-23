use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Field, Fields, parse_macro_input};

/// A derive macro that generates a `column_names() -> Vec<&'static str>` method,
/// one entry per field.
///
/// A field's column name defaults to its Rust name, but can be overridden with a
/// *field attribute*. Unlike the book's `#[label("...")]` (a list attribute), this
/// one uses the *name-value* shape: `#[rename = "..."]`.
#[proc_macro_derive(Renamed, attributes(rename))]
pub fn renamed(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    renamed_impl(&input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

fn renamed_impl(input: &DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let name = &input.ident;

    let Data::Struct(data) = &input.data else {
        return Err(syn::Error::new_spanned(
            input,
            "Renamed can only be derived for structs",
        ));
    };

    let Fields::Named(fields) = &data.fields else {
        return Err(syn::Error::new_spanned(
            input,
            "Renamed can only be derived for structs with named fields",
        ));
    };

    let columns = fields
        .named
        .iter()
        .map(column_name)
        .collect::<syn::Result<Vec<String>>>()?;

    Ok(quote! {
        #[automatically_derived]
        impl #name {
            pub fn column_names() -> Vec<&'static str> {
                vec![ #(#columns),* ]
            }
        }
    })
}

/// Returns the column name for a field: the value of its `#[rename = "..."]`
/// attribute if present, otherwise the field's own name.
fn column_name(field: &Field) -> syn::Result<String> {
    for attr in &field.attrs {
        if attr.path().is_ident("rename") {
            // `#[rename = "..."]` is a *name-value* attribute, so `require_name_value`
            // gives us the `key = value` pair.
            let nv = attr.meta.require_name_value()?;

            // TODO: `nv.value` is a `syn::Expr`. Pull the string out of it and
            //   `return Ok(..)` with it. The value should be a string-literal
            //   expression — match the `syn::Expr::Lit` wrapping a `syn::Lit::Str`
            //   and use its `.value()`. If it's anything else, `return` an `Err`
            //   (spanned on `nv.value`) saying a string literal was expected.
            //   (The book's `parse_args` doesn't help here — this is the name-value
            //   shape, so you inspect the expression yourself.)
            let _ = nv;
            todo!()
        }
    }

    Ok(field.ident.as_ref().unwrap().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use syn::parse_quote;

    fn first_field(input: DeriveInput) -> Field {
        match input.data {
            Data::Struct(data) => data.fields.into_iter().next().unwrap(),
            _ => panic!("expected a struct"),
        }
    }

    #[test]
    fn defaults_to_field_name() {
        let field = first_field(parse_quote! { struct S { user_id: u64 } });
        assert_eq!(column_name(&field).unwrap(), "user_id");
    }

    #[test]
    fn uses_rename_attribute() {
        let field = first_field(parse_quote! { struct S { #[rename = "id"] user_id: u64 } });
        assert_eq!(column_name(&field).unwrap(), "id");
    }
}
