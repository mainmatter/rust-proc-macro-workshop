#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/pass/*.rs");
    // The compile-fail snapshot is what actually proves the error lands on the
    // right *span* — a unit test on the message can't see where the `^^^^` points.
    t.compile_fail("tests/fail/*.rs");
}
