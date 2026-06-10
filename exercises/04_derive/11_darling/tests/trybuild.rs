#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    // Passing cases live in `examples/` so you can inspect the generated code
    // with `cargo expand --example <name>` (see the trybuild book chapter).
    t.pass("examples/*.rs");
    // `darling` generates the attribute validation; these snapshots capture the
    // diagnostics it produces for misuse (an unknown key, an unsupported shape).
    t.compile_fail("tests/fail/*.rs");
}
