use pretty_assertions::assert_eq;
use struct_fields_exercise::DebugFields;

#[derive(DebugFields)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 1, y: 2 };
    assert_eq!(p.debug_fields(), vec!["1".to_string(), "2".to_string()]);
}
