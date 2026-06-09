#!/usr/bin/env bash
set -euo pipefail

# The attribute-parsing checks live in the macros crate's unit tests, so test both
# the consumer crate and the macros crate (the consumer's trybuild test only covers
# the happy path).
cargo test --color always -p field-attr-exercise -p field-attr-exercise-macros
