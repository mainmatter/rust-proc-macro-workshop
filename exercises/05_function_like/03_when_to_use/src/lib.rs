//! You've now seen *both* kinds of function-like macro:
//!
//! - **declarative** (`macro_rules!`) — pattern-match token templates, no extra crate.
//! - **procedural** function-like (`#[proc_macro]`) — a real Rust function that parses
//!   the input tokens however it likes.
//!
//! A procedural macro is strictly more powerful, but it costs you a separate
//! `proc-macro` crate and a full hand-written parser. Reach for it only when
//! `macro_rules!` genuinely can't do the job — typically when you need to parse syntax
//! that *isn't* a simple token template, or do real compile-time work on the input.
//!
//! For each scenario, return `true` if a **procedural function-like macro** is the
//! right tool, or `false` if `macro_rules!` (or a plain function) would do.

/// Scenario: a `max!(a, b, c, ...)` macro that expands to a chain of
/// `std::cmp::max` calls over its comma-separated arguments.
pub fn scenario_max() -> bool {
    false
}

/// Scenario: an `html! { <ul><li>{item}</li></ul> }` macro that lets you write
/// HTML-like markup — angle brackets, tags, and `{ ... }` interpolation — directly
/// in Rust source.
pub fn scenario_html_dsl() -> bool {
    true
}

/// Scenario: a `sql!("SELECT name FROM users WHERE id = ?")` macro that connects to
/// your real database at compile time, checks the query against the live schema, and
/// generates a strongly-typed row struct.
pub fn scenario_checked_sql() -> bool {
    true
}

/// Scenario: a `hashset!(a, b, c)` macro that inserts its comma-separated arguments
/// into a fresh `HashSet`.
pub fn scenario_hashset() -> bool {
    false
}

/// Scenario: a `clamp(value, lo, hi)` helper that returns `value` bounded to the
/// `lo..=hi` range.
pub fn scenario_clamp() -> bool {
    false
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn max() {
        assert!(
            !scenario_max(),
            "The input is just a comma-separated list of expressions and the output is \
             a fixed template — `macro_rules!` matches `$($e:expr),+` and folds them. \
             No custom parsing or compile-time work, so a proc macro is overkill."
        );
    }

    #[test]
    fn html_dsl() {
        assert!(
            scenario_html_dsl(),
            "`<ul>` and `</ul>` are not valid Rust expressions, so `macro_rules!` can't \
             match them. You need to parse a custom grammar from raw tokens — exactly what \
             a procedural function-like macro is for."
        );
    }

    #[test]
    fn checked_sql() {
        assert!(
            scenario_checked_sql(),
            "Validating a query against a live database is real compile-time work that \
             only arbitrary Rust code — a proc macro — can perform. `macro_rules!` can \
             only shuffle tokens."
        );
    }

    #[test]
    fn hashset() {
        assert!(
            !scenario_hashset(),
            "Same shape as `vec!`: a comma-separated list expanded into `insert` calls. \
             `macro_rules!` handles this with a single repetition rule."
        );
    }

    #[test]
    fn clamp() {
        assert!(
            !scenario_clamp(),
            "This is plain runtime logic over values — no code generation and no syntax \
             inspection. A regular function is the right tool; it's not even a macro."
        );
    }
}
