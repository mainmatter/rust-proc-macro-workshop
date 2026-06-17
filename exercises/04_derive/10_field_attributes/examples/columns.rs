use field_attr_exercise::Renamed;
use pretty_assertions::assert_eq;

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
