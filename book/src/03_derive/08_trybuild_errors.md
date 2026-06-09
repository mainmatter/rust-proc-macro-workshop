# Testing error messages with `trybuild`

You spent three sections making `Getters` fail _gracefully_ — real diagnostics, good spans. But
how do you know the error a user sees is actually the one you intended? You can't see it from a
unit test: asserting that `getters_impl` returns an `Err` tells you _that_ it failed, not _what
the compiler prints_ or _where it points_. For that you need to compile the broken code for real
and capture the output — which is exactly what `trybuild`'s `compile_fail` mode does.

## `compile_fail` and `.stderr` snapshots

You met `trybuild` in chapter 2. The other half of its API is `compile_fail`:

```rust
#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/pass/*.rs");
    t.compile_fail("tests/fail/*.rs");
}
```

Each file under `tests/fail/` must **fail** to compile, and its diagnostic must match a sibling
`.stderr` snapshot:

```text
tests/fail/
├── on_enum.rs
├── on_enum.stderr
├── unit_field.rs
└── unit_field.stderr
```

The `.stderr` file is the captured compiler output — message _and_ the `^^^^` span. That makes
it a regression test for your error reporting: if a refactor accidentally changes a message or
moves a span, the snapshot stops matching and the test fails.

## Generating snapshots

You don't write `.stderr` files by hand. Write the `.rs` file, then let `trybuild` capture the
output for you:

```bash
TRYBUILD=overwrite cargo test --package trybuild-errors-exercise
```

On the first run there's no `.stderr`, so `trybuild` writes one from the actual compiler output.
**Always review the generated file with `git diff`** — the whole point is to confirm the message
and span are what you intended, not just to rubber-stamp whatever came out. For `Getters` you'd
expect:

```text
# tests/fail/unit_field.stderr
error: Getters can't generate a getter for the `()`-typed field `marker`
 --> tests/fail/unit_field.rs:6:13
  |
6 |     marker: (),
  |             ^^
```

The span underlines just `()`, line 6 — exactly the per-field span you built in the previous
section. An enum, by contrast, underlines the whole item. The snapshot is what _proves_ those
choices reached the user.

> Snapshots are tied to the exact compiler output, so they can shift between Rust versions — and
> when a derive leans on a crate like [`darling`](https://docs.rs/darling), a dependency bump can
> move the wording too. When a change is legitimate, refresh the snapshot the same way you first
> generated it: re-run with `TRYBUILD=overwrite`, then `git diff` to confirm the new output is
> still what you intended before committing it. Keep your fail tests focused on _your_ macro's
> diagnostics (the message and span you control) rather than incidental downstream errors, which
> are more prone to these shifts.

## Exercise

The `Getters` macro is complete. Two near-empty `compile_fail` files are scaffolded under
`tests/fail/` (just an import and `fn main`). Fill each with a program that misuses `Getters` so it
fails to compile — one deriving on an enum, one on a struct with a `()`-typed field. Then generate
the `.stderr` snapshots with `TRYBUILD=overwrite` and **review them**: confirm the enum error
underlines the whole item and the `()` case underlines just the field. The review is the point — a
snapshot only protects you once you've checked it says what you intended.
