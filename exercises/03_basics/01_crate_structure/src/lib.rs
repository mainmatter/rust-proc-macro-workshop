// TODO:
//   1. Create a proc-macro crate in the `macros/` subdirectory (next to this crate's
//      `Cargo.toml`). You'll need:
//      - `macros/Cargo.toml` — with `proc-macro = true` under `[lib]`.
//      - `macros/src/lib.rs` — define a derive macro named `Empty`
//        that returns an empty `TokenStream`.
//   2. Add it as a path dependency in this crate's `Cargo.toml`:
//        macros = { path = "macros" }
//   3. Uncomment the lines below.

// use macros::Empty;
//
// #[derive(Empty)]
// struct Foo;
//
// #[derive(Empty)]
// struct Bar {
//     x: i32,
//     y: i32,
// }

#[cfg(test)]
mod tests {
    // TODO: uncomment the tests below after creating the proc-macro crate.

    // use super::*;
    //
    // #[test]
    // fn foo_exists() {
    //     let _ = Foo;
    // }
    //
    // #[test]
    // fn bar_exists() {
    //     let _ = Bar { x: 1, y: 2 };
    // }
}
