use enums_exercise::Ordinal;
use pretty_assertions::assert_eq;

#[derive(Ordinal)]
enum Shape {
    Circle(f64),
    Rectangle { w: f64, h: f64 },
    Point,
}

fn main() {
    assert_eq!(Shape::Circle(1.0).ordinal(), 0);
    assert_eq!(Shape::Rectangle { w: 1.0, h: 2.0 }.ordinal(), 1);
    assert_eq!(Shape::Point.ordinal(), 2);
}
