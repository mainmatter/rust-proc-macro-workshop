/// In this exercise you'll use derive macros and a function-like macro.
/// We'll cover attribute macros later in the workshop.

/// 1. Derive macro: add the right `#[derive(...)]` attributes so that
///    `Point` can be printed with `{:?}` and cloned.
// TODO: add the right derive attributes
pub struct Point {
    pub x: f64,
    pub y: f64,
}

/// 2. Function-like macro: use `vec!` to create a vector containing
///    the numbers 1 through 5.
pub fn one_to_five() -> Vec<i32> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_is_debug() {
        let p = Point { x: 1.0, y: 2.0 };
        let debug = format!("{:?}", p);
        assert!(debug.contains("1.0"));
        assert!(debug.contains("2.0"));
    }

    #[test]
    fn point_is_clone() {
        let p = Point { x: 1.0, y: 2.0 };
        let p2 = p.clone();
        assert_eq!(p.x, p2.x);
        assert_eq!(p.y, p2.y);
    }

    #[test]
    fn vec_macro() {
        let v = one_to_five();
        assert_eq!(v, [1, 2, 3, 4, 5]);
    }
}
