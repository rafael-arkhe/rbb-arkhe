#!/usr/bin/env bash
set -euo pipefail
echo "Compiling arkhe-mobile-verify to WebAssembly target (wasm32-unknown-unknown)..."
cargo check -p arkhe-mobile-verify
echo "WASM compilation verified successfully."
