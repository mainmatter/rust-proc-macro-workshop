/// These tasks can only be done with macros, not regular functions.
/// Use the right standard library macro for each one.

/// Use a macro to build a string by concatenating several values with a separator.
/// A function like `join(a, b, c)` can't accept a variable number of arguments in Rust.
///
/// Hint: look at the `format!` macro.
pub fn build_greeting(name: &str, age: u32) -> String {
    // TODO: interpolate `name` and `age` into the greeting (see the test for the exact wording).
    todo!()
}

/// Use a macro to get the value of an environment variable at **compile time**.
/// A function can only read environment variables at runtime.
///
/// Hint: look at the `env!` macro.
pub fn package_name() -> &'static str {
    // TODO: use `env!` to return the value of the "CARGO_PKG_NAME" variable,
    //       which is set by Cargo at compile time.
    todo!()
}

/// Use a macro to embed the contents of a file as a `&'static str` at compile time.
/// A function would have to read the file at runtime and return a `String`.
///
/// Hint: look at the `include_str!` macro.
pub fn license_text() -> &'static str {
    // TODO: embed "LICENSE.txt". Note the path is relative to this source file.
    todo!()
}

/// Use a macro to concatenate string literals at compile time.
/// A function can only concatenate strings at runtime, producing a `String`.
///
/// Hint: look at the `concat!` macro.
pub fn version_tag() -> &'static str {
    // TODO: build a "v"-prefixed version tag (e.g. "v0.1.0") by concatenating a "v"
    //       prefix with this crate's version. `env!` can give you the version that
    //       Cargo defines at compile time; `concat!` can join the pieces.
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn greeting() {
        assert_eq!(
            build_greeting("Ada", 36),
            "Hello, Ada! You are 36 years old."
        );
    }

    #[test]
    fn pkg_name() {
        assert_eq!(package_name(), "macros-vs-functions");
    }

    #[test]
    fn license() {
        let text = license_text();
        assert!(text.contains("Mainmatter"));
    }

    #[test]
    fn version() {
        assert_eq!(version_tag(), "v0.1.0");
    }
}
