#!/bin/bash
# Build paddle_decoder for ALL platforms from Linux
# Targets: Linux, Windows, macOS (Intel + Apple Silicon)

set -e

echo "🚀 Building paddle_decoder for ALL platforms..."
echo ""

# Check prerequisites
echo "🔍 Checking prerequisites..."

# Check for osxcross
if ! command -v x86_64-apple-darwin20.4-clang &> /dev/null; then
    echo "⚠️  Warning: osxcross not found in PATH - macOS builds will be skipped"
    echo "   To enable macOS builds, run: ./setup_macos_crosscompile.sh"
    SKIP_MACOS=1
else
    echo "✓ osxcross found"
    SKIP_MACOS=0
fi

# Check for MinGW (64-bit and 32-bit)
MINGW_64_AVAILABLE=0
MINGW_32_AVAILABLE=0
MSVC_AVAILABLE=0

if command -v x86_64-w64-mingw32-gcc &> /dev/null; then
    echo "✓ MinGW-w64 (64-bit) found"
    MINGW_64_AVAILABLE=1
fi

if command -v i686-w64-mingw32-gcc &> /dev/null; then
    echo "✓ MinGW-w64 (32-bit) found"
    MINGW_32_AVAILABLE=1
fi

# Check for MSVC linker (only works on Windows or with Wine)
if command -v link.exe &> /dev/null 2>&1 || [ -n "$WINEPREFIX" ]; then
    echo "✓ MSVC toolchain found"
    MSVC_AVAILABLE=1
fi

if [ $MINGW_64_AVAILABLE -eq 0 ] && [ $MINGW_32_AVAILABLE -eq 0 ] && [ $MSVC_AVAILABLE -eq 0 ]; then
    echo "⚠️  Warning: No Windows toolchains found - Windows builds will be skipped"
    echo "   Install MinGW with: sudo pacman -S mingw-w64-gcc"
fi

echo ""

# Create release directory
mkdir -p release

# Linux (native)
echo "🐧 Building for Linux x86_64..."
cargo build --release
cp target/release/paddle_decoder release/paddle_decoder_linux_amd64
echo "✅ Linux build complete"
echo ""

# Windows builds (cross-compile)
if [ $MINGW_64_AVAILABLE -eq 1 ] || [ $MINGW_32_AVAILABLE -eq 1 ] || [ $MSVC_AVAILABLE -eq 1 ]; then
    echo "════════════════════════════════════════════════════"
    echo "🪟 Building Windows binaries..."
    echo "════════════════════════════════════════════════════"
    echo ""
    
    # Windows x86_64 GNU
    if [ $MINGW_64_AVAILABLE -eq 1 ]; then
        echo "Building Windows x86_64 (GNU)..."
        rustup target add x86_64-pc-windows-gnu 2>/dev/null || true
        cargo build --release --target x86_64-pc-windows-gnu
        cp target/x86_64-pc-windows-gnu/release/paddle_decoder.exe release/paddle_decoder_win64_gnu.exe
        echo "✅ Windows x86_64 GNU build complete"
        echo ""
    fi
    
    # Windows i686 GNU (32-bit)
    if [ $MINGW_32_AVAILABLE -eq 1 ]; then
        echo "Building Windows i686 (32-bit GNU)..."
        rustup target add i686-pc-windows-gnu 2>/dev/null || true
        cargo build --release --target i686-pc-windows-gnu
        cp target/i686-pc-windows-gnu/release/paddle_decoder.exe release/paddle_decoder_win32_gnu.exe
        echo "✅ Windows i686 GNU build complete"
        echo ""
    fi
    
    # Windows x86_64 MSVC
    if [ $MSVC_AVAILABLE -eq 1 ]; then
        echo "Building Windows x86_64 (MSVC)..."
        rustup target add x86_64-pc-windows-msvc 2>/dev/null || true
        cargo build --release --target x86_64-pc-windows-msvc
        cp target/x86_64-pc-windows-msvc/release/paddle_decoder.exe release/paddle_decoder_win64_msvc.exe
        echo "✅ Windows x86_64 MSVC build complete"
        echo ""
    else
        echo "⏭️  Skipping Windows MSVC build (MSVC toolchain not available on Linux)"
        echo "   Note: MSVC builds work best on native Windows"
        echo ""
    fi
else
    echo "⏭️  Skipping all Windows builds (no toolchains available)"
    echo ""
fi

# macOS Intel (cross-compile)
if [ $SKIP_MACOS -eq 0 ]; then
    echo "🍎 Building for macOS x86_64 (Intel)..."
    rustup target add x86_64-apple-darwin 2>/dev/null || true
    cargo build --release --target x86_64-apple-darwin
    cp target/x86_64-apple-darwin/release/paddle_decoder release/paddle_decoder_x86_64_macOS
    echo "✅ macOS Intel build complete"
    echo ""
    
    # macOS Apple Silicon (cross-compile)
    echo "🍏 Building for macOS aarch64 (Apple Silicon)..."
    rustup target add aarch64-apple-darwin 2>/dev/null || true
    cargo build --release --target aarch64-apple-darwin
    cp target/aarch64-apple-darwin/release/paddle_decoder release/paddle_decoder_aarch64_macOS
    echo "✅ macOS Apple Silicon build complete"
    echo ""
else
    echo "⏭️  Skipping macOS builds (osxcross not available)"
    echo ""
fi

# Summary
echo "═══════════════════════════════════════════════════"
echo "✨ BUILD SUMMARY"
echo "═══════════════════════════════════════════════════"
echo ""
echo "📊 Binary sizes:"
ls -lh release/paddle_decoder_* 2>/dev/null || echo "No binaries built"
echo ""
echo "🎯 Built platforms:"
if [ -f "release/paddle_decoder_linux_amd64" ]; then
    echo "  ✅ Linux (x86_64):              release/paddle_decoder_linux_amd64"
fi
if [ -f "release/paddle_decoder_win64_gnu.exe" ]; then
    echo "  ✅ Windows x86_64 (GNU):        release/paddle_decoder_win64_gnu.exe"
fi
if [ -f "release/paddle_decoder_win32_gnu.exe" ]; then
    echo "  ✅ Windows i686/32-bit (GNU):   release/paddle_decoder_win32_gnu.exe"
fi
if [ -f "release/paddle_decoder_win64_msvc.exe" ]; then
    echo "  ✅ Windows x86_64 (MSVC):       release/paddle_decoder_win64_msvc.exe"
fi
if [ -f "release/paddle_decoder_x86_64_macOS" ]; then
    echo "  ✅ macOS Intel (x86_64):        release/paddle_decoder_x86_64_macOS"
fi
if [ -f "release/paddle_decoder_aarch64_macOS" ]; then
    echo "  ✅ macOS Apple Silicon (ARM64): release/paddle_decoder_aarch64_macOS"
fi
echo ""

# Count built platforms
BUILT_COUNT=$(ls release/paddle_decoder_* 2>/dev/null | wc -l)
echo "📦 Total platforms built: $BUILT_COUNT"
echo ""
