/// Use `derive_more` to derive `Display` for this enum.
/// Each variant should display as specified in the tests.
///
/// This is a quick warm-up to remind you what derive macros look like
/// from the user's perspective — you'll be building your own from
/// the next section onwards.

#[derive(Debug, derive_more::Display)]
pub enum Shape {
    #[display("circle with radius {radius}")]
    Circle { radius: f64 },
    #[display("rectangle {width}x{height}")]
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
