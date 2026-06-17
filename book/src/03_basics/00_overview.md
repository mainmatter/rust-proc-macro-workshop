# The proc-macro toolkit

It's time to get your hands dirty. In this chapter, you'll build your first procedural macros
from scratch — starting with the bare minimum and gradually introducing the tools that make
macro development practical.

You'll learn:

- How proc-macro crates are structured and wired up
- What a derive macro actually returns
- How tokens and token streams work under the hood
- How [`proc-macro2`](https://docs.rs/proc-macro2) makes your macro code testable
- How [`syn`](https://docs.rs/syn) parses Rust code into a structured representation
- How [`quote`](https://docs.rs/quote) lets you generate code that looks like regular Rust
- The re-export pattern used by `serde`, `thiserror`, and most real-world macro crates

By the end of this chapter, you'll be comfortable with the full derive-macro workflow: parse
the input with `syn`, generate code with `quote`, and return it as a token stream.

## Exercise

A quick warm-up before diving in: use [`derive_more`](https://docs.rs/derive_more)'s `Display`
derive to implement `Display` for an enum. `derive_more` brings derive macros to many standard
library traits — `Display`, `From`, `Add`, `Deref`, and more — saving you the hand-written
boilerplate. It's a good reminder of what derive macros look like from the user's perspective —
you'll be building your own from the next section onwards.
