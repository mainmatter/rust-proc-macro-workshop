use struct_fields_exercise::DebugFields;

#[derive(DebugFields)]
struct Marker;

fn main() {
    let m = Marker;
    assert!(m.debug_fields().is_empty());
}
