#!/usr/bin/env bash
# Build examples/consumer — an app pinning Frame by tag, as the README shows —
# against the commit under test: its Frame git dependency is patched to this
# checkout, so the check works before the tag exists and needs no access to
# the (private) repo. Fails when the consumer's [patch] tables drift from
# Frame's, since a consumer must repeat them.
set -euo pipefail
root=$(cd "$(dirname "$0")/.." && pwd)
patches() { sed -n '/^\[patch\.crates-io\]/,/^$/p' "$1"; }
if ! diff <(patches "$root/Cargo.toml") <(patches "$root/examples/consumer/Cargo.toml"); then
  echo "examples/consumer's [patch.crates-io] differs from Frame's (above); repeat Frame's" >&2
  exit 1
fi
# Start from Frame's lock, as a new consumer should: a fresh resolve picks
# newer crates than the pins were built with (smallvec past what facet-styx
# rc.7 compiles against, 2026-11). Cargo drops what the consumer doesn't use.
cp "$root/Cargo.lock" "$root/examples/consumer/Cargo.lock"
url='https://github.com/FastTrackStudios/frame'
cargo check --manifest-path "$root/examples/consumer/Cargo.toml" --target-dir "${CARGO_TARGET_DIR:-$root/target}" \
  --config "patch.\"$url\".frame.path=\"$root/crates/frame\"" \
  --config "patch.\"$url\".frame-architect.path=\"$root/crates/frame-architect\"" \
  "$@"
echo "consumer: builds against $(git -C "$root" rev-parse --short HEAD) with Frame's patches"
