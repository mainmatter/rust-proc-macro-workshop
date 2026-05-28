# Summary

- [Welcome](00_intro/00_welcome.md)

- [Introduction](01_introduction/00_overview.md)
  - [Exploring macros with `cargo expand`](01_introduction/01_cargo_expand.md)
  - [`thiserror`: a derive macro in the wild](01_introduction/02_thiserror.md)
  - [Macros vs functions](01_introduction/03_macros_vs_functions.md)
  - [Declarative vs procedural macros](01_introduction/04_declarative_vs_procedural.md)
  - [When to use procedural macros](01_introduction/05_when_to_use.md)

- [The proc-macro toolkit](02_basics/00_overview.md)
  - [Crate structure](02_basics/01_crate_structure.md)
  - [The re-export pattern](02_basics/02_re_export_pattern.md)
  - [What a proc macro returns](02_basics/03_what_a_proc_macro_returns.md)
  - [A minimal derive macro](02_basics/04_minimal_derive.md)
  - [Token and TokenStream](02_basics/05_token_and_tokenstream.md)
  - [`proc-macro2`](02_basics/06_proc_macro2.md)
  - [Parsing Rust code with `syn`](02_basics/07_syn.md)
  - [Generating code with `quote`](02_basics/08_quote.md)
  - [Testing with `trybuild`](02_basics/09_trybuild.md)
