use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_macro_input};

/// A derive macro that generates a `field_names()` method returning the names of
/// all fields in a named struct as a `&'static [&'static str]`.
///
/// The current implementation uses string formatting. Your task is to rewrite
/// `field_names_impl` to use `quote!` instead.
///
/// Hint: look at the `quote!` repetition syntax covered in the book section.
#[proc_macro_derive(FieldNames)]
pub fn field_names(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    field_names_impl(&input).into()
}

fn field_names_impl(input: &DeriveInput) -> proc_macro2::TokenStream {
    let name = &input.ident;

    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => panic!("FieldNames only supports named structs"),
        },
        _ => panic!("FieldNames only supports structs"),
    };

    // TODO: Rewrite this function to use `quote!` instead of `format!`.
    let field_names_str = fields
        .iter()
        .map(|f| format!("\"{}\"", f.ident.as_ref().unwrap()))
        .collect::<Vec<_>>()
        .join(", ");

    format!(
        "impl {name} {{
            pub fn field_names() -> &'static [&'static str] {{
                &[{field_names_str}]
            }}
        }}"
    )
    .parse()
    .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_str;

    /// Unit-test the code generation directly, the same way you did in the
    /// `proc-macro2` exercise — parse a `DeriveInput` from a string and call
    /// `field_names_impl`, no separate crate and no real `#[derive]` needed.
    ///
    /// This keeps passing whether `field_names_impl` uses `format!` or `quote!`,
    /// since both produce the same tokens.
    ///
    /// Note its limits, though: it only checks the tokens we generate — not that
    /// they actually compile and run, and not what error a misuse shows the user.
    /// The next section (`trybuild`) covers exactly those gaps.
    ///
    /// We assert only on the structural shape (`impl Color`, a `field_names`
    /// method) rather than the exact encoding of the field names: `quote!` lets
    /// you emit them as string literals *or* via `stringify!`, and both are
    /// correct. That brittleness is itself a hint at why token-string assertions
    /// only go so far.
    #[test]
    fn generates_field_names() {
        let input: DeriveInput = parse_str("struct Color { r: u8, g: u8, b: u8 }").unwrap();
        let output = field_names_impl(&input).to_string();
        assert!(
            output.contains("impl Color"),
            "expected an `impl Color` block, got: {output}"
        );
        assert!(
            output.contains("field_names"),
            "expected a field_names method, got: {output}"
        );
    }

    #[test]
    fn empty_struct_still_has_the_method() {
        let input: DeriveInput = parse_str("struct Empty {}").unwrap();
        let output = field_names_impl(&input).to_string();
        assert!(
            output.contains("field_names"),
            "expected a field_names method, got: {output}"
        );
    }
}
