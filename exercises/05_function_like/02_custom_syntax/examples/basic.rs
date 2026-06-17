use custom_syntax_exercise::methods;
use pretty_assertions::assert_eq;

fn main() {
    let routes = methods! {
        get "/",
        post "/users",
        get "/users/:id",
    };

    assert_eq!(
        routes,
        vec![("GET", "/"), ("POST", "/users"), ("GET", "/users/:id")],
    );
}
