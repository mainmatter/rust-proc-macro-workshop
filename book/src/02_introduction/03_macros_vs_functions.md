# Macros vs functions

Why do macros exist when Rust already has functions? Functions are simpler, easier to understand,
and don't require any special syntax. Why would you reach for a macro instead?

## What functions can't do

Functions operate on **values** at **runtime**. Macros operate on **tokens** at **compile time**.
This gives macros capabilities that functions fundamentally cannot have.

### Variable number of arguments

```rust
// This works — println! accepts any number of arguments
println!("{} + {} = {}", a, b, a + b);
```

Rust functions have a fixed number of parameters. There's no variadic function syntax.
`println!` is a macro precisely because it needs to accept an arbitrary number of arguments
and type-check each one against the format string — all at compile time.

### Code generation

```rust
#[derive(Debug, Clone, PartialEq)]
struct Config {
    host: String,
    port: u16,
}
```

A function can't look at a struct definition and generate trait implementations for it.
That requires inspecting the structure of the code itself — its fields, their types, their
attributes — and producing new code based on what it finds.

### New syntax

```rust
let query = sqlx::query!("SELECT * FROM users WHERE id = $1", user_id);
```

[`sqlx::query!`](https://docs.rs/sqlx/latest/sqlx/macro.query.html) parses a SQL string at compile time, verifies it against your database schema,
and generates type-safe Rust code. No function can invent new syntax or perform compile-time
verification of a string literal.

## When functions are enough

If you can do it with a function, do it with a function. Macros are harder to write, harder to
debug, and harder for users to understand when something goes wrong.

Use a macro only when you need something a function cannot provide: code generation, compile-time
inspection of types, variadic arguments, or new syntax.

## Exercise

Use standard library macros ([`format!`](https://doc.rust-lang.org/std/macro.format.html), [`env!`](https://doc.rust-lang.org/std/macro.env.html), [`include_str!`](https://doc.rust-lang.org/std/macro.include_str.html), [`concat!`](https://doc.rust-lang.org/std/macro.concat.html)) to accomplish tasks
that would be impossible with regular functions: formatting with a variable number of arguments,
reading environment variables at compile time, embedding file contents, and concatenating
string literals.
