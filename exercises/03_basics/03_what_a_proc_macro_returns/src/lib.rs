use proc_macro_returns_macros::TypeName;

#[derive(TypeName)]
struct Greeting;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_type_name() {
        assert_eq!(Greeting::type_name(), "Greeting");
    }
}
