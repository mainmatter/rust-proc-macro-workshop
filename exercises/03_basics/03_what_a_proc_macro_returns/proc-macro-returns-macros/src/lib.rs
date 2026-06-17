use proc_macro::TokenStream;

/// A derive macro that generates a `type_name()` method on the annotated type,
/// returning the type's name as a `&'static str`.
///
/// For this exercise it's fine to hard-code the type name in the generated `impl`
/// — you'll learn how to read it from the input in the next exercise. The `Hello`
/// example in the book section shows the shape of the code you need to return.
#[proc_macro_derive(TypeName)]
pub fn type_name(_input: TokenStream) -> TokenStream {
    todo!()
}
