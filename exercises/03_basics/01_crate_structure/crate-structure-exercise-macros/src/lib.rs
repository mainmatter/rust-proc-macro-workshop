use proc_macro::TokenStream;

#[proc_macro_derive(Empty)]
pub fn empty(_input: TokenStream) -> TokenStream {
    TokenStream::new()
}
