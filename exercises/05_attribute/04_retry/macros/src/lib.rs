use darling::{FromMeta, ast::NestedMeta};
use proc_macro::TokenStream;
use syn::{ItemFn, parse_macro_input};

/// The capstone: `#[retry(times = 3, delay_ms = 100)]`. It wraps a function that
/// returns a `Result` so that, on `Err`, it tries again — up to `times` attempts,
/// sleeping `delay_ms` milliseconds between them — before giving up and returning
/// the last error.
///
/// It pulls together the whole chapter:
/// - parsing the **attribute arguments** with `darling`'s `FromMeta` (section 03);
/// - replacing the function **body** while keeping its signature (section 01);
/// - absolute paths so the generated code compiles anywhere (chapter 3).
#[derive(FromMeta)]
struct RetryArgs {
    times: u32,
    #[darling(default)]
    delay_ms: u64,
}

#[proc_macro_attribute]
pub fn retry(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut func = parse_macro_input!(item as ItemFn);

    let meta = match NestedMeta::parse_meta_list(attr.into()) {
        Ok(meta) => meta,
        Err(err) => return TokenStream::from(darling::Error::from(err).write_errors()),
    };
    let args = match RetryArgs::from_list(&meta) {
        Ok(args) => args,
        Err(err) => return TokenStream::from(err.write_errors()),
    };

    // TODO: replace the function's body with a retry loop, then re-emit `func`.
    //   You have everything you need: `func.block` (the original body),
    //   `func.sig.output` (the `-> Result<..>` return type), `args.times`, and
    //   `args.delay_ms`. (You'll want `use quote::quote;` and
    //   `use syn::{Block, parse_quote};`.)
    //
    //   Build a new `syn::Block` with `parse_quote! {{ ... }}` that loops up to
    //   `times` times. In each iteration:
    //     - run the original body as a CLOSURE that returns the function's own type,
    //       so a `?`/`return` inside it yields a value to inspect instead of escaping
    //       the whole function: `(|| #output #block)()`, where `#output` is
    //       `&func.sig.output` and `#block` is `&func.block`;
    //     - on `Ok(v)`, `return Ok(v)` immediately;
    //     - on `Err(e)`, if this was the last allowed attempt `return Err(e)`,
    //       otherwise sleep `delay_ms` (`::std::thread::sleep` +
    //       `::std::time::Duration::from_millis`) and loop again.
    //
    //   Assign the new block to `func.block` (a `Box<syn::Block>`) and return
    //   `quote!(#func).into()`. Use absolute paths (`::core::result::Result`, etc.)
    //   as you did in chapter 3.
    let _ = (&mut func, args.times, args.delay_ms);
    todo!()
}
