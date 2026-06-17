/// Use `derive_more` to derive `Display` for this enum.
/// Each variant should display as specified in the tests.
///
/// This is a quick warm-up to remind you what derive macros look like
/// from the user's perspective — you'll be building your own from
/// the next section onwards.

// TODO: Derive `derive_more::Display` and add a `#[display("...")]` attribute to
//       each variant. The format string can reference the variant's fields by name,
//       e.g. `{radius}` (see the tests for the exact wording each variant should produce).
#[derive(Debug)]
pub enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn circle() {
        let s = Shape::Circle { radius: 2.5 };
        assert_eq!(s.to_string(), "circle with radius 2.5");
    }

    #[test]
    fn rectangle() {
        let s = Shape::Rectangle {
            width: 3.0,
            height: 4.0,
        };
        assert_eq!(s.to_string(), "rectangle 3x4");
    }
}
