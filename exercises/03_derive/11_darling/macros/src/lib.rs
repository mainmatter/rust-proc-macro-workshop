use darling::{FromDeriveInput, FromField, ast, util::Ignored};
use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

/// A small ORM-flavoured derive macro that reads both a container attribute and
/// field attributes — but parses them *declaratively* with `darling` instead of
/// walking `input.attrs` by hand.
///
/// - `#[model(table = "...")]` on the struct sets the table name (default
///   `"items"`).
/// - `#[model(rename = "...")]` on a field overrides that field's column name.
///
/// It generates `table_name()` and `columns()` methods.
#[proc_macro_derive(Model, attributes(model))]
pub fn model(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    // `darling::Error` knows how to render itself as `compile_error!` tokens.
    model_impl(&input)
        .unwrap_or_else(|err| err.write_errors())
        .into()
}

/// The whole attribute schema, described as plain structs. `darling`'s derives
/// generate the parsing — including good error messages for unknown keys, wrong
/// value types, and unsupported shapes.
#[derive(FromDeriveInput)]
#[darling(attributes(model), supports(struct_named))]
struct ModelOpts {
    ident: syn::Ident,
    // `ast::Data<V, F>` captures the body: `V` for enum variants (unused here,
    // so `Ignored`), `F` for the per-field options below.
    data: ast::Data<Ignored, FieldOpts>,
    #[darling(default = "default_table")]
    table: String,
}

fn default_table() -> String {
    "items".to_string()
}

#[derive(FromField)]
#[darling(attributes(model))]
struct FieldOpts {
    ident: Option<syn::Ident>,
    // `Option<T>` is optional automatically — no `#[darling(default)]` needed.
    rename: Option<String>,
}

fn model_impl(input: &DeriveInput) -> darling::Result<proc_macro2::TokenStream> {
    // One call parses everything according to the `ModelOpts` / `FieldOpts`
    // schema above; `?` turns any attribute error into a `darling::Error`.
    let opts = ModelOpts::from_derive_input(input)?;

    // TODO: build the `impl` block from the parsed `opts`:
    //   - `opts.ident` is the struct name.
    //   - `opts.table` is the (already-defaulted) table name.
    //   - `opts.data.take_struct()` yields the fields. `supports(struct_named)`
    //     guarantees a struct, so it's safe to `.expect(..)`.
    //   - a field's column name is its `rename` if set, otherwise its `ident`.
    //   Generate `table_name()` and `columns()` to match `tests/pass/model.rs`.
    let _ = opts;
    todo!()
}
