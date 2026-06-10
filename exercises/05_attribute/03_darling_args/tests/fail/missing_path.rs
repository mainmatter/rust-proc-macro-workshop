use attr_args_exercise::endpoint;

// `path` is required but missing, so darling reports a clear error.
#[endpoint(method = "POST")]
fn handler() {}

fn main() {}
