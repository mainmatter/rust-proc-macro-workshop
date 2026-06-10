//! Warm-up: drive a *function-like* macro from the caller's side.
//!
//! [`serde_json::json!`](https://docs.rs/serde_json/latest/serde_json/macro.json.html)
//! is a function-like macro. Instead of inspecting a type (like a derive macro) or
//! wrapping an item (like an attribute macro), it takes a chunk of *tokens* that look
//! like JSON and turns them into a `serde_json::Value` at compile time:
//!
//! ```ignore
//! let v = serde_json::json!({ "ok": true, "items": [1, 2, 3] });
//! ```
//!
//! Notice the input isn't ordinary Rust — `{ "ok": true }` is a little JSON *DSL* the
//! macro defines. That's the super-power of function-like macros, and it's exactly
//! what you'll build yourself by the end of this chapter.
//!
//! You're *using* a function-like macro here, not writing one — that starts in the
//! next section.

use serde_json::Value;

/// Build the JSON document the tests expect, using the `json!` macro.
pub fn user_document() -> Value {
    serde_json::json!({
        "name": "Ada",
        "age": 36,
        "languages": ["Rust", "Ada"],
        "active": true,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn has_the_expected_shape() {
        let doc = user_document();

        assert_eq!(doc["name"], "Ada");
        assert_eq!(doc["age"], 36);
        assert_eq!(doc["active"], true);

        let languages = doc["languages"].as_array().expect("languages is an array");
        assert_eq!(languages.len(), 2);
        assert_eq!(languages[0], "Rust");
        assert_eq!(languages[1], "Ada");
    }
}
