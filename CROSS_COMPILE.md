# 🔀 Cross-Platform Building Guide

Complete guide for building Windows binaries from Linux.

---

## 🎯 Target Selection Guide

### **Use GNU target on Linux (Recommended):**
```bash
cargo build --target x86_64-pc-windows-gnu --release
```

✅ **Benefits:**
- Works out-of-the-box on Linux
- MinGW toolchain (already installed on most systems)
- Fully compatible with all Windows systems
- Simpler setup, no extra tools needed

### **Use MSVC target on Windows:**
```bash
cargo build --target x86_64-pc-windows-msvc --release
```

✅ **Benefits:**
- Native Windows compilation
- Slightly smaller binaries
- Maximum Windows compatibility

---

## 🐧 Building Windows binaries from Linux

### **Prerequisites**

1. **Install Rust Windows target:**
```bash
rustup target add x86_64-pc-windows-gnu
```

2. **Install MinGW (if not already installed):**

**Fedora/RHEL:**
```bash
sudo dnf install mingw64-gcc mingw64-winpthreads-static
```

**Ubuntu/Debian:**
```bash
sudo apt install gcc-mingw-w64-x86-64
```

**Arch:**
```bash
sudo pacman -S mingw-w64-gcc
```

### **Build for Windows:**
```bash
cargo build --target x86_64-pc-windows-gnu --release
```

**Output:** `target/x86_64-pc-windows-gnu/release/paddle_decoder.exe`

---

## 🪟 Building on Windows (Native)

### **Prerequisites**

1. **Install Rust:** https://rustup.rs
2. **Install Visual Studio Build Tools** with "Desktop development with C++"

### **Build:**
```bash
cargo build --target x86_64-pc-windows-msvc --release
```

Or simply:
```bash
cargo build --release
```

**Output:** `target/release/paddle_decoder.exe`

---

## 🍎 Building for macOS

### **On macOS (Native):**
```bash
cargo build --release
```

### **Cross-compile from Linux (Advanced):**

Requires `osxcross` toolchain:
```bash
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin
# Additional osxcross setup required
```

---

## 📦 Complete Build Script

Create `build_all.sh` for building all platforms:

```bash
#!/bin/bash

echo "🚀 Building for all platforms..."
echo ""

# Linux (native)
echo "🐧 Building Linux binary..."
cargo build --release
echo "✅ Linux: target/release/paddle_decoder"
echo ""

# Windows (cross-compile)
echo "🪟 Building Windows binary..."
cargo build --target x86_64-pc-windows-gnu --release
echo "✅ Windows: target/x86_64-pc-windows-gnu/release/paddle_decoder.exe"
echo ""

# Create distribution folder
mkdir -p dist
cp target/release/paddle_decoder dist/paddle_decoder-linux
cp target/x86_64-pc-windows-gnu/release/paddle_decoder.exe dist/

echo ""
echo "📊 Build Summary:"
ls -lh dist/
echo ""
echo "✨ All builds complete!"
```

Make it executable:
```bash
chmod +x build_all.sh
./build_all.sh
```

---

## 🔧 Troubleshooting

### **"linker `link.exe` not found" on Linux**

**Problem:** Trying to use MSVC target on Linux  
**Solution:** Use GNU target instead:

```bash
# ❌ This won't work on Linux:
cargo build --target x86_64-pc-windows-msvc --release

# ✅ Use this instead:
cargo build --target x86_64-pc-windows-gnu --release
```

### **"winuser` not found" compile error**

**Problem:** Missing winapi features  
**Solution:** Already fixed in Cargo.toml with:

```toml
[target.'cfg(target_os = "windows")'.dependencies]
winapi = { version = "0.3", features = ["winuser", "windef", "shellapi", "wincon"] }
```

### **MinGW not found**

**Check if installed:**
```bash
which x86_64-w64-mingw32-gcc
```

**Install if missing:**
```bash
# Fedora/RHEL
sudo dnf install mingw64-gcc

# Ubuntu/Debian
sudo apt install gcc-mingw-w64-x86-64
```

---

## 🎯 Quick Reference

| **Platform** | **Command** | **Output** |
|-------------|-------------|------------|
| Linux → Linux | `cargo build --release` | `target/release/paddle_decoder` |
| Linux → Windows | `cargo build --target x86_64-pc-windows-gnu --release` | `target/x86_64-pc-windows-gnu/release/paddle_decoder.exe` |
| Windows → Windows | `cargo build --release` | `target/release/paddle_decoder.exe` |
| macOS → macOS | `cargo build --release` | `target/release/paddle_decoder` |

---

## 🚀 GitHub Actions CI/CD

Example `.github/workflows/build.yml`:

```yaml
name: Build

on: [push, pull_request]

jobs:
  build:
    strategy:
      matrix:
        include:
          - os: ubuntu-latest
            target: x86_64-unknown-linux-gnu
            artifact: paddle_decoder-linux
          - os: ubuntu-latest
            target: x86_64-pc-windows-gnu
            artifact: paddle_decoder.exe
          - os: windows-latest
            target: x86_64-pc-windows-msvc
            artifact: paddle_decoder.exe
          - os: macos-latest
            target: x86_64-apple-darwin
            artifact: paddle_decoder-macos-intel
          - os: macos-latest
            target: aarch64-apple-darwin
            artifact: paddle_decoder-macos-arm64
    
    runs-on: ${{ matrix.os }}
    
    steps:
      - uses: actions/checkout@v4
      
      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
        with:
          targets: ${{ matrix.target }}
      
      - name: Install MinGW (Linux cross-compile to Windows)
        if: matrix.os == 'ubuntu-latest' && matrix.target == 'x86_64-pc-windows-gnu'
        run: sudo apt-get install -y gcc-mingw-w64-x86-64
      
      - name: Build
        run: cargo build --release --target ${{ matrix.target }}
      
      - name: Upload artifact
        uses: actions/upload-artifact@v4
        with:
          name: ${{ matrix.artifact }}
          path: |
            target/${{ matrix.target }}/release/paddle_decoder
            target/${{ matrix.target }}/release/paddle_decoder.exe
```

---

## ✨ Summary

**For daily development on Linux:**
```bash
# Build Windows binary
cargo build --target x86_64-pc-windows-gnu --release

# Build Linux binary  
cargo build --release
```

**The GNU target works perfectly on Windows** and is the standard cross-compilation method from Linux!

---

## 📚 See Also

- `BUILD_LINUX.md` - Native Linux building
- `BUILD_WINDOWS.md` - Native Windows building
- `BUILD_MACOS.md` - Native macOS building
- `README.md` - Project overview

**73!** 📻
