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

    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => {
                return Err(syn::Error::new_spanned(
                    input,
                    "Renamed can only be derived for structs with named fields",
                ));
            }
        },
        _ => {
            return Err(syn::Error::new_spanned(
                input,
                "Renamed can only be derived for structs",
            ));
        }
    };

    let columns = fields
        .iter()
        .map(column_name)
        .collect::<syn::Result<Vec<String>>>()?;

    Ok(quote! {
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

            if let syn::Expr::Lit(syn::ExprLit {
                lit: syn::Lit::Str(s),
                ..
            }) = &nv.value
            {
                return Ok(s.value());
            }
            return Err(syn::Error::new_spanned(
                &nv.value,
                "expected a string literal",
            ));
        }
    }

    Ok(field.ident.as_ref().unwrap().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use syn::parse_str;

    fn first_field(src: &str) -> Field {
        let input: DeriveInput = parse_str(src).unwrap();
        match input.data {
            Data::Struct(data) => data.fields.into_iter().next().unwrap(),
            _ => panic!("expected a struct"),
        }
    }

    #[test]
    fn defaults_to_field_name() {
        let field = first_field("struct S { user_id: u64 }");
        assert_eq!(column_name(&field).unwrap(), "user_id");
    }

    #[test]
    fn uses_rename_attribute() {
        let field = first_field(r#"struct S { #[rename = "id"] user_id: u64 }"#);
        assert_eq!(column_name(&field).unwrap(), "id");
    }
}
