use proc_macro::TokenStream;

/// A derive macro that generates a `type_name()` method returning the type's name.
///
/// For this exercise, it's okay to hardcode the struct name "Greeting" in the
/// generated code. We'll learn how to extract it from the input in the next exercise.
///
/// The generated code should look like:
///
/// ```ignore
/// impl Greeting {
///     pub fn type_name() -> &'static str {
///         "Greeting"
///     }
/// }
/// ```
///
/// Hint: You can parse a string into a `TokenStream` using `.parse().unwrap()`.
#[proc_macro_derive(TypeName)]
pub fn type_name(_input: TokenStream) -> TokenStream {
    todo!()
}
