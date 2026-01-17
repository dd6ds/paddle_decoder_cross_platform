#!/bin/bash
# Build script for macOS

echo "╔════════════════════════════════════════════╗"
echo "║  Building Paddle Decoder for macOS       ║"
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

# Check Xcode tools
if ! xcode-select -p &> /dev/null; then
    echo "❌ Xcode Command Line Tools not found!"
    echo "   Run: xcode-select --install"
    exit 1
fi

echo "✓ Xcode tools found"
echo ""

# Detect architecture
ARCH=$(uname -m)
echo "System architecture: $ARCH"
if [ "$ARCH" = "arm64" ]; then
    echo "✓ Building for Apple Silicon (ARM64)"
elif [ "$ARCH" = "x86_64" ]; then
    echo "✓ Building for Intel (x86_64)"
fi
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
    echo "To create .app bundle, see BUILD_MACOS.md"
    echo ""
else
    echo ""
    echo "❌ Build failed!"
    exit 1
fi
