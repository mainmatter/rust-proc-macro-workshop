use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

/// A derive macro that generates a `repeated() -> String` method, returning the
/// type's name repeated a number of times.
///
/// The count is read from a *container attribute*: `#[repeat(times = N)]`, where
/// `N` is an integer. When the attribute is absent, the count defaults to `1`.
///
/// Note the `attributes(repeat)` in the macro declaration below: a derive must
/// declare every helper attribute it reads, or the compiler rejects the unknown
/// `#[repeat(...)]` as an error before your macro ever runs.
#[proc_macro_derive(Repeat, attributes(repeat))]
pub fn repeat(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    repeat_impl(&input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

fn repeat_impl(input: &DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let name = &input.ident;
    let times = extract_times(input)?;

    Ok(quote! {
        #[automatically_derived]
        impl #name {
            pub fn repeated() -> String {
                stringify!(#name).repeat(#times)
            }
        }
    })
}

/// Reads the `times` value out of `#[repeat(times = N)]`, defaulting to `1` if
/// the attribute is missing.
fn extract_times(input: &DeriveInput) -> syn::Result<usize> {
    let mut times = 1usize;

    for attr in &input.attrs {
        // Skip every attribute that isn't ours (`derive`, `doc`, etc.).
        if !attr.path().is_ident("repeat") {
            continue;
        }

        // Walk the comma-separated `key = value` pairs inside `repeat(...)`.
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("times") {
                let lit: syn::LitInt = meta.value()?.parse()?;
                times = lit.base10_parse()?;
                Ok(())
            } else {
                Err(meta.error("unsupported repeat attribute; expected `times`"))
            }
        })?;
    }

    Ok(times)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use syn::parse_quote;

    #[test]
    fn default_times() {
        let input: DeriveInput = parse_quote! { struct S; };
        assert_eq!(extract_times(&input).unwrap(), 1);
    }

    #[test]
    fn custom_times() {
        let input: DeriveInput = parse_quote! { #[repeat(times = 4)] struct S; };
        assert_eq!(extract_times(&input).unwrap(), 4);
    }
}
