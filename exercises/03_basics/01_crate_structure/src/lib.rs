// The code below won't compile until you create the macro crate it depends on.
//
// TODO:
//   1. Create a proc-macro crate in the `crate-structure-exercise-macros/` subdirectory,
//      with its own unique package name and an `Empty` derive macro that returns an empty
//      `TokenStream`.
//   2. Add it as a path dependency in this crate's `Cargo.toml`.

use crate_structure_exercise_macros::Empty;

#[derive(Empty)]
struct Foo;

#[derive(Empty)]
struct Bar {
    x: i32,
    y: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn foo_exists() {
        let _ = Foo;
    }

    #[test]
    fn bar_exists() {
        let _ = Bar { x: 1, y: 2 };
    }
}
