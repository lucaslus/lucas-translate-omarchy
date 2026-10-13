#!/usr/bin/env bash
# Build and bundle the Omarchy variant without the desktop app or its release pipeline.
set -euo pipefail
task_root=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)
cd "$task_root"
[[ $(uname -s) == Linux ]] || { echo "Build Omarchy release bundles on Linux." >&2; exit 1; }
cargo build --release --locked --package lucas-translate-omarchy-backend
version=$(python3 -c 'import json; print(json.load(open("manifest.json"))["version"])')
out="$task_root/dist"
mkdir -p "$out"
stage=$(mktemp -d "$out/build.XXXXXX")
trap 'rm -rf -- "$stage"' EXIT
bundle="$stage/lucas-translate-omarchy"
python3 scripts/plugin_files.py "$bundle"
mkdir -p "$bundle/bin" "$bundle/scripts"
install -m755 target/release/lucas-translate-omarchy-backend "$bundle/bin/"
install -m755 scripts/install-omarchy-native.py "$bundle/scripts/"
install -m644 scripts/plugin_files.py "$bundle/scripts/"
tar -czf "$out/lucas-translate-omarchy-$version-$(uname -m).tar.gz" -C "$stage" lucas-translate-omarchy
echo "$out/lucas-translate-omarchy-$version-$(uname -m).tar.gz"
