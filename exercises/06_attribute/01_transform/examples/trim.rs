use attr_transform_exercise::trimmed;
use pretty_assertions::assert_eq;

#[trimmed]
fn padded() -> String {
    "  hello  ".to_string()
}

#[trimmed]
fn greeting(name: &str) -> String {
    format!("\n{name}\t")
}

fn main() {
    // The macro left the signatures and arguments alone, but wrapped the body so
    // the returned String comes back trimmed.
    assert_eq!(padded(), "hello");
    assert_eq!(greeting("world"), "world");
}
