/// For each scenario, return whether a procedural macro is the right tool.
///
/// Return `true` if a proc macro is a good fit, `false` if another approach
/// (function, trait, generic, `macro_rules!`, or manual impl) is better.

/// Scenario: You have 30 struct types that each need a `to_json` method
/// which serializes all their fields. The implementation follows the same
/// pattern for every struct — think `serde::Serialize`.
pub fn scenario_serialize_30_structs() -> bool {
    true
}

/// Scenario: You want a helper function that retries an HTTP request
/// up to 3 times with exponential backoff.
pub fn scenario_retry_http() -> bool {
    false
}

/// Scenario: You want to write `assert_approx_eq!(a, b, epsilon)` that
/// works with any float type and gives a nice error message showing
/// both values.
pub fn scenario_assert_approx() -> bool {
    false
}

/// Scenario: You have two structs that need the same 5-line trait
/// implementation.
pub fn scenario_two_structs() -> bool {
    false
}

/// Scenario: You want a `#[derive(Validate)]` that reads `#[validate(...)]`
/// attributes on struct fields to generate validation logic (e.g. min/max
/// length, regex patterns).
pub fn scenario_derive_validate() -> bool {
    true
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn serialize_many_structs() {
        assert!(
            scenario_serialize_30_structs(),
            "The macro needs to inspect each struct's fields to generate \
             serialization code. Only a proc macro can look at a struct definition \
             — a function or `macro_rules!` can't."
        );
    }

    #[test]
    fn retry_http() {
        assert!(
            !scenario_retry_http(),
            "This is pure runtime logic — no code generation, no syntax inspection. \
             A regular function handles this fine."
        );
    }

    #[test]
    fn assert_approx() {
        assert!(
            !scenario_assert_approx(),
            "This only needs to capture expressions and emit a formatted error message. \
             `macro_rules!` can do that — no need to parse Rust syntax or inspect types."
        );
    }

    #[test]
    fn two_structs() {
        assert!(
            !scenario_two_structs(),
            "Only two structs and five lines each. The setup cost of a proc macro crate \
             far exceeds just writing those ten lines."
        );
    }

    #[test]
    fn derive_validate() {
        assert!(
            scenario_derive_validate(),
            "The macro needs to read custom attributes on individual fields and generate \
             different validation code per field. This requires parsing the struct \
             definition and its attributes — exactly what proc macros are for."
        );
    }
}
