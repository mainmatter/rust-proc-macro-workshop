# Using absolute paths

A derive macro generates code that gets pasted into _someone else's_ crate, in a module you've
never seen. You don't control what's in scope there — and that's a problem, because the code
you generate has to compile no matter what names the user has brought into scope.

## The trap

Suppose your macro generates code that returns a `String`:

```rust
quote! {
    impl #name {
        pub fn type_name(&self) -> String {
            String::from(stringify!(#name))
        }
    }
}
```

This looks fine, and it works in every test you write. Then a user does this:

```rust
type String = MyFancyString;

#[derive(TypeName)]
struct Widget;
```

Now the `String` in your generated code resolves to `MyFancyString`, and their build breaks
with a confusing error pointing at _their_ `#[derive(TypeName)]`, not at your macro. Derive
macro output is **not** path-hygienic: bare names like `String`, `Option`, `Result`, `Vec`, and
`Box` resolve at the _call site_, against whatever the user has in scope.

## The fix: absolute paths

Refer to every external item by a fully-qualified, absolute path — one starting with a leading
`::`. A leading `::` anchors the path at a crate root, so it can't be intercepted by a local
name:

```rust
quote! {
    impl #name {
        pub fn type_name(&self) -> ::std::string::String {
            ::std::string::String::from(stringify!(#name))
        }
    }
}
```

Common standard-library paths:

| Bare name                | Absolute path                                               |
| ------------------------ | ----------------------------------------------------------- |
| `Option`, `Some`, `None` | `::core::option::Option`, `::core::option::Option::Some`, … |
| `Result`, `Ok`, `Err`    | `::core::result::Result`, …                                 |
| `String`                 | `::std::string::String`                                     |
| `Vec`                    | `::std::vec::Vec`                                           |
| `Box`                    | `::std::boxed::Box`                                         |
| `Default::default()`     | `::core::default::Default::default()`                       |

> Many `core` items are also re-exported from `std`, so `::std::option::Option` works too. Using
> `::core` lets your generated code work in `#![no_std]` crates as well — a nice habit if you
> want your macro to be maximally portable.

## Referring to your own crate

The same problem applies to _your_ crate's items. If your macro lives in a crate `my_macros` but
the trait it implements lives in a companion crate `my_lib`, the generated code must name
`::my_lib::TheTrait`, not `TheTrait` — the user may not have imported it. This is the other half
of why real macro crates use the re-export pattern you saw in chapter 2.

> Robust crates often go one step further and accept the crate path as an attribute (serde's
> `#[serde(crate = "...")]`) so the macro keeps working even if the user renamed the dependency.
> That's beyond this exercise, but it's the same idea taken to its conclusion.

To keep each later exercise focused on its own new idea, the generated code in this chapter
mostly sticks to bare names (`String`, `Vec`, `format!`). That's fine for tests that don't shadow
those names — but in a macro you ship, qualify every external item with an absolute path, as the
capstone in the last section does.

## Exercise

The `TypeName` derive macro generates a `type_name()` method, but it refers to `String` by its
bare name. `tests/pass/shadowed.rs` shadows `String` and currently fails to compile. Fix the
macro to use absolute paths so both passing tests compile.
