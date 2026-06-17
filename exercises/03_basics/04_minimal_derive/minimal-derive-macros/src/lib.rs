use proc_macro::TokenStream;

/// A derive macro that generates a `type_name()` method returning the type's name.
///
/// Unlike the previous exercise, this must work for *any* struct, not just a
/// hardcoded name — so you'll need to parse the input and read the type's name
/// from it. The book section's `Greet` example shows the technique.
#[proc_macro_derive(TypeName)]
pub fn type_name(input: TokenStream) -> TokenStream {
    todo!()
}
