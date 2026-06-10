use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DeriveInput, Fields, Ident, parse_macro_input};

/// A derive macro that generates a *module-level* accessor function for every
/// field of a named struct, plus a method that calls it. For
/// `struct Point { x: i32 }` it emits, alongside the `impl`, a function returning
/// `&point.x`, and a `Point::x(&self)` method that forwards to it.
///
/// The accessor functions are emitted at module scope (next to the `impl`, not
/// inside it) on purpose — imagine passing them around as `fn(&Point) -> &T`
/// pointers. That means their names share the surrounding module and must not
/// collide.
#[proc_macro_derive(Accessors)]
pub fn accessors(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    accessors_impl(&input).into()
}

fn accessors_impl(input: &DeriveInput) -> proc_macro2::TokenStream {
    let name = &input.ident;

    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => panic!("Accessors only supports named structs"),
        },
        _ => panic!("Accessors only supports structs"),
    };

    let mut accessor_fns = Vec::new();
    let mut methods = Vec::new();

    for field in fields {
        let fname = field.ident.as_ref().unwrap();
        let ty = &field.ty;
        let accessor = accessor_name(name, fname);

        accessor_fns.push(quote! {
            #[allow(non_snake_case)]
            fn #accessor(value: &#name) -> &#ty {
                &value.#fname
            }
        });

        methods.push(quote! {
            pub fn #fname(&self) -> &#ty {
                #accessor(self)
            }
        });
    }

    quote! {
        #(#accessor_fns)*

        impl #name {
            #(#methods)*
        }
    }
}

/// Builds a collision-proof name for one field's module-level accessor function.
fn accessor_name(struct_name: &Ident, field_name: &Ident) -> Ident {
    // TODO: return a module-unique identifier built with `format_ident!`. It has
    //   to survive TWO kinds of clash at module scope:
    //     - within one struct, every field's function shares the module, so the
    //       name must vary per field (use `field_name`);
    //     - across structs, two structs may share a field name (both have an
    //       `x`), so the name must vary per struct too (use `struct_name`).
    //   A `__` prefix keeps you clear of names a user would plausibly write.
    format_ident!("__{}_{}", struct_name, field_name)
}
