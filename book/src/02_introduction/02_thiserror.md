# `thiserror`: a derive macro in the wild

Let's look at a real-world derive macro: [`thiserror`](https://docs.rs/thiserror).

`thiserror` provides a `#[derive(Error)]` macro that generates implementations of
`std::fmt::Display` and `std::error::Error` for your error types. Without it, you'd have to
write those implementations by hand — repetitive boilerplate.

## Before `thiserror`

```rust
use std::fmt;

#[derive(Debug)]
enum ApiError {
    NotFound,
    Unauthorized(String),
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiError::NotFound => write!(f, "resource not found"),
            ApiError::Unauthorized(msg) => write!(f, "unauthorized: {msg}"),
        }
    }
}

impl std::error::Error for ApiError {}
```

## After `thiserror`

```rust
use thiserror::Error;

#[derive(Debug, Error)]
enum ApiError {
    #[error("resource not found")]
    NotFound,
    #[error("unauthorized: {0}")]
    Unauthorized(String),
}
```

The macro reads the `#[error("...")]` attributes on each variant and generates the `Display`
and `Error` implementations for you.

## Attributes

Notice how `thiserror` uses **helper attributes** to customize the generated code:

- `#[error("...")]` controls the `Display` message for each variant.
- [`#[from]`](https://docs.rs/thiserror/latest/thiserror/index.html) can be added to a variant field to automatically generate a `From` implementation.
  It also implies `#[source]`, so you don't need both on the same field.
- [`#[source]`](https://docs.rs/thiserror/latest/thiserror/index.html) marks a field as the error source for the `Error::source()` method.

These are **derive macro helper attributes** — attributes that only have meaning in the context
of the derive macro they belong to. They are not standalone attribute macros.

This pattern — a derive macro combined with helper attributes — is extremely common in the Rust
ecosystem. `serde`, `clap`, `sqlx`, and many other crates use it.

Later in this workshop, you'll learn how to define your own derive macros with helper attributes
just like these.

## Under the hood

Since we just learned about `cargo expand`, let's see what `thiserror` generates.
Given the `ApiError` enum from the "After" example above, running `cargo expand` would show
(among other things) a `Display` implementation like this:

```rust
impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ApiError::NotFound => f.write_str("resource not found"),
            ApiError::Unauthorized(v) => {
                write!(f, "unauthorized: {}", v)
            }
        }
    }
}

impl std::error::Error for ApiError {}
```

The same code you would have written by hand.

## Exercise

Use `thiserror` to define an error type with multiple variants. Pay attention to the attributes.
