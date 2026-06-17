// TODO: Re-export the `Describe` derive macro from the `re-export-exercise-macros`
//       crate so that users of this crate can write:
//
//           use re_export_exercise::Describe;
//           #[derive(Describe)]
//           struct Foo;
//
//       Steps:
//       1. Add `re-export-exercise-macros` as a dependency in Cargo.toml.
//       2. Add a `pub use` statement here to re-export the derive macro.

pub trait SelfDescribe {
    fn describe(&self) -> &'static str;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Describe)]
    struct Apple;

    #[derive(Describe)]
    struct Banana;

    #[test]
    fn apple_describe() {
        let a = Apple;
        assert_eq!(a.describe(), "Apple");
    }

    #[test]
    fn banana_describe() {
        let b = Banana;
        assert_eq!(b.describe(), "Banana");
    }
}
