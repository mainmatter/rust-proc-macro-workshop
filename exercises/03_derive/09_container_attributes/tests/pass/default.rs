use container_attr_exercise::Repeat;

#[derive(Repeat)]
struct Solo;

fn main() {
    // No `#[repeat(..)]` -> defaults to one copy.
    assert_eq!(Solo::repeated(), "Solo");
}
