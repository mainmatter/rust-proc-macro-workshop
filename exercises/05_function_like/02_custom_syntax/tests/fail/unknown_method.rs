use custom_syntax_exercise::methods;

fn main() {
    // `delete` is not one of the keywords the DSL understands.
    let _ = methods! {
        delete "/users",
    };
}
