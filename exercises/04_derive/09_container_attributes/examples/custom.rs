use container_attr_exercise::Repeat;
use pretty_assertions::assert_eq;

#[derive(Repeat)]
#[repeat(times = 3)]
struct Na;

fn main() {
    assert_eq!(Na::repeated(), "NaNaNa");
}
