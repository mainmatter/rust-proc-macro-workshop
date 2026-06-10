#!/usr/bin/env bash
set -euo pipefail

dir="$(dirname "$0")"

if [ ! -f "$dir/macros/Cargo.toml" ]; then
    echo "ERROR: macros/Cargo.toml not found. Create a proc-macro crate in the macros/ subdirectory."
    exit 1
fi

if ! grep -q 'proc-macro\s*=\s*true' "$dir/macros/Cargo.toml"; then
    echo "ERROR: macros/Cargo.toml is missing 'proc-macro = true' under [lib]."
    exit 1
fi

if [ ! -f "$dir/macros/src/lib.rs" ]; then
    echo "ERROR: macros/src/lib.rs not found."
    exit 1
fi

if ! grep -q 'macros' "$dir/Cargo.toml"; then
    echo "ERROR: Cargo.toml is missing the macros dependency."
    exit 1
fi

if grep -q '^// use macros' "$dir/src/lib.rs"; then
    echo "ERROR: The code in src/lib.rs is still commented out. Uncomment it."
    exit 1
fi

cargo test --color always
