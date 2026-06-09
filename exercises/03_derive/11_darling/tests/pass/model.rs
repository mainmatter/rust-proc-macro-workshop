use darling_exercise::Model;

#[derive(Model)]
#[model(table = "users")]
struct User {
    id: u64,
    #[model(rename = "email_address")]
    email: String,
    name: String,
}

#[derive(Model)]
struct Item {
    sku: String,
}

fn main() {
    assert_eq!(User::table_name(), "users");
    assert_eq!(User::columns(), vec!["id", "email_address", "name"]);

    // No `#[model(table = ..)]` -> the default table name.
    assert_eq!(Item::table_name(), "items");
    assert_eq!(Item::columns(), vec!["sku"]);
}
