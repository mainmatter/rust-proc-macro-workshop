//! Warm-up: drive `serde`'s derive macros with *attributes*.
//!
//! `serde` exposes two kinds of attributes:
//!
//! - **container attributes** sit on the struct/enum (e.g. `#[serde(rename_all = ...)]`)
//! - **field attributes** sit on individual fields (e.g. `#[serde(rename = ...)]`)
//!
//! Add the attributes needed to make the tests pass. You are *using* a derive macro
//! here, not writing one — from the next section onwards you'll build your own.

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub first_name: String,
    pub last_name: String,

    #[serde(rename = "email")]
    pub email_address: String,

    #[serde(default)]
    pub is_admin: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn serializes_in_camel_case() {
        let user = User {
            first_name: "Ada".into(),
            last_name: "Lovelace".into(),
            email_address: "ada@example.com".into(),
            is_admin: true,
        };

        let json = serde_json::to_string(&user).unwrap();

        // The container attribute renames fields to camelCase...
        assert!(json.contains("\"firstName\""), "got: {json}");
        assert!(json.contains("\"lastName\""), "got: {json}");
        // ...and the field attribute renames this one specifically.
        assert!(json.contains("\"email\""), "got: {json}");
        assert!(!json.contains("emailAddress"), "got: {json}");
    }

    #[test]
    fn deserializes_with_default() {
        // `isAdmin` is missing from the input, so it should fall back to `false`.
        let json = r#"{
            "firstName": "Grace",
            "lastName": "Hopper",
            "email": "grace@example.com"
        }"#;

        let user: User = serde_json::from_str(json).unwrap();

        assert_eq!(user.first_name, "Grace");
        assert_eq!(user.email_address, "grace@example.com");
        assert!(!user.is_admin);
    }
}
