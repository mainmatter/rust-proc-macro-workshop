#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    // Passing cases live in `examples/` so you can inspect the generated code
    // with `cargo expand --example <name>` (see the trybuild book chapter).
    t.pass("examples/*.rs");
    // The compile-fail snapshot is what actually proves the error lands on the
    // right *span* — a unit test on the message can't see where the `^^^^` points.
    t.compile_fail("tests/fail/*.rs");
}
