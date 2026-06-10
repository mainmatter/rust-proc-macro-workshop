use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

/// A derive macro that generates a `type_name()` method.
///
/// The actual code generation is delegated to `type_name_impl`, which uses
/// `proc_macro2` types so it could be tested independently.
#[proc_macro_derive(TypeName)]
pub fn type_name(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    type_name_impl(&input).into()
}

/// Generate the `type_name()` method implementation.
///
/// TODO: Fill in this function. It should:
/// 1. Extract the type name from `input.ident`.
/// 2. Return a `proc_macro2::TokenStream` containing an `impl` block with a
///    `pub fn type_name() -> &'static str` method that returns the type name as a string.
///
/// Hint: You can use `format!(...)` and `.parse().unwrap()` on a string to produce
/// a `proc_macro2::TokenStream`.
fn type_name_impl(input: &DeriveInput) -> proc_macro2::TokenStream {
    let name = &input.ident;

    format!(
        "impl {name} {{
            pub fn type_name() -> &'static str {{
                \"{name}\"
            }}
        }}"
    )
    .parse()
    .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::assert_matches;
    use syn::parse_str;

    /// This test demonstrates the key benefit of `proc-macro2`: you can unit-test
    /// your macro logic directly, without needing to compile a separate crate.
    ///
    /// `syn::parse_str` parses a string into a `DeriveInput` using `proc_macro2`
    /// types under the hood — no compiler context needed.
    #[test]
    fn generates_type_name_method() {
        let input: DeriveInput = parse_str("struct Gamma;").unwrap();
        let output = type_name_impl(&input);
        let output_str = output.to_string();
        assert!(
            output_str.contains("impl Gamma"),
            "expected generated code to contain 'impl Gamma', got: {output_str}"
        );
        assert!(
            output_str.contains("type_name"),
            "expected generated code to contain 'type_name', got: {output_str}"
        );
        assert!(
            output_str.contains("Gamma"),
            "expected generated code to return \"Gamma\" as the type name, got: {output_str}"
        );
    }

    #[test]
    fn output_parses_as_valid_impl() {
        let input: DeriveInput = parse_str("struct Delta { x: i32 }").unwrap();
        let output = type_name_impl(&input);
        let item: syn::Item = syn::parse2(output.clone())
            .unwrap_or_else(|e| panic!("generated code is not a valid Rust item: {e}\n{output}"));
        assert_matches!(
            item,
            syn::Item::Impl(_),
            "expected an impl block, got: {output}"
        );
    }
}
