#!/usr/bin/env bash
# Copyright 2026 Michael F. Collins, III
# Licensed under the Naked Time license. See LICENSE.md for details.
#
# Generates the Homebrew formula for a release.
#
# Usage: update-homebrew-formula.sh <tag> <version> <sha256-dir> <output-file>

set -euo pipefail

if [ "$#" -ne 4 ]; then
  echo "usage: $0 <tag> <version> <sha256-dir> <output-file>" >&2
  exit 2
fi

tag="$1"
version="$2"
sha_dir="$3"
output="$4"
# The tag may include a component prefix (e.g. "cli/v1.2.3"). Asset file
# names use only the final path component (e.g. "v1.2.3").
asset_tag="${tag##*/}"
template="$(cd "$(dirname "$0")/.." && pwd)/packaging/homebrew/naked-time.rb.tmpl"

contents=$(cat "$template")
contents=${contents//@TAG@/$tag}
contents=${contents//@ASSET_TAG@/$asset_tag}
contents=${contents//@VERSION@/$version}

for target in aarch64-apple-darwin x86_64-unknown-linux-gnu aarch64-unknown-linux-gnu; do
  file="$sha_dir/naked-time-${asset_tag}-${target}.tar.gz.sha256"
  if [ ! -f "$file" ]; then
    echo "error: missing checksum file $file" >&2
    exit 1
  fi
  sum=$(awk '{print $1}' "$file")
  if ! [[ "$sum" =~ ^[0-9a-f]{64}$ ]]; then
    echo "error: invalid checksum in $file" >&2
    exit 1
  fi
  placeholder="@SHA256_$(echo "$target" | tr 'a-z-' 'A-Z_')@"
  contents=${contents//$placeholder/$sum}
done

mkdir -p "$(dirname "$output")"
printf '%s\n' "$contents" > "$output"
