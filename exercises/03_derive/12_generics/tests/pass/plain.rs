use generics_exercise::Empty;

#[derive(Empty)]
struct Plain {
    count: u32,
    name: String,
}

fn main() {
    let p = Plain::empty();
    assert_eq!(p.count, 0);
    assert_eq!(p.name, "");
}
