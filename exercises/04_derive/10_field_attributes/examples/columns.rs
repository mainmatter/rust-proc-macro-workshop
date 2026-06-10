use field_attr_exercise::Renamed;

#[derive(Renamed)]
struct Row {
    id: u64,
    #[rename = "full_name"]
    name: String,
    #[rename = "email_address"]
    email: String,
}

fn main() {
    assert_eq!(
        Row::column_names(),
        vec!["id", "full_name", "email_address"]
    );
}
