use attr_graceful_exercise::describe;

#[describe]
fn ping() -> &'static str {
    "pong"
}

fn main() {
    // The original function is preserved...
    assert_eq!(ping(), "pong");
    // ...and the companion accessor was generated alongside it.
    assert_eq!(describe_ping(), "ping");
}
