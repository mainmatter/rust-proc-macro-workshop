use container_attr_exercise::Repeat;

#[derive(Repeat)]
#[repeat(times = 3)]
struct Na;

fn main() {
    assert_eq!(Na::repeated(), "NaNaNa");
}
