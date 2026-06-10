# Building a `#[derive(Builder)]`

Time to put it all together. The builder pattern is a textbook use for a derive macro. For a
struct `Command`, `#[derive(Builder)]` generates a `CommandBuilder` that collects fields one at a
time and validates them at the end:

```rust
let cmd = Command::builder()
    .executable("cargo".to_string())
    .args(vec!["build".to_string()])
    .build()?; // errors if a required field is missing
```

## What the macro generates

For

```rust
#[derive(Builder)]
struct Command {
    executable: String,
    args: Vec<String>,
    current_dir: Option<String>,
}
```

it emits roughly (bare `Option`/`Result`/`String` here for readability — the real macro
qualifies them with absolute paths, as the bullets below explain):

```rust
pub struct CommandBuilder {
    executable: Option<String>,
    args: Option<Vec<String>>,
    current_dir: Option<String>, // already optional — see below
}

impl Command {
    pub fn builder() -> CommandBuilder {
        CommandBuilder { executable: None, args: None, current_dir: None }
    }
}

impl CommandBuilder {
    pub fn executable(&mut self, value: String) -> &mut Self { self.executable = Some(value); self }
    // ... one setter per field ...

    pub fn build(&mut self) -> Result<Command, Box<dyn std::error::Error>> {
        Ok(Command {
            executable: self.executable.clone().ok_or_else(|| /* "field `executable` is not set" */)?,
            args: self.args.clone().ok_or_else(|| /* ... */)?,
            current_dir: self.current_dir.clone(), // optional: no error
        })
    }
}
```

This single macro exercises the whole chapter:

- **Per-field code generation** — a builder field, an initialiser, a setter, and a `build`
  expression for every field (chapter sections on struct fields).
- **Type inspection** — fields that are already `Option<T>` are _optional_: their setter takes the
  inner `T`, and `build()` doesn't require them. Detecting that means looking inside the
  `syn::Type`.
- **Error reporting** — `build()` returns a `Result` and names the first missing required field.
- **Robust output** — absolute paths (`::core::option::Option`, `::std::result::Result`) and a
  derived `...Builder` name via `format_ident!`.

## Detecting `Option<T>`

The one genuinely new technique is digging into a type. A field's `ty` is a `syn::Type`; for
`Option<String>` it's a `Type::Path` whose last path segment is `Option` with one angle-bracketed
type argument. Pulling out that inner type is a drill-down through `syn`'s type model:

```rust
fn option_inner(ty: &syn::Type) -> Option<&syn::Type> {
    let syn::Type::Path(type_path) = ty else { return None };
    let segment = type_path.path.segments.last()?;
    if segment.ident != "Option" { return None; }
    let syn::PathArguments::AngleBracketed(args) = &segment.arguments else { return None };
    match args.args.first()? {
        syn::GenericArgument::Type(inner) => Some(inner),
        _ => None,
    }
}
```

(This is a syntactic check: it matches the name `Option`, so a user who writes
`std::option::Option<T>` still works, but one who aliases `type Maybe<T> = Option<T>` would not —
a fundamental limitation of macros, which only ever see syntax, never resolved types.)

## Exercise

Complete `#[derive(Builder)]`. The builder struct, its initialiser, and the setters are generated
for you, along with the `option_inner` helper. Your job is the heart of `build()`: for each field,
emit the expression that produces its value — cloning optional fields straight through, and
erroring on unset required ones. Make `tests/pass/command.rs` pass.

> **Going further.** A common extension is a field attribute, `#[builder(each = "arg")]`, that
> generates a method to push items onto a `Vec` field one at a time. With this chapter behind you,
> you have every tool needed to add it: parse the field attribute (sections 10–11), and generate an
> extra setter. A great challenge if you want one.
