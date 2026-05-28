---
name: review-chapter
description: Review a workshop chapter — check content accuracy, phrasing, style, exercise coverage, and structural issues. Accepts a git ref, chapter directory, or defaults to the current diff.
disable-model-invocation: false
allowed-tools: Bash, Read, Agent, Explore
user-invocable: true
---

# Workshop Chapter Review

Review a chapter of this proc macro workshop for quality, accuracy, and completeness.

## Input

The user provides one of:
- A **git ref** (commit hash, branch name) or **jj change ID** — review all chapter files changed in that ref/change.
- A **chapter directory** path (e.g. `book/src/01_introduction`) — review that chapter.
- Nothing — review the current uncommitted changes (`git diff` or `jj diff`).

## What to review

### 1. Content accuracy

- Are technical claims correct? Flag anything misleading or wrong.
- Are code examples valid Rust? Check that they would compile (or are clearly marked as pseudocode).
- Are macro types correctly identified? (e.g. don't call `vec!` a procedural macro when it's `macro_rules!`; don't call `#[test]` an attribute macro when it's a compiler built-in.)
- Do links point to real crates/docs?
- When third-party crates or macros are mentioned by name (e.g. `thiserror`, `serde::Serialize`, `tokio::main`), are they linked to their docs.rs documentation? Flag any unlinked crate/macro references.

### 2. Phrasing and style

- Compare against the tone of sibling Mainmatter workshops (see CLAUDE.md for reference paths). The tone should be: direct, second-person ("you"), concise sentences, no filler.
- Flag overly long paragraphs, passive voice, or jargon that isn't explained.
- Flag inconsistent terminology (e.g. switching between "proc macro" and "procedural macro" without reason).
- Check spelling and grammar in both book sections and exercise comments/docstrings. Flag typos, subject-verb disagreements, missing articles, incorrect punctuation, etc.

### 3. Book section ↔ exercise alignment

For each book section (`book/src/<module>/<section>.md`), find its matching exercise (`exercises/<module>/<section>/`). Check:
- Does the exercise test the concepts introduced in the section?
- Are there concepts in the section that the exercise doesn't touch? Flag gaps.
- Does the exercise introduce concepts not covered in the section? Flag surprises.
- Does the exercise description in the book's "## Exercise" paragraph match what the exercise actually asks?
- Are there mismatches between the book text and exercise code (e.g. book says "enum" but exercise uses a struct)?

### 4. Exercise quality

- Is the `todo!()` placement clear — does the participant know what to fill in?
- Are test assertions correct and sufficient?
- Do tests leak the answer too obviously (e.g. expected values in assertion comments)?
- Are exercise Cargo.toml dependencies correct and using workspace dependencies where appropriate?

### 5. Structural issues

- Are there orphaned or misplaced files (e.g. placeholder files for future modules in the wrong directory)?
- Is `book/src/SUMMARY.md` consistent with the actual files?
- Does the chapter match the plan in `toc.md`?

### 6. Suggestions to extend

- Are there obvious topics the section should cover but doesn't?
- Would a concrete example, number, or tip make a point more tangible?
- Are there practical tips for the workshop context (e.g. workspace-specific cargo commands)?

## Output format

Structure the review as:

```
## Content & Phrasing

Per-section notes (use ### for each section filename). Only flag issues — don't narrate what's fine.

## Structural Issues

Problems with file organization, SUMMARY.md, toc.md alignment, placeholder files.

## Suggestions to Extend

Numbered list of concrete suggestions with rationale.

## Exercise Coverage Summary

| Section | Exercise | Coverage |
|---|---|---|
| ... | ... | Good / Adequate / Gap: <description> |

```

Keep it concise. Lead each finding with the specific file and line if applicable. Don't pad with praise — focus on actionable items.
