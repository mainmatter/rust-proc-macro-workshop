- Welcome

- Introduction
  - The three types of procedural macros
  - Exploring macros with cargo expand
  - thiserror: a derive macro in the wild
  - Macros vs functions
  - Declarative vs procedural macros
  - When to use procedural macros

- The proc-macro toolkit
  - Crate structure (create a proc-macro crate, wire it up)
  - The re-export pattern (facade crate, e.g. serde + serde_derive)
  - What a proc macro returns (valid TokenStream)
  - A minimal derive macro
  - Token and TokenStream
  - proc-macro2
  - Parsing Rust code with syn
  - Generating code with quote
  - Testing with trybuild

- Derive macros
  - serde: container and field attributes
  - Handling struct fields
  - Handling enums
  - Using absolute paths: `::std::option::Option`
  - Preventing name clashes in generated symbols
  - Error reporting: panic vs compile_error!
  - Error reporting: syn::Error and Span
  - Spans for better error messages
  - Testing error messages with trybuild (using trybuild for error snapshots)
  - Container attributes
  - Field attributes
  - Parsing attributes with Darling
  - Handling generics (type parameters, lifetimes, where clauses)
  - Building a #[derive(Builder)]

- Function-like macros
  - Overview: why is println! a macro? (json! warm-up)
  - Parsing arbitrary token input
  - Custom syntax
  - When to use function-like over declarative macros
  - Building a simple DSL (routes!)

- Attribute macros
  - A minimal attribute macro
  - Transforming a function (e.g. adding a timing/logging wrapper)
  - Graceful error handling: re-emitting the original item to avoid cascading errors
  - Parsing attribute arguments with Darling's FromMeta
  - Building a #[retry] attribute: #[retry(times = 3, delay_ms = 100)] — wraps a function to retry on error

- Putting it all together
  - Building a state machine macro: derive-style code generation (enum + methods), custom attributes (transitions, initial state), error reporting
    (invalid transitions), hygiene (generated identifiers), and possibly even custom syntax?
