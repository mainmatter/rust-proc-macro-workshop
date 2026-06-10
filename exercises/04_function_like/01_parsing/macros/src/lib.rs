use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{Expr, Token, parse_macro_input, punctuated::Punctuated};

/// `avg!(a, b, c, ...)` — the average of its comma-separated arguments, as an `f64`.
///
/// Unlike a derive macro (which always receives a type definition), a function-like
/// macro receives *arbitrary* tokens. Here that's a comma-separated list of
/// expressions, which we parse with `syn`'s `Punctuated` before generating code.
#[proc_macro]
pub fn avg(input: TokenStream) -> TokenStream {
    // `parse_terminated` parses a comma-separated, optionally trailing-comma list of
    // expressions out of the raw token input.
    let args = parse_macro_input!(input with Punctuated::<Expr, Token![,]>::parse_terminated);
    avg_impl(args).into()
}

fn avg_impl(args: Punctuated<Expr, Token![,]>) -> proc_macro2::TokenStream {
    if args.is_empty() {
        return syn::Error::new(Span::call_site(), "avg! needs at least one argument")
            .to_compile_error();
    }

    // TODO: generate an expression that averages the parsed arguments as `f64`.
    //   - `args.len()` is how many there are — you'll need it as the divisor.
    //   - iterate `args.iter()` and splice each expression into the output with
    //     `quote!`'s `#( ... ),*` repetition, casting each one to `f64`.
    //   - the generated code should sum the values and divide by the count.
    //   The book's `min!` example shows the same parse-then-`quote!` shape; here the
    //   new wrinkle is using the *number* of arguments in the generated code.
    let _ = &args;
    todo!()
}
