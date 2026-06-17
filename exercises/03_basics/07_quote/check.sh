#!/usr/bin/env bash
set -euo pipefail

dir="$(dirname "$0")"
lib="$dir/quote-exercise-macros/src/lib.rs"

# Strip comment lines so a mention of `format!`/`quote!` in a doc comment
# doesn't trip these checks — we only care about actual code.
code="$(grep -v '^[[:space:]]*//' "$lib")"

if grep -q 'format!' <<<"$code"; then
    echo "ERROR: Still using format! for code generation. Rewrite field_names_impl to use quote! instead."
    exit 1
fi

if ! grep -q 'quote!' <<<"$code"; then
    echo "ERROR: Expected quote! usage in quote-exercise-macros/src/lib.rs."
    exit 1
fi

# Test both the consumer crate and the macro crate (the latter holds the unit
# tests for `field_names_impl`).
cargo test --color always -p quote-exercise -p quote-exercise-macros
