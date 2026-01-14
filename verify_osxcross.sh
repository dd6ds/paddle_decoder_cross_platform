#!/bin/bash
# Quick verification that osxcross is properly set up

echo "🔍 Verifying osxcross setup..."
echo ""

echo "1. Checking PATH:"
if echo "$PATH" | grep -q osxcross; then
    echo "   ✅ osxcross is in PATH"
else
    echo "   ❌ osxcross NOT in PATH"
    echo "   Run: export PATH=\"\$HOME/osxcross/target/bin:\$PATH\""
    exit 1
fi

echo ""
echo "2. Checking x86_64 clang:"
if command -v x86_64-apple-darwin23.5-clang &> /dev/null; then
    echo "   ✅ x86_64-apple-darwin23.5-clang found"
    which x86_64-apple-darwin23.5-clang
else
    echo "   ❌ x86_64-apple-darwin23.5-clang NOT found"
    exit 1
fi

echo ""
echo "3. Checking aarch64 clang:"
if command -v aarch64-apple-darwin23.5-clang &> /dev/null; then
    echo "   ✅ aarch64-apple-darwin23.5-clang found"
    which aarch64-apple-darwin23.5-clang
else
    echo "   ❌ aarch64-apple-darwin23.5-clang NOT found"
    exit 1
fi

echo ""
echo "4. Checking SDK:"
if [ -d "$HOME/osxcross/target/SDK/MacOSX14.5.sdk" ]; then
    echo "   ✅ MacOSX14.5.sdk found"
else
    echo "   ❌ SDK not found"
    exit 1
fi

echo ""
echo "5. Checking Cargo config:"
if grep -q "darwin23.5" ~/.cargo/config.toml 2>/dev/null; then
    echo "   ✅ Cargo config looks good"
    echo ""
    echo "   Configured targets:"
    grep -A 1 "target\." ~/.cargo/config.toml | head -4
else
    echo "   ❌ Cargo config needs update"
    exit 1
fi

echo ""
echo "════════════════════════════════════════════════════════════════"
echo "✅ ALL CHECKS PASSED! Ready to build macOS targets!"
echo "════════════════════════════════════════════════════════════════"
echo ""
echo "Run: ./build_all_platforms.sh"
echo ""
