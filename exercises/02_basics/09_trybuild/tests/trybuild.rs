// TODO:
//   Write trybuild tests for the `FieldNames` derive macro.
//
//   1. Add a passing test: create `tests/pass/named_struct.rs` with a named struct
//      that derives `FieldNames` and verifies the generated `field_names()` method
//      in `fn main()`.
//   2. Add a compile-fail test: create `tests/fail/enum.rs` that tries to use
//      `#[derive(FieldNames)]` on an enum. Then generate the `.stderr` snapshot by
//      running: TRYBUILD=overwrite cargo test --package trybuild-exercise

#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/pass/*.rs");
    t.compile_fail("tests/fail/*.rs");
}
