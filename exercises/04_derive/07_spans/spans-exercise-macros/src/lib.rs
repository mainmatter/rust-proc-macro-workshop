use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Type, parse_macro_input};

/// `Getters`, now with a *per-field* validation rule: it refuses to generate a
/// getter for a field of the unit type `()`. The interesting part is the span —
/// the error should point at the offending field, not at the whole struct.
#[proc_macro_derive(Getters)]
pub fn getters(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    getters_impl(&input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

fn getters_impl(input: &DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let name = &input.ident;

    // Whole-item errors: the span covers the entire input, which is the right
    // choice here — the *type itself* is the wrong shape.
    let Data::Struct(data) = &input.data else {
        return Err(syn::Error::new_spanned(
            input,
            "Getters can only be derived for structs",
        ));
    };

    let Fields::Named(fields) = &data.fields else {
        return Err(syn::Error::new_spanned(
            input,
            "Getters can only be derived for structs with named fields",
        ));
    };

    // Per-field error: the span should cover just the offending field.
    for field in fields.named.iter() {
        if is_unit_type(&field.ty) {
            let fname = field.ident.as_ref().unwrap();
            return Err(syn::Error::new_spanned(
                &field.ty,
                format!("Getters can't generate a getter for the `()`-typed field `{fname}`"),
            ));
        }
    }

    let getters = fields.named.iter().map(|f| {
        let fname = f.ident.as_ref().unwrap();
        let ty = &f.ty;
        quote! {
            pub fn #fname(&self) -> &#ty {
                &self.#fname
            }
        }
    });

    Ok(quote! {
        #[automatically_derived]
        impl #name {
            #(#getters)*
        }
    })
}

/// Returns `true` for the unit type `()`, which `syn` models as an empty tuple.
fn is_unit_type(ty: &Type) -> bool {
    matches!(ty, Type::Tuple(t) if t.elems.is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;

    #[test]
    fn rejects_unit_typed_field() {
        let input: DeriveInput = parse_quote! { struct S { ok: i32, marker: () } };
        let err = getters_impl(&input).unwrap_err();
        let msg = err.to_string();
        // The message should name the specific field that's at fault...
        assert!(
            msg.contains("marker"),
            "message should name the field: {msg}"
        );
        // ...and mention its type.
        assert!(msg.contains("()"), "message should mention the type: {msg}");
    }

    #[test]
    fn accepts_normal_fields() {
        let input: DeriveInput = parse_quote! { struct S { x: i32, y: i32 } };
        assert!(getters_impl(&input).is_ok());
    }
}
