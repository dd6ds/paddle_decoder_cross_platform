#!/bin/bash
# Build paddle_decoder for macOS (x86_64 and aarch64)
# Requires osxcross to be set up first

set -e

echo "🍎 Building paddle_decoder for macOS..."
echo ""

# Check if osxcross is in PATH
if ! command -v x86_64-apple-darwin20.4-clang &> /dev/null; then
    echo "❌ Error: osxcross not found in PATH"
    echo ""
    echo "Please ensure osxcross is installed and added to PATH:"
    echo '  export PATH="$HOME/osxcross/target/bin:$PATH"'
    echo ""
    echo "If not installed, run: ./setup_macos_crosscompile.sh"
    exit 1
fi

# Add Rust targets if not already added
echo "📦 Ensuring Rust targets are installed..."
rustup target add x86_64-apple-darwin 2>/dev/null || true
rustup target add aarch64-apple-darwin 2>/dev/null || true

# Build for x86_64 (Intel Macs)
echo ""
echo "🔨 Building for macOS x86_64 (Intel)..."
cargo build --release --target x86_64-apple-darwin

# Build for aarch64 (Apple Silicon - M1/M2/M3)
echo ""
echo "🔨 Building for macOS aarch64 (Apple Silicon)..."
cargo build --release --target aarch64-apple-darwin

# Copy to release folder
echo ""
echo "📦 Copying binaries to release folder..."
mkdir -p release
cp target/x86_64-apple-darwin/release/paddle_decoder release/paddle_decoder_x86_64_macOS
cp target/aarch64-apple-darwin/release/paddle_decoder release/paddle_decoder_aarch64_macOS

echo ""
echo "✅ Build complete!"
echo ""
echo "📊 Binary sizes:"
ls -lh release/paddle_decoder_*_macOS
echo ""
echo "🎯 Binaries ready:"
echo "  Intel Macs:      release/paddle_decoder_x86_64_macOS"
echo "  Apple Silicon:   release/paddle_decoder_aarch64_macOS"
echo ""
