use abs_paths_exercise::TypeName;
use pretty_assertions::assert_eq;

#[derive(TypeName)]
struct Widget;

fn main() {
    assert_eq!(Widget.type_name(), "Widget");
}
