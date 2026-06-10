# Putting it all together

You've now built all three kinds of procedural macro: a `#[derive(Builder)]` (chapter 4), a
`routes!` DSL (chapter 5), and a `#[retry]` attribute (chapter 6). Each chapter ended with a
capstone that combined that chapter's techniques. This final chapter combines the techniques from
across the _whole workshop_ into one macro: `#[derive(StateMachine)]`.

A state machine is a perfect fit for a derive macro. The states are the variants of an enum, the
allowed transitions are written as attributes, and the macro turns that declaration into the methods
that enforce the rules at runtime:

```rust
#[derive(StateMachine)]
enum TrafficLight {
    #[initial]
    #[transition(Green)]
    Red,
    #[transition(Yellow)]
    Green,
    #[transition(Red)]
    Yellow,
}
```

That declaration reads like a specification — _start at `Red`; `Red` may become `Green`; `Green` may
become `Yellow`; `Yellow` may become `Red`_ — and the macro makes it executable:

```rust
let light = TrafficLight::initial();              // Red
assert!(light.can_transition_to(&TrafficLight::Green));

let next = light.transition_to(TrafficLight::Green).unwrap();   // Green
let bad = next.transition_to(TrafficLight::Red);               // Err: not allowed
```

## What the macro generates

For `TrafficLight`, `#[derive(StateMachine)]` emits roughly this (shown with bare paths for
readability — the real output uses absolute paths, as the bullets below explain):

```rust
#[derive(Debug)]
pub struct TrafficLightInvalidTransition {
    from: &'static str,
    to: &'static str,
}
// + Display / Error impls, so it reads as "invalid transition from Red to Yellow"

impl TrafficLight {
    pub fn initial() -> Self {
        TrafficLight::Red
    }

    pub fn name(&self) -> &'static str {
        match self {
            TrafficLight::Red => "Red",
            TrafficLight::Green => "Green",
            TrafficLight::Yellow => "Yellow",
        }
    }

    pub fn can_transition_to(&self, target: &Self) -> bool {
        match (self, target) {
            (TrafficLight::Red, TrafficLight::Green) => true,
            (TrafficLight::Green, TrafficLight::Yellow) => true,
            (TrafficLight::Yellow, TrafficLight::Red) => true,
            _ => false,
        }
    }

    pub fn transition_to(self, target: Self) -> Result<Self, TrafficLightInvalidTransition> {
        if self.can_transition_to(&target) {
            Ok(target)
        } else {
            Err(TrafficLightInvalidTransition { from: self.name(), to: target.name() })
        }
    }
}
```

This one macro draws on nearly everything you've learned:

- **Per-variant code generation** — iterate the enum's variants and emit a `name()` arm for each
  and a `can_transition_to` arm for each declared edge (chapter 4, _Handling enums_).
- **Custom helper attributes** — `#[initial]` and `#[transition(..)]` are declared with
  `#[proc_macro_derive(StateMachine, attributes(initial, transition))]` and read off each variant
  (chapter 4, _Container / field attributes_).
- **Error reporting** — a `#[transition(Purple)]` that names a state which doesn't exist is rejected
  at compile time, with the error spanned on `Purple` itself; so is a missing or duplicated
  `#[initial]`, a non-enum, or a variant that carries data (chapter 4, _`syn::Error` and `Span`_).
- **A generated identifier** — the per-type `TrafficLightInvalidTransition` is built with
  `format_ident!`, so two derives in the same module never clash (chapter 4, _Preventing name
  clashes_), and the generated impls use absolute paths like `::core::result::Result` and
  `::core::fmt::Display` so they compile in any context (chapter 4, _Using absolute paths_).

## The shape of the solution

The work splits cleanly into two halves — _understand the input_, then _emit the output_ — and the
skeleton you're given is organised that way:

1. `parse_state_machine` validates the `DeriveInput` and distils it into a plain
   `struct StateMachine { name, variants, initial, transitions }`. All the rules live here: it must
   be an enum, every variant must be a unit variant, exactly one must be `#[initial]`, and every
   `#[transition(..)]` target must be a real state.
2. `generate` takes that validated `StateMachine` and produces the `TokenStream` — it never has to
   worry about bad input, because step 1 already rejected it.

Splitting "validate" from "generate" like this is how most non-trivial macros are structured: by the
time you're calling `quote!`, every value you splice in is known to be good.

## A couple of things worth noticing

**`match (self, target)`.** `can_transition_to` matches on the _pair_ of states, with one arm per
declared edge and a `_ => false` catch-all. Because the states are unit variants, the patterns are
just `(Enum::A, Enum::B)` — no payload to bind — and you never need the enum to implement
`PartialEq`.

**Validate against the variant list.** To reject `#[transition(Purple)]`, the macro first collects
every variant name, _then_ checks each transition target against that list. That ordering matters:
you can't know whether `Purple` is valid until you've seen all the states. It's the same
"syntactic, not semantic" check you met with the builder's `Option<T>` detection — the macro only
ever compares identifiers, never resolved types.

## Exercise

This is the capstone, so you write the macro itself — not just one corner of it. Only the boilerplate
is provided: the `#[proc_macro_derive]` entry point, the `StateMachine` struct that the two halves
pass between them, and the one-line glue that wires them together. Both halves are `todo!()`:

1. **`parse_state_machine`** — validate the `DeriveInput` and distil it into a `StateMachine`. Every
   broken rule is a spanned `syn::Error`: it must be an enum, every variant must be a unit variant,
   exactly one must be `#[initial]`, and every `#[transition(..)]` target must name a real state. You
   read the `#[transition(..)]` arguments with `parse_args_with` and a `Punctuated` list, exactly as
   in chapters 4–5.
2. **`generate`** — turn that validated `StateMachine` into the `TokenStream` shown in
   _What the macro generates_ above: the per-type `..InvalidTransition` error (named with
   `format_ident!`), its `Display`/`Error` impls written with absolute paths, and the four methods,
   their match arms built by iterating the variants and edges.

The inline `TODO`s spell out each rule and which API to reach for. Make `examples/traffic_light.rs`
pass and every file in `tests/fail/` produce its expected compiler error.

> **Going further.** A state machine is fertile ground for extensions, each a small reuse of a
> technique from the workshop: emit a `transitions()` method listing every reachable state; reject a
> machine where some state is unreachable from `#[initial]`; add an `#[on_enter]` hook; or design a
> richer DSL — `state_machine! { Red -> Green -> Yellow -> Red }` as a _function-like_ macro
> (chapter 5) instead of a derive. With all three macro kinds behind you, you have the tools for any
> of them. Congratulations — you've finished the workshop.
