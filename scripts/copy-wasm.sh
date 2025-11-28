#!/usr/bin/env sh
set -e
SRC="cargo-workspace/crates/zksign-wasm/pkg"
DST="packages/zk-prover/wasm"
if [ -d "$SRC" ]; then
  rm -rf "$DST"
  mkdir -p "$(dirname "$DST")"
  cp -r "$SRC" "$DST"
  echo "Copied wasm pkg to $DST"
else
  echo "Source pkg not found. Build wasm first with wasm-pack." >&2
  exit 1
fi
