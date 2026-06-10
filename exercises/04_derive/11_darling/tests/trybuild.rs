#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/pass/*.rs");
    // `darling` generates the attribute validation; these snapshots capture the
    // diagnostics it produces for misuse (an unknown key, an unsupported shape).
    t.compile_fail("tests/fail/*.rs");
}
