use container_attr_exercise::Repeat;
use pretty_assertions::assert_eq;

#[derive(Repeat)]
struct Solo;

fn main() {
    // No `#[repeat(..)]` -> defaults to one copy.
    assert_eq!(Solo::repeated(), "Solo");
}
