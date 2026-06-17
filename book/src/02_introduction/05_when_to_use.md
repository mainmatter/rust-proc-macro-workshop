# When to use procedural macros

Procedural macros are powerful, but they come with costs. Before reaching for one, it's worth
asking: is a proc macro the right tool here?

## Good reasons to use a proc macro

### Eliminating boilerplate

The strongest use case. If you find yourself writing the same trait implementation for dozens
of types, with only the field names changing, a derive macro can generate all of it from the
struct definition alone.

Examples: [`serde::Serialize`](https://docs.rs/serde/latest/serde/derive.Serialize.html), [`thiserror::Error`](https://docs.rs/thiserror/latest/thiserror/derive.Error.html), [`clap::Parser`](https://docs.rs/clap/latest/clap/trait.Parser.html).

### Enforcing patterns at compile time

A proc macro can reject invalid usage with a clear error message before the program ever runs.
For example, a `#[derive(Builder)]` macro can check that all required fields are set, or a
routing macro can verify that path parameters match function arguments.

### Providing ergonomic APIs

When the "natural" way to express something in Rust is verbose or awkward, a macro can provide
a cleaner interface. [`sqlx::query!`](https://docs.rs/sqlx/latest/sqlx/macro.query.html) lets you write SQL directly and get type-checked results.
[`tokio::main`](https://docs.rs/tokio/latest/tokio/attr.main.html) lets you write `async fn main()` without the runtime setup boilerplate.

## Bad reasons to use a proc macro

### "It's clever"

If a function, a trait, a generic, or a `macro_rules!` macro can do the job, prefer those.
They are easier to write, easier to maintain, and easier for users to understand.

### Hiding complexity

Macros that do too much behind the scenes become a maintenance burden. Users can't step through
generated code in a debugger. Error messages point to the macro invocation, not the generated
code. IDE support is limited.

### Small scale

If you only have two or three types that need the same implementation, writing it by hand is
fine. The setup cost of a proc macro crate (separate crate, parsing, code generation) isn't
worth it for a handful of types.

## The cost of proc macros

- **Compile time**: a proc macro must be compiled as a standalone binary that the compiler runs
  during your build, which adds linking and code-generation overhead you don't pay for a "normal"
  library dependency.
- **Debuggability**: generated code is harder to debug than hand-written code.
- **Error messages**: getting good error messages from macros takes significant effort.
- **Maintenance**: macro code is harder to read and maintain than regular Rust code.

## Exercise

Given a set of scenarios, identify which ones would benefit from a procedural macro and which
ones are better solved with other tools.
