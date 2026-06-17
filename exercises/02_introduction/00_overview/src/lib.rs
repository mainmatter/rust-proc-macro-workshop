/// In this exercise you'll use derive macros and a function-like macro.
/// We'll cover attribute macros later in the workshop.

/// 1. Derive macros: add the right `#[derive(...)]` attributes so that two
///    `Version` values can be compared with `==` and ordered with `<`, `>`,
///    and so that a `Vec<Version>` can be `.sort()`ed.
///
///    That's four standard-library traits: two for equality and two for
///    ordering. The compiler will tell you if you forget one — sorting a
///    vector needs a total order, which builds on the others.
// TODO: add the right derive attributes
pub struct Version {
    pub major: u32,
    pub minor: u32,
}

/// 2. Function-like macro: use `vec!` to build the release history below,
///    in this order: 1.0, then 1.1, then 2.0.
pub fn release_history() -> Vec<Version> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn versions_compare_for_equality() {
        let a = Version { major: 1, minor: 0 };
        let b = Version { major: 1, minor: 0 };
        let c = Version { major: 2, minor: 0 };
        assert!(a == b);
        assert!(a != c);
    }

    #[test]
    fn versions_are_ordered() {
        let old = Version { major: 1, minor: 0 };
        let new = Version { major: 2, minor: 0 };
        assert!(old < new);
        assert!(new > old);
    }

    #[test]
    fn history_can_be_sorted() {
        let mut history = release_history();
        assert_eq!(history.len(), 3);
        // `.sort()` requires a total order (`Ord`), which in turn requires
        // the equality and partial-ordering traits.
        history.sort();
        assert!(history[0] < history[1]);
        assert!(history[1] < history[2]);
    }
}
