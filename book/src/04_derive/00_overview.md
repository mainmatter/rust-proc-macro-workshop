# Derive macros

In the last chapter you built the full derive-macro workflow end to end: parse a `DeriveInput`
with `syn`, generate code with `quote`, return it as a `TokenStream`, and test it with
`trybuild`. The macros were deliberately small — they read a type name and a flat list of
fields.

Real derive macros do much more. They generate code for every field and every variant, stay
correct no matter what the user's surrounding code looks like, report misuse with clear
compiler errors, and let the user tune the output with attributes. This chapter is where you
build those skills, one at a time:

- **Reading the input properly** — handling all three struct field layouts, and enums.
- **Generating robust code** — absolute paths and hygienic identifiers, so your output compiles
  in any context.
- **Reporting errors well** — turning panics into real compiler diagnostics that point at the
  right span, and snapshotting them with `trybuild`.
- **Attributes** — reading container and field attributes by hand, then with the `darling`
  crate.
- **Generics** — making a derive work for `struct Wrapper<T> { .. }`, lifetimes and `where`
  clauses included.

By the end you'll combine all of it into a real-world macro: a `#[derive(Builder)]` that
generates a builder for any struct.

## A derive macro in the wild: `serde`

Before writing your own, it's worth looking at the most widely used derive macros in the
ecosystem: [`serde`](https://serde.rs)'s `Serialize` and `Deserialize`. You already met
[`thiserror`](https://docs.rs/thiserror) in chapter 2; `serde` adds something new — **attributes**.

`serde` lets you customise the generated code with `#[serde(...)]` attributes, and they come in
two flavours you'll be implementing yourself later in this chapter:

- **Container attributes** sit on the struct or enum and affect the whole type:

  ```rust
  #[derive(Serialize)]
  #[serde(rename_all = "kebab-case")] // every field name -> kebab-case in the output
  struct Product {
      unit_price: f64, // serializes as "unit-price"
  }
  ```

  `rename_all` accepts a whole family of cases — `"camelCase"`, `"snake_case"`, `"kebab-case"`,
  `"SCREAMING_SNAKE_CASE"`, and more — so you pick the one that matches the shape you need.

- **Field attributes** sit on a single field and affect only that field:

  ```rust
  #[derive(Serialize)]
  struct Product {
      #[serde(rename = "id")] // this field serializes as "id"
      sku: String,
  }
  ```

Not every field attribute renames things — some change _behaviour_. `#[serde(default)]`, for
example, tells the generated `Deserialize` code to fall back to the field's `Default` value when
it's missing from the input, instead of failing:

```rust
#[derive(Deserialize)]
struct Product {
    #[serde(default)] // missing in the JSON? use `bool::default()` (false)
    in_stock: bool,
}
```

The derive macro reads these attributes while it generates code and adjusts its output
accordingly. That's the core idea behind a configurable derive macro, and it's what makes
`serde` flexible enough to match almost any JSON shape.

## Exercise

A warm-up from the user's perspective: add `serde` container and field attributes — `rename_all`,
`rename`, and `default` — to a struct so it serializes and deserializes the way the tests expect.
You're _using_ a derive macro here,
not writing one — that starts in the next section.
