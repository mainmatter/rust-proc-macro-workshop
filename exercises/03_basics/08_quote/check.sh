#!/usr/bin/env bash
set -euo pipefail

dir="$(dirname "$0")"

if grep -q 'format!' "$dir/macros/src/lib.rs"; then
    echo "ERROR: Still using format! for code generation. Rewrite field_names_impl to use quote! instead."
    exit 1
fi

if ! grep -q 'quote!' "$dir/macros/src/lib.rs"; then
    echo "ERROR: Expected quote! usage in macros/src/lib.rs."
    exit 1
fi

# Test both the consumer crate and the macro crate (the latter holds the unit
# tests for `field_names_impl`).
cargo test --color always -p quote-exercise -p quote-exercise-macros
