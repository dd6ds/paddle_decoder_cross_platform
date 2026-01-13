#!/bin/bash
# Build script for Linux

echo "╔════════════════════════════════════════════╗"
echo "║  Building Paddle Decoder for Linux       ║"
echo "╚════════════════════════════════════════════╝"
echo ""

# Check Rust
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust not found!"
    echo "   Install from: https://rustup.rs"
    exit 1
fi

echo "✓ Rust found: $(rustc --version)"
echo ""

# Build
echo "Building release version..."
echo ""
cargo build --release

if [ $? -eq 0 ]; then
    echo ""
    echo "╔════════════════════════════════════════════╗"
    echo "║  ✅ Build successful!                     ║"
    echo "╚════════════════════════════════════════════╝"
    echo ""
    echo "Binary location: target/release/paddle_decoder"
    echo ""
    echo "To run:"
    echo "  cargo run --release"
    echo "  OR"
    echo "  ./target/release/paddle_decoder"
    echo ""
else
    echo ""
    echo "❌ Build failed!"
    exit 1
fi
