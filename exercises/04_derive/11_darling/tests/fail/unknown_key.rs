use darling_exercise::Model;

// `darling` validates the attribute schema for you: `renmae` is a typo of
// `rename`, and the generated `FromField` parser rejects the unknown key with a
// precise, well-spanned error — no hand-written validation in the macro.
#[derive(Model)]
struct User {
    #[model(renmae = "email_address")]
    email: String,
}

fn main() {}
