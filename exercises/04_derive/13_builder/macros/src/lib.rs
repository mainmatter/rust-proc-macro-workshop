use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DeriveInput, Fields, GenericArgument, PathArguments, Type, parse_macro_input};

/// The capstone: `#[derive(Builder)]`. For a struct `Command`, it generates a
/// `CommandBuilder` with a setter per field and a fallible `build()` method.
///
/// It pulls together everything from this chapter:
/// - reading named struct fields and generating code per field;
/// - inspecting a field's *type* to special-case `Option<T>` (those fields are
///   optional — no error if left unset);
/// - reporting a clear runtime error from `build()` when a required field is
///   missing;
/// - using absolute paths and a derived `...Builder` name in the output.
#[proc_macro_derive(Builder)]
pub fn builder(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    builder_impl(&input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

fn builder_impl(input: &DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let name = &input.ident;
    let builder_name = format_ident!("{}Builder", name);

    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => {
                return Err(syn::Error::new_spanned(
                    input,
                    "Builder only supports structs with named fields",
                ));
            }
        },
        _ => {
            return Err(syn::Error::new_spanned(
                input,
                "Builder only supports structs",
            ));
        }
    };

    let mut builder_fields: Vec<proc_macro2::TokenStream> = Vec::new();
    let mut builder_init: Vec<proc_macro2::TokenStream> = Vec::new();
    let mut setters: Vec<proc_macro2::TokenStream> = Vec::new();
    let mut build_fields: Vec<proc_macro2::TokenStream> = Vec::new();

    for field in fields {
        let ident = field.ident.as_ref().unwrap();
        let ty = &field.ty;

        // If the field is `Option<U>`, `optional` is `Some(U)` and the field is
        // not required. Otherwise the setter and storage use the field type as-is.
        let optional = option_inner(ty);
        let stored = optional.unwrap_or(ty);

        // Every builder field starts empty: `name: Option<stored>`.
        builder_fields.push(quote! {
            #ident: ::core::option::Option<#stored>,
        });
        builder_init.push(quote! {
            #ident: ::core::option::Option::None,
        });

        // The setter takes the *inner* type and stores `Some(value)`.
        setters.push(quote! {
            pub fn #ident(&mut self, value: #stored) -> &mut Self {
                self.#ident = ::core::option::Option::Some(value);
                self
            }
        });

        // TODO: push the expression that produces this field's final value inside
        //   `build()` — i.e. `#ident: <expr>,`. `build` takes `&mut self`, so you
        //   can't move a field out — clone it. There are two cases:
        //   - optional field (`optional.is_some()`): the stored value is already
        //     an `Option`, so clone it straight through (no error).
        //   - required field: it must be present by now, so clone it, convert a
        //     missing (`None`) value into an error that names the field, and let
        //     `?` propagate it out of `build()`.
        let build_field: proc_macro2::TokenStream = todo!();
        build_fields.push(build_field);
    }

    Ok(quote! {
        pub struct #builder_name {
            #(#builder_fields)*
        }

        impl #name {
            pub fn builder() -> #builder_name {
                #builder_name {
                    #(#builder_init)*
                }
            }
        }

        impl #builder_name {
            #(#setters)*

            pub fn build(
                &mut self,
            ) -> ::std::result::Result<#name, ::std::boxed::Box<dyn ::std::error::Error>> {
                ::std::result::Result::Ok(#name {
                    #(#build_fields)*
                })
            }
        }
    })
}

/// If `ty` is `Option<U>`, returns `Some(U)`; otherwise `None`.
fn option_inner(ty: &Type) -> Option<&Type> {
    let Type::Path(type_path) = ty else {
        return None;
    };
    if type_path.qself.is_some() {
        return None;
    }
    let segment = type_path.path.segments.last()?;
    if segment.ident != "Option" {
        return None;
    }
    let PathArguments::AngleBracketed(args) = &segment.arguments else {
        return None;
    };
    match args.args.first()? {
        GenericArgument::Type(inner) => Some(inner),
        _ => None,
    }
}
