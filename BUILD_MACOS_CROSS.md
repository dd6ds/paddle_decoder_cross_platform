# 🍎 macOS Cross-Compilation Guide

Building macOS binaries (x86_64 and aarch64) from Linux using osxcross.

---

## ⚡ Quick Commands (If osxcross is already set up)

```bash
# Simple: Use the build script
./build_macos_cross.sh

# Or manually:
rustup target add x86_64-apple-darwin aarch64-apple-darwin
cargo build --release --target x86_64-apple-darwin
cargo build --release --target aarch64-apple-darwin
```

**Output:**
- `target/x86_64-apple-darwin/release/paddle_decoder` (Intel Macs)
- `target/aarch64-apple-darwin/release/paddle_decoder` (Apple Silicon M1/M2/M3)

---

## 🛠️ First-Time Setup (Complete Guide)

### **Step 1: Install Dependencies**

**Manjaro/Arch:**
```bash
sudo pacman -S clang cmake libxml2 openssl zlib git
```

**Ubuntu/Debian:**
```bash
sudo apt install clang cmake libxml2-dev libssl-dev zlib1g-dev git
```

**Fedora/RHEL:**
```bash
sudo dnf install clang cmake libxml2-devel openssl-devel zlib-devel git
```

### **Step 2: Download macOS SDK**

You need an official macOS SDK. Get it from:
- https://github.com/joseluisq/macosx-sdks/releases

Recommended versions:
- **MacOSX12.3.sdk.tar.xz** (for older compatibility)
- **MacOSX13.3.sdk.tar.xz** (for newer features)

```bash
# Download example (adjust version as needed):
cd ~/Downloads
wget https://github.com/joseluisq/macosx-sdks/releases/download/13.3/MacOSX13.3.sdk.tar.xz
```

### **Step 3: Clone and Build osxcross**

```bash
# Clone osxcross
cd ~
git clone https://github.com/tpoechtrager/osxcross.git
cd osxcross

# Copy SDK to osxcross
mkdir -p tarballs
cp ~/Downloads/MacOSX*.sdk.tar.xz tarballs/

# Build osxcross toolchain (takes 5-10 minutes)
./build.sh
```

### **Step 4: Add osxcross to PATH**

Add to `~/.bashrc` or `~/.zshrc`:

```bash
echo 'export PATH="$HOME/osxcross/target/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

**Verify it works:**
```bash
x86_64-apple-darwin20.4-clang --version
```

### **Step 5: Configure Cargo**

Create or edit `~/.cargo/config.toml`:

```toml
# macOS cross-compilation (osxcross)
[target.x86_64-apple-darwin]
linker = "x86_64-apple-darwin20.4-clang"
ar = "x86_64-apple-darwin20.4-ar"

[target.aarch64-apple-darwin]
linker = "aarch64-apple-darwin20.4-clang"
ar = "aarch64-apple-darwin20.4-ar"
```

**Or run this command:**
```bash
cat >> ~/.cargo/config.toml << 'EOF'

[target.x86_64-apple-darwin]
linker = "x86_64-apple-darwin20.4-clang"
ar = "x86_64-apple-darwin20.4-ar"

[target.aarch64-apple-darwin]
linker = "aarch64-apple-darwin20.4-clang"
ar = "aarch64-apple-darwin20.4-ar"
EOF
```

### **Step 6: Add Rust Targets**

```bash
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin
```

### **Step 7: Build!**

```bash
cd /home/developer/rust/paddle_decoder_cross_platform
./build_macos_cross.sh
```

**Or manually:**
```bash
cargo build --release --target x86_64-apple-darwin
cargo build --release --target aarch64-apple-darwin
```

---

## 📦 Automated Setup Script

We've included a setup script that automates most of the process:

```bash
./setup_macos_crosscompile.sh
```

This script will:
1. ✅ Install system dependencies
2. ✅ Clone osxcross
3. ⚠️  Prompt you to download the SDK manually
4. ✅ Build osxcross toolchain
5. ✅ Configure Cargo

**Note:** You still need to download the macOS SDK manually (licensing reasons).

---

## 🚀 Building for macOS

### **Option 1: Using the build script (Recommended)**

```bash
./build_macos_cross.sh
```

### **Option 2: Manual commands**

```bash
# For Intel Macs (x86_64)
cargo build --release --target x86_64-apple-darwin

# For Apple Silicon (M1/M2/M3 - aarch64)
cargo build --release --target aarch64-apple-darwin

