use routes_exercise::routes;

fn index() -> String {
    "the index".to_string()
}

fn main() {
    // The `=>` between the path and the handler is required by the grammar; leaving
    // it out is a clean parse error.
    let _ = routes! {
        GET "/" index,
    };
}
