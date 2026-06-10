use syn_exercise_macros::FieldCount;

#[derive(FieldCount)]
struct Empty {}

#[derive(FieldCount)]
struct Point {
    x: f64,
    y: f64,
}

#[derive(FieldCount)]
struct User {
    name: String,
    email: String,
    age: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_fields() {
        assert_eq!(Empty::field_count(), 0);
    }

    #[test]
    fn point_fields() {
        assert_eq!(Point::field_count(), 2);
    }

    #[test]
    fn user_fields() {
        assert_eq!(User::field_count(), 3);
    }
}
