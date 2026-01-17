#!/bin/bash
# Setup macOS cross-compilation on Linux
# This script sets up osxcross for building macOS binaries from Linux

set -e

echo "🍎 Setting up macOS cross-compilation environment..."
echo ""

# 1. Install dependencies
echo "📦 Installing dependencies..."
if command -v apt &> /dev/null; then
    sudo apt install -y clang cmake libxml2-dev libssl-dev zlib1g-dev
elif command -v dnf &> /dev/null; then
    sudo dnf install -y clang cmake libxml2-devel openssl-devel zlib-devel
elif command -v pacman &> /dev/null; then
    sudo pacman -S --needed clang cmake libxml2 openssl zlib
fi

# 2. Clone osxcross
if [ ! -d "$HOME/osxcross" ]; then
    echo "📥 Cloning osxcross..."
    cd ~
    git clone https://github.com/tpoechtrager/osxcross.git
else
    echo "✓ osxcross already cloned"
fi

cd ~/osxcross

# 3. Download macOS SDK
echo ""
echo "⚠️  IMPORTANT: You need a macOS SDK!"
echo "Download MacOSX SDK from:"
echo "  https://github.com/joseluisq/macosx-sdks/releases"
echo ""
echo "Recommended: MacOSX12.3.sdk.tar.xz or MacOSX13.3.sdk.tar.xz"
echo ""
echo "Place the SDK file in: ~/osxcross/tarballs/"
echo ""
read -p "Have you downloaded and placed the SDK? (y/n) " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo "Please download the SDK first, then run this script again."
    exit 1
fi

# 4. Build osxcross
echo "🔨 Building osxcross toolchain..."
./build.sh

# 5. Add to PATH (add to ~/.bashrc or ~/.zshrc)
echo ""
echo "✅ osxcross built successfully!"
echo ""
echo "Add this to your ~/.bashrc or ~/.zshrc:"
echo 'export PATH="$HOME/osxcross/target/bin:$PATH"'
echo ""

# 6. Setup Cargo config for macOS targets
echo "📝 Creating Cargo config for macOS cross-compilation..."
mkdir -p ~/.cargo

cat >> ~/.cargo/config.toml << 'EOF'

# macOS cross-compilation (osxcross)
[target.x86_64-apple-darwin]
linker = "x86_64-apple-darwin20.4-clang"
ar = "x86_64-apple-darwin20.4-ar"

[target.aarch64-apple-darwin]
linker = "aarch64-apple-darwin20.4-clang"
ar = "aarch64-apple-darwin20.4-ar"
EOF

echo ""
echo "✨ Setup complete!"
echo ""
echo "Next steps:"
echo "  1. Add osxcross to PATH (see above)"
echo "  2. Restart your terminal or run: source ~/.bashrc"
echo "  3. Add Rust targets: rustup target add x86_64-apple-darwin aarch64-apple-darwin"
echo "  4. Build: cargo build --release --target x86_64-apple-darwin"
echo ""
