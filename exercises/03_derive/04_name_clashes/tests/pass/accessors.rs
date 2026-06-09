use name_clashes_exercise::Accessors;

#[derive(Accessors)]
struct Point {
    x: i32,
    y: i32,
}

// A second struct in the SAME module that also has a field named `x`. The
// generated accessor names must not collide — not between `Point`'s own fields,
// nor between `Point` and `Line` both having an `x`.
#[derive(Accessors)]
struct Line {
    x: i32,
}

fn main() {
    let p = Point { x: 1, y: 2 };
    assert_eq!(*p.x(), 1);
    assert_eq!(*p.y(), 2);

    let l = Line { x: 9 };
    assert_eq!(*l.x(), 9);
}
