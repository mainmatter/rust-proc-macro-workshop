# Testing with `trybuild`

You've seen how `proc-macro2` lets you unit-test macro logic. But unit tests only verify that
your macro produces the right tokens — they don't check that those tokens actually compile, or
that invalid input produces a helpful error message.

[`trybuild`](https://docs.rs/trybuild) fills this gap. It's a test harness that compiles
standalone Rust files and checks whether they succeed or fail with expected error messages.

## How it works

A `trybuild` test is a regular `#[test]` function that points at a directory of `.rs` files:

```rust
#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/pass/*.rs");
    t.compile_fail("tests/fail/*.rs");
}
```

- **`pass()`** — each file must compile and run successfully.
- **`compile_fail()`** — each file must fail to compile, and the error output must match a
  corresponding `.stderr` file.

## Test file structure

Each test file is a self-contained Rust program. For testing a derive macro, a passing test
might look like:

```rust
// tests/pass/basic_struct.rs
use my_macros::MyDerive;

#[derive(MyDerive)]
struct Foo {
    x: i32,
}

fn main() {
    // Verify the generated code works.
    assert_eq!(Foo::field_names(), &["x"]);
}
```

Note the `fn main()` — each test file is compiled as its own binary.

## Testing compilation failures

For `compile_fail` tests, `trybuild` compares the compiler's error output against a `.stderr`
file with the same name:

```text
tests/
├── fail/
│   ├── not_a_struct.rs
│   └── not_a_struct.stderr
└── pass/
    └── basic_struct.rs
```

On the first run, if no `.stderr` file exists, `trybuild` shows the actual compiler output and
fails the test. You can set the environment variable `TRYBUILD=overwrite` to have it write
the `.stderr` files for you, then review them with `git diff` to make sure they match your
expectations.

## Why `trybuild`?

- It catches regressions in error messages — if a `syn::Error` span changes, the `.stderr`
  snapshot will fail.
- It tests the macro from the user's perspective — real compilation, real error messages.
- It complements `proc-macro2` unit tests: unit tests verify your logic, `trybuild` verifies
  the end result.

We'll use `trybuild` throughout the rest of this workshop to verify both successful
compilation and expected error messages.

## Exercise

Write `trybuild` tests for the `FieldNames` derive macro you built in the previous exercise.
Add both a passing test (a named struct) and a compile-fail test (using the macro on an enum).
