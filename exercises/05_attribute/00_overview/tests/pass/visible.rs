// `secret` is declared *without* `pub` inside a private module. The only way the
// call in `main` can compile is if `#[make_public]` rewrote it to `pub fn`.
mod inner {
    use attr_minimal_exercise::make_public;

    #[make_public]
    fn secret() -> u32 {
        42
    }
}

fn main() {
    assert_eq!(inner::secret(), 42);
}
