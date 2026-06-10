use trybuild_exercise::FieldNames;

#[derive(FieldNames)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    assert_eq!(Point::field_names(), &["x", "y"]);
}
