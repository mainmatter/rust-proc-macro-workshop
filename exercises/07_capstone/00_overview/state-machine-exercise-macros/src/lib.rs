use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::punctuated::Punctuated;
use syn::{Data, DeriveInput, Fields, Ident, Token, parse_macro_input};

/// The grand finale: `#[derive(StateMachine)]`.
///
/// Derived on an enum whose unit variants are the *states*, it reads two helper
/// attributes — `#[initial]` on the starting state and `#[transition(..)]` on each
/// state's reachable neighbours — and generates the methods that drive the machine:
/// `initial()`, `name()`, `can_transition_to()` and a fallible `transition_to()`.
///
/// It pulls together the whole workshop:
/// - per-variant **code generation** from an enum (chapter 4, enums);
/// - reading **custom helper attributes** declared via `attributes(..)` (chapter 4);
/// - **error reporting** with spanned `syn::Error`s — rejecting a transition to a
///   state that doesn't exist, a missing/duplicate `#[initial]`, a non-enum, or a
///   data-carrying variant (chapter 4, error reporting);
/// - a **generated identifier** (the per-type `..InvalidTransition` error) built with
///   `format_ident!`, plus absolute paths, so the output never clashes and compiles
///   anywhere (chapter 4, name clashes / absolute paths).
///
/// This entry point is boilerplate you've written many times: parse the input, run
/// the implementation, and turn any `syn::Error` into a `compile_error!`. The two
/// halves it calls — `parse_state_machine` and `generate` — are yours to write.
#[proc_macro_derive(StateMachine, attributes(initial, transition))]
pub fn state_machine(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    state_machine_impl(&input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

/// The validated description of a state machine, distilled from the `DeriveInput`.
///
/// This is the contract between the two halves: `parse_state_machine` fills it in,
/// `generate` consumes it. Because parsing already rejected every bad input, by the
/// time `generate` runs every field here is known to be good.
struct StateMachine {
    name: Ident,
    variants: Vec<Ident>,
    initial: Ident,
    transitions: Vec<(Ident, Ident)>,
}

fn state_machine_impl(input: &DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let sm = parse_state_machine(input)?;
    Ok(generate(&sm))
}

/// Validate the derive input and distil it into a `StateMachine` — the "understand
/// the input" half of the macro.
fn parse_state_machine(input: &DeriveInput) -> syn::Result<StateMachine> {
    // TODO: validate `input` and build a `StateMachine`. Every broken rule is a
    //   spanned `syn::Error` (chapter 4) — the compile-fail tests pin both the
    //   messages *and* the spans, so use `syn::Error::new_spanned(<node>, <message>)`
    //   on the most specific node you can.
    //
    //   The rules:
    //     1. The input must be an enum — its variants are the states. Match
    //        `input.data` against `Data::Enum`; reject anything else.
    //     2. Every variant must be a unit variant (a state is just a name). Reject
    //        any variant whose `fields` aren't `Fields::Unit`, spanning the variant.
    //        Collect the surviving variant names — you'll need the full list to
    //        validate transition targets in step 4.
    //     3. Exactly one variant must carry `#[initial]`. Spot it with
    //        `attr.path().is_ident("initial")` (just like the transition attribute
    //        below). Reject both "none" and "more than one", with messages/spans that
    //        say which.
    //     4. Collect the `#[transition(..)]` edges. For each variant — the `from`
    //        state — read every `#[transition(..)]` attribute's comma-separated
    //        targets and pair each one with `from`. Read the parenthesised list with
    //        `Attribute::parse_args_with` and a `Punctuated<Ident, Token![,]>` parser
    //        (chapters 4–5; `Punctuated` and `Token` are imported for you). A target
    //        that isn't one of the variant names from step 2 is an error spanned on
    //        that target token.
    //
    //   Tip: collect the variant names *first*, then check transition targets against
    //   them — you can't know a target is unknown until you've seen every state.
    let name = input.ident.clone();

    // 1. The input must be an enum.
    let Data::Enum(data) = &input.data else {
        return Err(syn::Error::new_spanned(
            input,
            "StateMachine can only be derived for enums",
        ));
    };

    // 2. Every variant must be a unit variant. Collect the state names.
    let mut variants = Vec::new();
    for variant in &data.variants {
        if !matches!(variant.fields, Fields::Unit) {
            return Err(syn::Error::new_spanned(
                variant,
                "StateMachine states must be unit variants (no fields)",
            ));
        }
        variants.push(variant.ident.clone());
    }

    // 3. Exactly one variant must carry `#[initial]`.
    let mut initial: Option<Ident> = None;
    for variant in &data.variants {
        for attr in &variant.attrs {
            if attr.path().is_ident("initial") {
                if initial.is_some() {
                    return Err(syn::Error::new_spanned(
                        variant,
                        "only one variant may be marked `#[initial]`",
                    ));
                }
                initial = Some(variant.ident.clone());
            }
        }
    }
    let Some(initial) = initial else {
        return Err(syn::Error::new_spanned(
            input,
            "one variant must be marked `#[initial]`",
        ));
    };

    // 4. Collect the `#[transition(..)]` edges, validating every target.
    let mut transitions = Vec::new();
    for variant in &data.variants {
        let from = variant.ident.clone();
        for attr in &variant.attrs {
            if attr.path().is_ident("transition") {
                let targets =
                    attr.parse_args_with(Punctuated::<Ident, Token![,]>::parse_terminated)?;
                for target in targets {
                    if !variants.contains(&target) {
                        return Err(syn::Error::new_spanned(
                            &target,
                            format!("unknown state `{target}`; not a variant of this enum"),
                        ));
                    }
                    transitions.push((from.clone(), target));
                }
            }
        }
    }

    Ok(StateMachine {
        name,
        variants,
        initial,
        transitions,
    })
}

/// Turn a validated `StateMachine` into the generated code — the "emit the output"
/// half. Every value here is already known good, so this is pure code generation.
fn generate(sm: &StateMachine) -> proc_macro2::TokenStream {
    // TODO: build the output `TokenStream` with `quote!` (chapter 3). There's no
    //   error handling here — parsing already guaranteed `sm` is valid. The book's
    //   "What the macro generates" section shows the exact shape to aim for. Emit:
    //
    //     - A per-type error struct `<Name>InvalidTransition` with `from`/`to`
    //       `&'static str` fields and `#[derive(Debug)]`. Build its name with
    //       `format_ident!` so two derives in one module never clash (chapter 4,
    //       name clashes).
    //     - `Display` and `Error` impls for it. Use absolute paths
    //       (`::core::fmt::Display`, `::std::error::Error`, ...) so the output
    //       compiles in any context (chapter 4, absolute paths).
    //     - An `impl <Name>` block with four methods:
    //         - `initial()` returns the `#[initial]` state.
    //         - `name(&self)` maps each variant to its name as a `&'static str`
    //           (one match arm per `sm.variants` entry).
    //         - `can_transition_to(&self, target: &Self)` matches the *pair*
    //           `(self, target)` with one `true` arm per `sm.transitions` edge and
    //           a `_ => false` catch-all.
    //         - `transition_to(self, target: Self)` returning `Result<Self, _>`,
    //           delegating to `can_transition_to`.
    //
    //   Build the repeated match arms by iterating `sm.variants` / `sm.transitions`
    //   into `quote! { .. }` fragments, then splice them with `#(#arms)*` (chapter 3).
    let name = &sm.name;
    let initial = &sm.initial;
    let error_name = format_ident!("{}InvalidTransition", name);

    let name_arms = sm.variants.iter().map(|variant| {
        let variant_str = variant.to_string();
        quote! { #name::#variant => #variant_str, }
    });

    let transition_arms = sm.transitions.iter().map(|(from, to)| {
        quote! { (#name::#from, #name::#to) => true, }
    });

    quote! {
        #[derive(Debug)]
        pub struct #error_name {
            pub from: &'static str,
            pub to: &'static str,
        }

        #[automatically_derived]
        impl ::core::fmt::Display for #error_name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::write!(f, "invalid transition from {} to {}", self.from, self.to)
            }
        }

        #[automatically_derived]
        impl ::std::error::Error for #error_name {}

        #[automatically_derived]
        impl #name {
            pub fn initial() -> Self {
                #name::#initial
            }

            pub fn name(&self) -> &'static str {
                match self {
                    #(#name_arms)*
                }
            }

            pub fn can_transition_to(&self, target: &Self) -> bool {
                match (self, target) {
                    #(#transition_arms)*
                    _ => false,
                }
            }

            pub fn transition_to(self, target: Self) -> ::core::result::Result<Self, #error_name> {
                if self.can_transition_to(&target) {
                    ::core::result::Result::Ok(target)
                } else {
                    ::core::result::Result::Err(#error_name {
                        from: self.name(),
                        to: target.name(),
                    })
                }
            }
        }
    }
}