# Find your binaries:
ls -lh target/x86_64-apple-darwin/release/paddle_decoder
ls -lh target/aarch64-apple-darwin/release/paddle_decoder
```

---

## 📂 Output Locations


After building, binaries are located at:

```
target/x86_64-apple-darwin/release/paddle_decoder      # Intel Macs
target/aarch64-apple-darwin/release/paddle_decoder     # Apple Silicon
```

The build script copies them to:
```
release/paddle_decoder_x86_64_macOS                    # Intel Macs
release/paddle_decoder_aarch64_macOS                   # Apple Silicon
```

---

## 🔧 Troubleshooting

### **"x86_64-apple-darwin20.4-clang not found"**

**Problem:** osxcross not in PATH

**Solution:**
```bash
export PATH="$HOME/osxcross/target/bin:$PATH"
# Add to ~/.bashrc to make permanent
echo 'export PATH="$HOME/osxcross/target/bin:$PATH"' >> ~/.bashrc
```

### **"SDK not found" during osxcross build**

**Problem:** SDK tarball not in correct location

**Solution:**
```bash
# SDK must be in osxcross/tarballs/
ls ~/osxcross/tarballs/MacOSX*.sdk.tar.xz
# If not there, copy it:
cp ~/Downloads/MacOSX*.sdk.tar.xz ~/osxcross/tarballs/
```

### **"linker failed" during cargo build**

**Problem:** Cargo config not set correctly

**Solution:** Verify `~/.cargo/config.toml` contains:
```toml
[target.x86_64-apple-darwin]
linker = "x86_64-apple-darwin20.4-clang"
ar = "x86_64-apple-darwin20.4-ar"
```

### **Checking osxcross installation**

```bash
# Check if tools are available
which x86_64-apple-darwin20.4-clang
which aarch64-apple-darwin20.4-clang

# Test compilation
echo 'int main() { return 0; }' > test.c
x86_64-apple-darwin20.4-clang test.c -o test
file test  # Should show "Mach-O 64-bit x86_64 executable"
rm test test.c
```

---

## 🎯 Quick Reference

| **Command** | **Output** | **Architecture** |
|------------|------------|------------------|
| `./build_macos_cross.sh` | Both binaries | Intel + ARM |
| `cargo build --release --target x86_64-apple-darwin` | `target/x86_64-apple-darwin/release/paddle_decoder` | Intel (x86_64) |
| `cargo build --release --target aarch64-apple-darwin` | `target/aarch64-apple-darwin/release/paddle_decoder` | Apple Silicon (ARM64) |

---

## 📋 Dependencies Summary

**System packages:**
- clang
- cmake  
- libxml2
- openssl
- zlib

**Rust targets:**
- x86_64-apple-darwin
- aarch64-apple-darwin

**External tools:**
- osxcross (macOS cross-compiler toolchain)
- macOS SDK (from Apple, via third-party mirror)

---

## 🌟 Complete Build Command for All Platforms


Want to build for all platforms from Linux? Here's the ultimate build script:

```bash
#!/bin/bash
# Build for all platforms: Linux, Windows, and macOS

set -e

echo "🚀 Building paddle_decoder for ALL platforms..."
echo ""

# Linux (native)
echo "🐧 Building for Linux..."
cargo build --release
cp target/release/paddle_decoder release/paddle_decoder_linux_amd64

# Windows (cross-compile)
echo "🪟 Building for Windows..."
cargo build --release --target x86_64-pc-windows-gnu
cp target/x86_64-pc-windows-gnu/release/paddle_decoder.exe release/paddle_decoder_win64_gnu.exe

# macOS Intel (cross-compile)
echo "🍎 Building for macOS Intel..."
cargo build --release --target x86_64-apple-darwin
cp target/x86_64-apple-darwin/release/paddle_decoder release/paddle_decoder_x86_64_macOS

# macOS Apple Silicon (cross-compile)
echo "🍏 Building for macOS Apple Silicon..."
cargo build --release --target aarch64-apple-darwin
cp target/aarch64-apple-darwin/release/paddle_decoder release/paddle_decoder_aarch64_macOS

echo ""
echo "✅ ALL BUILDS COMPLETE!"
echo ""
echo "📊 Binary Summary:"
ls -lh release/paddle_decoder_*
echo ""
echo "🎯 Ready to distribute:"
echo "  Linux:               release/paddle_decoder_linux_amd64"
echo "  Windows:             release/paddle_decoder_win64_gnu.exe"
echo "  macOS (Intel):       release/paddle_decoder_x86_64_macOS"
echo "  macOS (Apple Silicon): release/paddle_decoder_aarch64_macOS"
```

---

## 📚 See Also

- [BUILD_LINUX.md](BUILD_LINUX.md) - Native Linux building
- [BUILD_WINDOWS.md](BUILD_WINDOWS.md) - Native Windows building  
- [BUILD_MACOS.md](BUILD_MACOS.md) - Native macOS building
- [CROSS_COMPILE.md](CROSS_COMPILE.md) - Windows cross-compilation
- [README.md](README.md) - Project overview

---

## ✨ Tips

**Optimize binary size:**
```bash
# Add to Cargo.toml
[profile.release]
opt-level = "z"     # Optimize for size
lto = true          # Link-time optimization
strip = true        # Strip symbols
```

**Check binary architecture:**
```bash
file target/x86_64-apple-darwin/release/paddle_decoder
# Output: Mach-O 64-bit executable x86_64

file target/aarch64-apple-darwin/release/paddle_decoder
# Output: Mach-O 64-bit executable arm64
```

**Test on macOS:**
- Transfer binary to macOS machine
- Make executable: `chmod +x paddle_decoder`
- Right-click → Open (first time only, to bypass Gatekeeper)
- Or: `xattr -cr paddle_decoder` to remove quarantine flag

---

**73!** 📻
