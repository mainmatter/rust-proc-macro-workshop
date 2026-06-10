#!/usr/bin/env bash
set -euo pipefail

dir="$(dirname "$0")"

pass_files=$(find "$dir/examples" -name "*.rs" 2>/dev/null | wc -l)
fail_files=$(find "$dir/tests/fail" -name "*.rs" 2>/dev/null | wc -l)

if [ "$pass_files" -eq 0 ]; then
    echo "ERROR: No passing test files found in examples/. Add at least one .rs file."
    exit 1
fi

if [ "$fail_files" -eq 0 ]; then
    echo "ERROR: No compile-fail test files found in tests/fail/. Add at least one .rs file."
    exit 1
fi

cargo test --color always
