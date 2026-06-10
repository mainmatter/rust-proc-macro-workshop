#!/usr/bin/env bash
set -euo pipefail

dir="$(dirname "$0")"

fail_files=$(find "$dir/tests/fail" -name "*.rs" 2>/dev/null | wc -l)

if [ "$fail_files" -eq 0 ]; then
    echo "ERROR: No compile-fail test files found in tests/fail/. Add at least one .rs file"
    echo "       (plus its .stderr snapshot, generated with TRYBUILD=overwrite)."
    exit 1
fi

cargo test --color always
