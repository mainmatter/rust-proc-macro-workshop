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
    stream
        .into_iter()
        .map(|tree| match tree {
            TokenTree::Ident(_) => 1,
            TokenTree::Group(g) => count_idents(g.stream()),
            _ => 0,
        })
        .sum()
}

/// Find the struct name by looking for the first `Ident` after the
/// "struct" keyword.
fn find_struct_name(stream: TokenStream) -> String {
    let mut iter = stream.into_iter();
    while let Some(tree) = iter.next() {
        if let TokenTree::Ident(ident) = &tree {
            if ident.to_string() == "struct" {
                if let Some(TokenTree::Ident(name)) = iter.next() {
                    return name.to_string();
                }
            }
        }
    }
    panic!("expected a struct")
}
