#!/usr/bin/env bash
set -euo pipefail

echo "Running starter checks..."

if command -v cargo >/dev/null 2>&1; then
  cargo fmt --all --check
  cargo clippy --workspace --all-targets --all-features -- -D warnings
  cargo test --workspace
else
  echo "cargo not found; skipping Rust checks"
fi

if [[ -x ./clients/android-viewer/gradlew ]]; then
  (cd clients/android-viewer && ./gradlew assembleDebug)
else
  echo "gradle wrapper not found; skipping Android build"
fi
