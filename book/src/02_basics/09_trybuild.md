# Testing with `trybuild`

In the [`proc-macro2`](./06_proc_macro2.md) section you wrote a unit test that called your
code-generation function directly and asserted on the tokens it produced — no separate crate,
no real `#[derive]`. That's the fastest way to check your _logic_, and you can do the same for
the `FieldNames` macro:

```rust
#[test]
fn generates_field_names() {
    let input: syn::DeriveInput = syn::parse_str("struct Foo { x: i32 }").unwrap();
    let output = field_names_impl(&input).to_string();
    assert!(output.contains("field_names"));
}
```

This is useful, but a unit test like this has two blind spots.

## Blind spot 1: valid tokens ≠ working code

You can strengthen the test by asserting that the output _parses_ — for example with
[`syn::parse2`](https://docs.rs/syn/latest/syn/fn.parse2.html). But parsing only proves the
tokens are syntactically well-formed; it does **not** prove they compile and run. Imagine the
macro had a typo and generated the method name `feild_names`:

```rust
quote! {
    impl #name {
        pub fn feild_names() -> &'static [&'static str] { &[ #(#field_strings),* ] }
    }
}
```

This still parses as a perfectly valid `impl` block, so a `syn::parse2` assertion passes. But
every caller that writes `Foo::field_names()` now gets a compile error. A unit test won't catch
this — only actually _compiling_ the generated code will.

## Blind spot 2: you can't see the error the user sees

When the macro is misused — say, on an enum — `field_names_impl` panics. A unit test can
confirm _that_ it panics:

```rust
#[test]
#[should_panic]
fn rejects_enums() {
    let input: syn::DeriveInput = syn::parse_str("enum E { A, B }").unwrap();
    field_names_impl(&input);
}
```

But this tells you nothing about what the _user_ experiences at their `#[derive(FieldNames)]`
call site: which message they get, and where the compiler points. Right now that message is
just a raw panic — from chapter 3 onward you'll put real effort into making these diagnostics
helpful, and you'll need a way to see and lock in exactly what the user gets.

## `trybuild` fills both gaps

[`trybuild`](https://docs.rs/trybuild) is a test harness that compiles standalone Rust files —
as if a user wrote them — and checks whether they succeed or fail with the expected error
messages. It catches both blind spots: it compiles the generated code for real, and it
snapshots the exact diagnostic a misuse produces.

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

- It closes **blind spot 1**: `pass` tests compile _and run_ the generated code, so a typo like
  `feild_names` fails the test instead of slipping through.
- It closes **blind spot 2**: `compile_fail` tests snapshot the exact diagnostic the user sees.
  Once you start improving error messages in chapter 3, the `.stderr` file is what proves those
  messages are actually helpful — and it catches regressions if a `syn::Error` span later
  changes.
- It complements `proc-macro2` unit tests rather than replacing them: unit tests verify your
  logic fast and in isolation, `trybuild` verifies the end result from the user's perspective.

We'll use `trybuild` throughout the rest of this workshop to verify both successful
compilation and expected error messages.

## Exercise

Write `trybuild` tests for the `FieldNames` derive macro you built in the previous exercise.
Add both a passing test (a named struct) and a compile-fail test (using the macro on an enum).
