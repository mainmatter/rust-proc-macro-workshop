# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Mainmatter workshop teaching Rust procedural macros. It follows the same structure as other Mainmatter Rust workshops (rust-telemetry-workshop, rust-python-interoperability). Exercises are paired with a companion book built with mdbook, hosted on rust-exercises.com.

## Repository Structure

- `exercises/` — Workshop exercises, organized as `[module]/[exercise]/` (e.g. `exercises/01_derive/02_fields/`). Each exercise is an individual Cargo crate and a workspace member.
- `book/` — mdbook source. Each chapter in `book/src/` corresponds to an exercise module.
- `Cargo.toml` — Workspace root with `members = ["exercises/*/*"]`.
- `.wr.toml` — Configuration for the `wr` workshop runner tool.
- `dprint.json` — Formatter config for markdown and TOML files.

## Common Commands

```bash
# Build all exercises
cargo build --workspace

# Test a single exercise
cargo test --package <exercise_crate_name>

# Run all tests
cargo test --workspace

# Build the book locally
cd book && mdbook serve

# Format markdown and TOML
dprint fmt

# Check formatting
dprint check
```

## Reference Workshops

Use these sibling Mainmatter workshops as reference for structure, layout, and tone. If you have
them checked out next to this repository (e.g. `../rust-telemetry-workshop`), prefer the local
copy; otherwise consult them on GitHub:

- `../rust-telemetry-workshop` — <https://github.com/mainmatter/rust-telemetry-workshop>
- `../rust-python-interoperability` — <https://github.com/mainmatter/rust-python-interoperability>
- `../100-exercises-to-learn-rust` — <https://github.com/mainmatter/100-exercises-to-learn-rust>

## Workshop Plan

`toc.md` contains the current plan for the workshop: the planned modules, exercises, and their ordering. Use it as the source of truth when creating or reorganizing exercises and book chapters.

## Formatting

- Always run `dprint fmt` after editing markdown (`.md`) or TOML files.

## Exercise Conventions

- Each section of the book has exactly one associated exercise.
- When adding a new section to the book, `book/src/SUMMARY.md` must be updated to include it.

- Modules use 2-digit prefixed directories: `00_intro`, `01_derive`, `02_attribute`.
- Exercises within modules also use 2-digit prefixes: `00_welcome`, `01_basic`.
- Each exercise has its own `Cargo.toml` and `src/` directory.
- Exercises use `todo!()` as placeholders for participants to fill in.
- Tests live in `tests/` subdirectory with separate files (e.g. `success.rs`, `failure.rs`).
- Exercise verification is configured via `.wr.toml` (root or per-exercise).
- Common dependencies (syn, quote, proc-macro2) should be declared as `[workspace.dependencies]` and referenced with `workspace = true` in exercise Cargo.toml files.

## Exercise design — make participants think

An exercise must NOT be a copy-paste of the book section it accompanies. The book teaches a
technique with a fully-worked example; the exercise must require the participant to _apply_ that
technique, not transcribe it.

- **Use a different-but-analogous macro.** If the book walks through macro `Foo`, the exercise
  should build a _sibling_ macro `Bar` that exercises the same skill on a different problem. Never
  ship an exercise whose solution is the book's worked example verbatim. (E.g. book derives
  `as_str` for enums → exercise derives `Ordinal`; book gives `Loud` with a type-only unique name →
  exercise needs a per-(type, field) unique name.)
- **Require transfer, not transcription.** The exercise should force at least one genuinely new
  decision the book example doesn't make for them — a different output, an extra dimension
  (`enumerate`, an index, a second identifier to combine), or a case the book's example doesn't
  cover. Prefer designs where the _obvious naive approach fails_, and verify it actually fails
  (e.g. confirm the clash/compile error) so the exercise has real substance.
- **TODO comments guide, they don't answer.** Explain the goal and the "why" (which API, what
  constraint to satisfy), but do NOT paste the literal solution — no exact `format_ident!` string,
  no spelled-out match arms, no copyable snippet. Point back to the book example as the reference.
- **Tests must not leak the answer.** Assertions should go through stable, participant-independent
  surfaces (a method named after the field, a public return value) rather than referencing the
  exact identifiers/tokens the macro is supposed to generate.
- **Workflow:** write the full working solution first and verify it (`cargo test`), then replace
  only the load-bearing logic with a `todo!()`/TODO. Keep the surrounding scaffolding compiling so
  the participant gets useful errors, not a wall of unrelated ones.
- **The verification must fail on the unmodified stub.** After stubbing, run the exercise's actual
  verification (`wr check`, or the configured `cargo test`/`check.sh`) and confirm it FAILS, then
  confirm the solution makes it PASS. A green check on the untouched stub means the test doesn't
  exercise the participant's work. Watch the common trap: the consumer crate's `trybuild` _pass_
  test only covers the happy path, so work on an **error/edge path** (rejecting bad input, a
  per-field validation) won't be caught by it. Those checks usually live in the **macros crate's**
  `#[cfg(test)]` unit tests — which `cargo test -p <consumer>` does NOT run. When that's the case,
  add a per-exercise `.wr.toml` + `check.sh` that tests both packages
  (`cargo test -p <consumer> -p <consumer>-macros`), as `03_basics/08_quote` does.

## Testing with `trybuild`

From chapter 4 onwards, exercises should use `trybuild` for testing macro output. Each exercise
should include:

- `tests/pass/*.rs` — files that must compile and run successfully.
- `tests/fail/*.rs` — files that must fail to compile, with matching `.stderr` snapshots.

`trybuild` is introduced in chapter 3 (`03_basics/09_trybuild`). Exercises in chapters 2 and 3
use inline `#[cfg(test)]` tests instead.

## Solutions

Solutions live on a separate `solutions` branch as a single commit on top of `main`. The `main` branch contains exercises with `todo!()` placeholders.

### Keeping `solutions` up to date

The `solutions` commit is stacked on top of `main`, so any change to an exercise on `main` must be
reflected in the solution. After modifying an exercise, you MUST:

1. **Rebase `solutions` onto the new commit** so it sits on top of the latest `main`.
   - With `jj`: `jj rebase -b solutions -d main` (or, when the solution is already stacked on the
     in-progress change, jj rebases it automatically — verify with `jj log`).
   - With `git`: check out `solutions` and `git rebase main`.
2. **Resolve any conflicts**, keeping the solution's filled-in implementation (the real code that
   replaces the exercise's `todo!()`) while picking up the new exercise scaffolding (tests,
   derives, `.wr.toml`/`check.sh`, comments, etc.).
3. **Ensure `wr check` passes for the updated solution.** Check out the `solutions` commit
   (`jj edit solutions`) and run `wr check` from the affected exercise's directory. The solution
   must be green before you consider the exercise change complete.
