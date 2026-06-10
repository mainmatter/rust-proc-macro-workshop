// TODO:
//   Write trybuild tests for the `FieldNames` derive macro.
//
//   1. Add a passing test: create `examples/named_struct.rs` with a named struct
//      that derives `FieldNames` and verifies the generated `field_names()` method
//      in `fn main()`. (Passing cases go under `examples/`, not `tests/pass/`, so
//      you can run `cargo expand --example named_struct` to see the macro output.)
//   2. Add a compile-fail test: create `tests/fail/enum.rs` that tries to use
//      `#[derive(FieldNames)]` on an enum. Then generate the `.stderr` snapshot by
//      running: TRYBUILD=overwrite cargo test --package trybuild-exercise

#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    // Passing cases live in `examples/` so you can inspect the generated code
    // with `cargo expand --example <name>` (see the trybuild book chapter).
    t.pass("examples/*.rs");
    t.compile_fail("tests/fail/*.rs");
}
