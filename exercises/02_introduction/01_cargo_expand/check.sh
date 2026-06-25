#!/usr/bin/env bash
set -eu

# This exercise relies on `cargo expand`, so make sure it is installed before
# running the tests. See the book for installation instructions.
if ! cargo expand --version >/dev/null 2>&1; then
    echo "ERROR: cargo-expand is not installed. Install it with 'cargo binstall cargo-expand' (see the book chapter)."
    exit 1
fi

cargo test --color always -p cargo-expand-exercise
