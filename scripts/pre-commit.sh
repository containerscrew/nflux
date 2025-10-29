#!/usr/bin/env bash
set -euo pipefail

separator() {
    echo -e "\n--- $1 ---"
}

separator "Running pre-commit hooks"
pre-commit run -a --show-diff-on-failure

# separator "Scanning for secrets"
# gitleaks git -v

# separator "Running tests with nextest"
# cargo nextest run

separator "Running cargo fmt and cargo check"
cargo fmt --all --
cargo check -q
