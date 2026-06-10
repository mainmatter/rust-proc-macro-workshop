use proc_macro::{TokenStream, TokenTree};

/// A derive macro that counts the total number of `Ident` tokens in its input,
/// including those nested inside groups (delimited by `()`, `[]`, or `{}`).
///
/// It should generate an `ident_count()` method returning a `usize`.
///
/// Hints:
/// - Iterate over the `TokenStream` and match on each `TokenTree`.
/// - For `TokenTree::Ident(_)`, increment the count.
/// - For `TokenTree::Group(g)`, recurse into `g.stream()` to count idents inside.
/// - To extract the struct name for the `impl` block, find the first `Ident` after
///   the "struct" keyword in the token stream (similar to the "parsing the hard way"
///   example in the "A minimal derive macro" section, but working with `TokenTree`
///   values instead of strings).
#[proc_macro_derive(IdentCount)]
pub fn ident_count(input: TokenStream) -> TokenStream {
    let count = count_idents(input.clone());
    let name = find_struct_name(input);

    format!("impl {name} {{ pub fn ident_count() -> usize {{ {count} }} }}")
        .parse()
        .unwrap()
}

/// Count the total number of `Ident` tokens in a `TokenStream`,
/// recursing into groups.
fn count_idents(stream: TokenStream) -> usize {
    todo!()
}

/// Find the struct name by looking for the first `Ident` after the
/// "struct" keyword.
fn find_struct_name(stream: TokenStream) -> String {
    todo!()
}
