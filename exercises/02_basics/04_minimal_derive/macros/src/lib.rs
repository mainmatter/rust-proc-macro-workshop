use proc_macro::TokenStream;

/// A derive macro that generates a `type_name()` method returning the type's name.
///
/// Unlike the previous exercise, this must work for *any* struct, not just a
/// hardcoded name.
///
/// Use `syn::parse_macro_input!` and `syn::DeriveInput` to parse the input and
/// extract the type's name from `input.ident`.
///
/// Hint:
/// ```ignore
/// let input = parse_macro_input!(input as DeriveInput);
/// let name = &input.ident;
/// ```
#[proc_macro_derive(TypeName)]
pub fn type_name(input: TokenStream) -> TokenStream {
    todo!()
}
