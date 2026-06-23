use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_macro_input};

/// A derive macro that generates a getter method for every field of a named
/// struct: `struct Foo { x: i32 }` gets `fn x(&self) -> &i32`.
///
/// It only makes sense for structs with named fields. For anything else it must
/// report an error — and it should do so with a real compiler diagnostic, not a
/// panic.
#[proc_macro_derive(Getters)]
pub fn getters(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    getters_impl(&input).into()
}

fn getters_impl(input: &DeriveInput) -> proc_macro2::TokenStream {
    let name = &input.ident;

    let Data::Struct(data) = &input.data else {
        // TODO: same here — produce a `compile_error!` instead of panicking.
        panic!("Getters can only be derived for structs");
    };

    let Fields::Named(fields) = &data.fields else {
        // TODO: a panicking macro shows the user the unhelpful "proc-macro
        //   derive panicked" message. Return a normal compiler error instead:
        //   a token stream that invokes the `compile_error!` macro with an
        //   explanatory message. (The book section shows the shape.)
        panic!("Getters can only be derived for structs with named fields");
    };

    let getters = fields.named.iter().map(|f| {
        let fname = f.ident.as_ref().unwrap();
        let ty = &f.ty;
        quote! {
            pub fn #fname(&self) -> &#ty {
                &self.#fname
            }
        }
    });

    quote! {
        #[automatically_derived]
        impl #name {
            #(#getters)*
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;

    #[test]
    fn rejects_enums_with_a_compile_error() {
        // A misused macro must *return* a `compile_error!` invocation, not panic.
        // If `getters_impl` panicked, this test would fail with that panic.
        let input: DeriveInput = parse_quote! { enum E { A, B } };
        let output = getters_impl(&input).to_string();
        assert!(
            output.contains("compile_error"),
            "expected a compile_error!, got: {output}"
        );
    }

    #[test]
    fn rejects_tuple_structs_with_a_compile_error() {
        let input: DeriveInput = parse_quote! { struct T(i32, i32); };
        let output = getters_impl(&input).to_string();
        assert!(
            output.contains("compile_error"),
            "expected a compile_error!, got: {output}"
        );
    }
}
