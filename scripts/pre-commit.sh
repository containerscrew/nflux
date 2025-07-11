#! /usr/bin/env bash

# Run pre-commit hooks
pre-commit run -a

# Cargo
cargo fmt --all --
cargo nextest run

# Changelog
#git cliff --output CHANGELOG.md
