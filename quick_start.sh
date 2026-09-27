#!/bin/bash
# Quick Start - Minimal setup for testing

set -e

SITE="${1:-https://example.com}"
OUTPUT="${2:-reports/quick-report.xlsx}"

echo "=== SEO Auditor Quick Start ==="
echo "Site: $SITE"
echo "Output: $OUTPUT"

# Check for Rust
if ! command -v cargo &> /dev/null; then
    echo "Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
fi

# Build
echo "Building..."
cargo build --release

# Run (no external services needed for basic SEO)
mkdir -p reports
./target/release/seo-semantic-auditor scan     --site "$SITE"     --query queries/comprehensive.yaml     --output "$OUTPUT"

echo "Done! Report: $OUTPUT"
