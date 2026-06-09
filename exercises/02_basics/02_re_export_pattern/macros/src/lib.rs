use proc_macro::TokenStream;

#[proc_macro_derive(Describe)]
pub fn describe(input: TokenStream) -> TokenStream {
    let input = input.to_string();
    let name = input
        .split_whitespace()
        .skip_while(|t| *t != "struct")
        .nth(1)
        .expect("expected a struct")
        .trim_end_matches(|c: char| !c.is_alphanumeric());

    // NOTE: referring to the trait as `crate::SelfDescribe` is a shortcut that only works
    // because, in this exercise, the deriving code lives in the same crate that defines the
    // trait. A real downstream crate would have its own `crate` root with no `SelfDescribe`
    // in it, so this generated code would fail to compile there. Producing a robust path to
    // the trait (e.g. an absolute `::re_export_exercise::SelfDescribe`) is covered in a later
    // chapter; for now this keeps the example minimal.
    format!(
        "impl crate::SelfDescribe for {name} {{
            fn describe(&self) -> &'static str {{
                \"{name}\"
            }}
        }}"
    )
    .parse()
    .unwrap()
}
