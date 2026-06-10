// TODO:
//   Two near-empty compile-fail files are scaffolded under `tests/fail/` (just an
//   import and `fn main`). Fill each with a small program that misuses `Getters`
//   so it fails to compile:
//     - `on_enum.rs`    — derive `Getters` on an enum
//     - `unit_field.rs` — derive `Getters` on a struct with a `()`-typed field
//   Then capture the diagnostics by running:
//
//     TRYBUILD=overwrite cargo test --package trybuild-errors-exercise
//
//   and review the generated `.stderr` files (e.g. with `git diff`): the enum
//   error should underline the whole item, and the `()` case should underline
//   just the field. Those snapshots lock in the diagnostics the macro produces,
//   so a later change to a message or span fails the test.

#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    // Passing cases live in `examples/` so you can inspect the generated code
    // with `cargo expand --example <name>` (see the trybuild book chapter).
    t.pass("examples/*.rs");
    t.compile_fail("tests/fail/*.rs");
}
