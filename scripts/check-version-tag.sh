#!/bin/bash
set -euo pipefail

tag=$(git describe --tags --exact-match 2>/dev/null || true)

if [[ -z "$tag" ]]; then
  echo "Not on a Git tag, skipping Cargo.toml version check."
  exit 0
fi

tag_version="${tag#v}"
cargo_version=$(grep '^version' Cargo.toml | head -1 | cut -d '"' -f2)

if [[ "$tag_version" != "$cargo_version" ]]; then
  echo "❌ Git tag '$tag' does not match Cargo.toml version '$cargo_version'"
  exit 1
fi

echo "✅ Git tag matches Cargo.toml version ($cargo_version)"
