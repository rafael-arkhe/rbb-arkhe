#!/usr/bin/env bash
set -euo pipefail
echo "Running OpenCLAW 20-node BFT simulation with 20% byzantine tolerance..."
cargo test -p arkhe-openclaw --test bft
echo "OpenCLAW BFT simulation finished successfully with >2/3 quorum."
