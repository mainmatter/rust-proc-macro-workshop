use abs_paths_exercise::TypeName;
use pretty_assertions::assert_eq;

// A hostile (or just unlucky) user shadows `String` in their own module.
// The generated code must still compile and behave correctly, which means it
// cannot rely on `String` referring to `std::string::String` here.
#[allow(dead_code)]
type String = ();

#[derive(TypeName)]
struct Widget;

fn main() {
    // Spell out the real type explicitly — `String` means `()` in this module.
    let name: std::string::String = Widget.type_name();
    assert_eq!(name, "Widget");
}
