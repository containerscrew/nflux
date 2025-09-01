#!/usr/bin/env bash
set -euo pipefail

separator() {
    echo -e "\n--- $1 ---\n"
}

separator "Formatting Rust code"
cargo fmt --all > /dev/null

separator "Running pre-commit hooks"
pre-commit run -a --show-diff-on-failure

separator "Running tests with nextest"
cargo nextest run
