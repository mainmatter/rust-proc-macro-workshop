use pretty_assertions::assert_eq;
use struct_fields_exercise::DebugFields;

#[derive(DebugFields)]
struct Pair(i32, &'static str);

fn main() {
    let p = Pair(1, "hi");
    assert_eq!(
        p.debug_fields(),
        vec!["1".to_string(), "\"hi\"".to_string()]
    );
}
