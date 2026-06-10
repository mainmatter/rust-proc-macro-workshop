#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    // Passing cases live in `examples/` so you can inspect the generated code
    // with `cargo expand --example <name>` (see the trybuild book chapter).
    t.pass("examples/*.rs");
}
