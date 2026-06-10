# Building a `#[retry]` attribute

Time to combine the whole chapter into one real-world macro: `#[retry]`, which wraps a fallible
function so it automatically tries again when it fails.

```rust
#[retry(times = 3, delay_ms = 100)]
fn fetch(url: &str) -> Result<String, HttpError> {
    // ... a flaky network call ...
}
```

`fetch` now runs up to three times, pausing 100ms between attempts, and only surfaces an `Err` if
_every_ attempt failed. The caller's code doesn't change at all — the signature is identical; only
the body is rewritten.

This pulls together everything you've built:

- **Attribute arguments** (`times`, `delay_ms`) parsed with `darling`'s `FromMeta` (section 03).
- **Body transformation** — replace the function's block while keeping its signature (section 01).
- **Absolute paths** in the generated code, so it compiles in any module (chapter 4).

## What the macro generates

For the `fetch` above, `#[retry]` rewrites the body to roughly:

```rust
fn fetch(url: &str) -> Result<String, HttpError> {
    let mut attempt: u32 = 0;
    loop {
        attempt += 1;
        let result = (|| -> Result<String, HttpError> {
            // ... the original body ...
        })();
        match result {
            Ok(value) => return Ok(value),
            Err(err) => {
                if attempt >= 3 {
                    return Err(err);
                }
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
        }
    }
}
```

(Shown with plain paths and concrete values for readability — the real expansion uses `times` and
`delay_ms` from the parsed arguments, and absolute paths like `::core::result::Result` and
`::std::thread::sleep`, for the hygiene reasons from chapter 4.)

## The one subtle part: the closure

Why wrap the original body in `(|| { ... })()` instead of running it directly?

Because the body is fallible code, it almost certainly uses `?` and `return`. If you spliced it
straight into the loop, a `?` on the _first_ failed attempt would return the `Err` out of `fetch`
entirely — defeating the whole point of retrying. Running the body inside a **closure that returns
the function's own type** contains that: `?` and `return` exit the _closure_, handing you a
`Result` you can inspect and decide whether to retry.

You can build that closure's return type straight from the parsed function: `func.sig.output` is the
`-> Result<...>` token sequence, so `(|| #output #block)()` produces
`(|| -> Result<...> { <body> })()`. (This is exactly the "run the body in a closure" escape hatch
the timing section hinted at.)

The rest is a plain loop: count attempts, return on `Ok`, and on `Err` either give up (last attempt)
or sleep and go round again.

## Exercise

Finish `#[retry]`. The argument parsing (`times`, `delay_ms` via `FromMeta`) and the function
parsing are written for you; your job is the heart of the macro — generating the retry loop and
installing it as the new function body.

Build a `syn::Block` with `parse_quote!` that wraps the original body in the closure shown above,
loops up to `times` times, returns on success, and sleeps `delay_ms` between failed attempts before
giving up with the last error. Reach for absolute paths (`::core::result::Result`, `::std::thread`,
`::std::time::Duration`) so the generated code never depends on what's in scope at the call site.
Make `examples/retrying.rs` pass.

> **Going further.** Real retry helpers add exponential backoff (doubling the delay each time), a
> jitter factor, or a predicate that only retries _certain_ errors. Each is a small extension of the
> loop you just generated — and a good way to cement the technique.
