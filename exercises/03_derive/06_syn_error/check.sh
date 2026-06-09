#!/usr/bin/env bash
set -euo pipefail

# The error-path checks live in the macros crate's unit tests, so test both the
# consumer crate and the macros crate (the consumer's trybuild test only covers
# the happy path).
cargo test --color always -p syn-error-exercise -p syn-error-exercise-macros
