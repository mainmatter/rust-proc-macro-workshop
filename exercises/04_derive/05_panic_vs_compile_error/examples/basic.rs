use compile_error_exercise::Getters;
use pretty_assertions::assert_eq;

#[derive(Getters)]
struct User {
    name: String,
    age: u32,
}

fn main() {
    let user = User {
        name: "Ada".to_string(),
        age: 36,
    };
    assert_eq!(user.name(), "Ada");
    assert_eq!(*user.age(), 36);
}
