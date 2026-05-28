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

cargo test --color always
