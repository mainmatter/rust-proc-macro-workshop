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
