use routes_exercise::routes;

fn index() -> String {
    "the index".to_string()
}

fn about() -> String {
    "about us".to_string()
}

fn create_user() -> String {
    "user created".to_string()
}

fn main() {
    let router = routes! {
        GET "/" => index,
        GET "/about" => about,
        POST "/users" => create_user,
    };

    // Matching routes dispatch to their handler.
    assert_eq!(router("GET", "/"), Some("the index".to_string()));
    assert_eq!(router("GET", "/about"), Some("about us".to_string()));
    assert_eq!(router("POST", "/users"), Some("user created".to_string()));

    // Both the method and the path have to match.
    assert_eq!(router("POST", "/"), None);
    assert_eq!(router("GET", "/users"), None);
    assert_eq!(router("GET", "/missing"), None);
    assert_eq!(router("DELETE", "/"), None);
}
