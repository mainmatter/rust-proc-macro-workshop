use abs_paths_exercise::TypeName;

#[derive(TypeName)]
struct Widget;

fn main() {
    assert_eq!(Widget.type_name(), "Widget");
}
