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
