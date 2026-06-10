use minimal_derive_macros::TypeName;

#[derive(TypeName)]
struct Foo;

#[derive(TypeName)]
struct Bar {
    _x: i32,
}

#[derive(TypeName)]
struct Baz(u8, u8);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn foo_type_name() {
        assert_eq!(Foo::type_name(), "Foo");
    }

    #[test]
    fn bar_type_name() {
        assert_eq!(Bar::type_name(), "Bar");
    }

    #[test]
    fn baz_type_name() {
        assert_eq!(Baz::type_name(), "Baz");
    }
}
