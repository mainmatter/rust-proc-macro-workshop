use quote_exercise_macros::FieldNames;

#[derive(FieldNames)]
struct Empty {}

#[derive(FieldNames)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

#[derive(FieldNames)]
struct Config {
    host: String,
    port: u16,
    verbose: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn empty_fields() {
        assert_eq!(Empty::field_names(), &[] as &[&str]);
    }

    #[test]
    fn color_fields() {
        assert_eq!(Color::field_names(), &["r", "g", "b"]);
    }

    #[test]
    fn config_fields() {
        assert_eq!(Config::field_names(), &["host", "port", "verbose"]);
    }
}
