use generics_exercise::Empty;
use pretty_assertions::assert_eq;

// A type parameter `T`: the generated code must add a `Default` bound on `T` and
// repeat `<T>` on the impl.
#[derive(Empty)]
struct Wrapper<T> {
    value: T,
    tag: String,
}

// A pre-existing bound must be preserved, not clobbered.
#[derive(Empty)]
struct Bounded<T: Clone> {
    first: T,
    second: T,
}

fn main() {
    let w: Wrapper<i32> = Wrapper::empty();
    assert_eq!(w.value, 0);
    assert_eq!(w.tag, "");

    let b: Bounded<u8> = Bounded::empty();
    assert_eq!(b.first, 0);
    assert_eq!(b.second, 0);
}
