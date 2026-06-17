use attr_args_exercise::endpoint;
use pretty_assertions::assert_eq;

#[endpoint(path = "/users", method = "POST")]
fn create() {}

// `method` is omitted, so it falls back to the "GET" default.
#[endpoint(path = "/")]
fn index() {}

fn main() {
    assert_eq!(create_path(), "/users");
    assert_eq!(create_method(), "POST");

    assert_eq!(index_path(), "/");
    assert_eq!(index_method(), "GET");
}
