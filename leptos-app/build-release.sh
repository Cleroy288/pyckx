#!/bin/sh
# Build optimized WASM release bundle
#
# Trunk's built-in wasm-opt doesn't pass
# --enable-bulk-memory (needed by Rust 1.84+).
# We disable it in index.html (data-wasm-opt="0")
# and run wasm-opt manually after trunk build.

set -e

DIST="../backend/static"

# Step 1: Trunk release build (no wasm-opt)
trunk build --release

# Step 2: Run wasm-opt with all WASM features
f=$(ls "${DIST}"/*_bg.wasm 2>/dev/null | head -1)
if [ -z "$f" ]; then
    echo "No .wasm file found in ${DIST}"
    exit 1
fi

BEFORE=$(wc -c < "$f" | tr -d ' ')

wasm-opt -Oz \
    --enable-bulk-memory \
    --enable-nontrapping-float-to-int \
    --enable-sign-ext \
    --enable-mutable-globals \
    "$f" -o "$f.opt" && mv "$f.opt" "$f"

AFTER=$(wc -c < "$f" | tr -d ' ')
SAVED=$((BEFORE - AFTER))

echo ""
echo "wasm-opt: ${BEFORE} -> ${AFTER} bytes"
echo "          saved ${SAVED} bytes"
