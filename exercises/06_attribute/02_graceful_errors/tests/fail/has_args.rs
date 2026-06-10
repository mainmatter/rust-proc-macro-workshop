use attr_graceful_exercise::describe;

// This function takes an argument, so `#[describe]` rejects it.
#[describe]
fn greet(name: &str) -> String {
    format!("hello {name}")
}

fn main() {
    // Because the macro re-emits `greet` even when it errors, this call still
    // resolves — so the ONLY error reported is the macro's own message, with no
    // misleading "cannot find function `greet`" cascade.
    let _ = greet("world");
}
