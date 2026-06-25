#!/usr/bin/env bash
set -eu

dir="$(dirname "$0")"
macros="$dir/crate-structure-exercise-macros"

if [ ! -f "$macros/Cargo.toml" ]; then
    echo "ERROR: crate-structure-exercise-macros/Cargo.toml not found. Create a proc-macro crate in the crate-structure-exercise-macros/ subdirectory."
    exit 1
fi

if ! grep -q 'proc-macro\s*=\s*true' "$macros/Cargo.toml"; then
    echo "ERROR: crate-structure-exercise-macros/Cargo.toml is missing 'proc-macro = true' under [lib]."
    exit 1
fi

if [ ! -f "$macros/src/lib.rs" ]; then
    echo "ERROR: crate-structure-exercise-macros/src/lib.rs not found."
    exit 1
fi

if ! grep -q 'crate-structure-exercise-macros' "$dir/Cargo.toml"; then
    echo "ERROR: Cargo.toml is missing the crate-structure-exercise-macros dependency."
    exit 1
fi

cargo test --color always
