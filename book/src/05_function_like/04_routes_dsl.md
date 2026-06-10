# Building a simple DSL: `routes!`

Time to put the chapter together. You'll build a `routes!` macro — a tiny domain-specific language
for declaring HTTP routes, the kind of thing web frameworks expose:

```rust
let router = routes! {
    GET  "/"       => index,
    GET  "/about"  => about,
    POST "/users"  => create_user,
};

assert_eq!(router("GET", "/about"), Some("about us".to_string()));
assert_eq!(router("DELETE", "/"),   None);
```

Each line is `METHOD "path" => handler`, where `handler` is an ordinary `fn() -> String`. The macro
expands to a **closure** that takes a method and path and dispatches to the first matching
handler — or returns `None` if nothing matches:

```rust
|method: &str, path: &str| -> Option<String> {
    if method == "GET" && path == "/" { return Some(index()); }
    if method == "GET" && path == "/about" { return Some(about()); }
    if method == "POST" && path == "/users" { return Some(create_user()); }
    None
}
```

This exercises everything from the chapter: a custom grammar (`02`), a hand-written `Parse` impl
over a `Punctuated` list (`01`–`02`), and `quote!` code generation with absolute paths
(chapter 4).

## The grammar

A route has four parts read in order — a method identifier, a path literal, the `=>` arrow, and a
handler identifier — so it gets a struct and a `Parse` impl:

```rust
struct Route {
    method: Ident,
    path: LitStr,
    handler: Ident,
}
```

The `Parse` impl pulls those four pieces off the `ParseStream` one at a time, in grammar order —
the same technique as section `02`'s `Entry`.
The method is a plain `Ident` rather than a custom keyword — `GET`,
`POST`, `PUT` and friends are all accepted uniformly, and the macro just stringifies whatever
identifier you wrote.

## Generating the dispatcher

With the routes parsed, the output is one `if` arm per route wrapped in a closure. For each
`Route` you compare the runtime `method`/`path` against the route's own — the method as a string
(`route.method.to_string()`), the path as its literal — and on a match, call the handler and return
`Some(...)`. The arms are spliced into the closure body with `quote!`'s `#( ... )*` repetition, and
everything is qualified with absolute paths so the expansion compiles in any module:

```rust
quote! {
    |method: &str, path: &str| -> ::core::option::Option<::std::string::String> {
        #( #arms )*
        ::core::option::Option::None
    }
}
```

## Exercise

This is the chapter's capstone, so you write both halves. First the **parser**: fill in the
`Parse` impl for a single `Route`, reading the method, path, `=>`, and handler off the stream in
order (the `Routes` wrapper around the `Punctuated` list is provided). Then the **generator**: turn
the parsed routes into the matching closure shown above — build one match arm per route by mapping
over the list, splice the arms into the closure body, and fall through to `None`. Reach for the
absolute paths you learned in chapter 4 so the generated code never collides with names in the
caller's scope. `tests/fail/missing_arrow.rs` checks your parser rejects a missing `=>`, and
`tests/pass/router.rs` checks the whole macro — make them both pass.

> **Going further.** Real routers do more: matching path parameters like `/users/:id` and handing
> them to the handler, supporting a fallback/404 handler, or grouping routes under a shared prefix.
> Each is a natural extension of the parser and the generator you just wrote — good challenges if
> you want to keep going.
