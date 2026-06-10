// TODO: Wire up the facade pattern so that both the `SelfDescribe` trait and the
//       `Describe` derive macro can be imported from this single crate. The macro
//       already lives in the `re-export-exercise-macros/` crate.

pub use re_export_exercise_macros::Describe;

pub trait SelfDescribe {
    fn describe(&self) -> &'static str;
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

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
