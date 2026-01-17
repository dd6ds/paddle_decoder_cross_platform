# 🍎 Building on macOS

Complete guide for building Paddle Decoder on macOS.

---

## 📦 Prerequisites

### **1. Install Xcode Command Line Tools**

Required for compiling:
```bash
xcode-select --install
```

Click "Install" in the dialog that appears.

Verify installation:
```bash
xcode-select -p
# Should output: /Library/Developer/CommandLineTools
```

### **2. Install Rust**

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

Verify installation:
```bash
rustc --version
cargo --version
```

### **3. Install Homebrew (Optional but Recommended)**

```bash
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
```

### **4. Install Audio Dependencies**

macOS includes Core Audio, but for better compatibility:
```bash
brew install portaudio
```

---

## 🔨 Building

### **1. Navigate to project**

```bash
cd /path/to/paddle_decoder_cross_platform
```

### **2. Build the application**

```bash
cargo build --release
```

First build takes 3-5 minutes. Subsequent builds are faster.

### **3. The binary will be at:**
```bash
target/release/paddle_decoder
```

---

## 🚀 Running

### **Method 1: Using cargo**
```bash
cargo run --release
```

### **Method 2: Run the binary directly**
```bash
./target/release/paddle_decoder
```

### **Method 3: Create macOS app bundle (optional)**

See "Creating .app Bundle" section below.

---

## 🔧 Platform-Specific Issues

### **Issue: "Cannot be opened because the developer cannot be verified"**

**Cause:** macOS Gatekeeper blocking unsigned application

**Solution:**
1. Right-click (or Control-click) the app
2. Select "Open"
3. Click "Open" in the dialog
4. App will be added to exceptions

**Or via Terminal:**
```bash
xattr -cr target/release/paddle_decoder
./target/release/paddle_decoder
```

---

### **Issue: "No MIDI device found"**

**Check USB device:**
```bash
system_profiler SPUSBDataType | grep -A 10 "16c0"
```

**Check MIDI devices:**
```bash
# Install coreutils if needed:
brew install coreutils

# No built-in MIDI list command on macOS
# Use the app to check - it will list available MIDI ports
```

**Check Audio MIDI Setup:**
1. Open "Audio MIDI Setup" (in Applications/Utilities)
2. Window → Show MIDI Studio
3. Should see Digispark device

**Permissions:**
macOS may ask for MIDI access permission:
- Click "OK" when prompted
- Or: System Preferences → Security & Privacy → Privacy → Automation

---

### **Issue: No sound**

**Check System Sound Settings:**
1. System Preferences → Sound
2. Output tab
3. Select correct output device
4. Adjust volume

**Check app isn't muted:**
```bash
# Open Sound settings and check per-app volume
```

---

### **Issue: Slow performance on Apple Silicon (M1/M2/M3)**

**Build native ARM64 version:**

Check your Mac's architecture:
```bash
uname -m
# arm64 = Apple Silicon
# x86_64 = Intel
```

For Apple Silicon, build with:
```bash
# Already builds native ARM64 by default on M1/M2/M3
cargo build --release

# To explicitly target ARM64:
rustup target add aarch64-apple-darwin
cargo build --release --target aarch64-apple-darwin
```

**Binary will be at:**
```bash
target/aarch64-apple-darwin/release/paddle_decoder
```

---

## 🎯 macOS-Specific Features

### **Retina Display Support**
Application automatically supports high-DPI Retina displays.

### **Dark Mode**
Follows macOS system appearance (dark/light mode).

### **Keyboard Shortcuts**
Standard macOS shortcuts work:
- Cmd+Q: Quit
- Cmd+W: Close window
- Cmd+M: Minimize

---

## 📦 Creating .app Bundle

Create a proper macOS application:

### **1. Create directory structure:**
```bash
mkdir -p PaddleDecoder.app/Contents/MacOS
mkdir -p PaddleDecoder.app/Contents/Resources
```

### **2. Copy binary:**
```bash
cp target/release/paddle_decoder PaddleDecoder.app/Contents/MacOS/
```

### **3. Create Info.plist:**

Create `PaddleDecoder.app/Contents/Info.plist`:
```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" 
"http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleName</key>
    <string>Paddle Decoder</string>
    <key>CFBundleDisplayName</key>
    <string>Paddle Decoder</string>
    <key>CFBundleIdentifier</key>
    <string>com.amateurradio.paddledecoder</string>
    <key>CFBundleVersion</key>
    <string>1.0.0</string>
    <key>CFBundleShortVersionString</key>
    <string>1.0.0</string>
    <key>CFBundleExecutable</key>
    <string>paddle_decoder</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>NSHighResolutionCapable</key>
    <true/>
    <key>LSMinimumSystemVersion</key>
    <string>10.13</string>
</dict>
</plist>
```

### **4. Make executable:**
```bash
chmod +x PaddleDecoder.app/Contents/MacOS/paddle_decoder
```

### **5. Run it:**
```bash
open PaddleDecoder.app
```

Or drag to Applications folder!

---

## 📋 Quick Test Script

Create `test_macos.sh`:

```bash
#!/bin/bash
echo "=== macOS System Test ==="
echo ""

# 1. Check Rust
echo "1. Checking Rust..."
if command -v rustc >/dev/null 2>&1; then
    echo "   ✓ Rust installed: $(rustc --version)"
else
    echo "   ✗ Rust not found"
    echo "   Install from: https://rustup.rs"
    exit 1
fi

# 2. Check Xcode tools
echo "2. Checking Xcode Command Line Tools..."
if xcode-select -p >/dev/null 2>&1; then
    echo "   ✓ Xcode tools installed"
else
    echo "   ✗ Xcode tools not found"
    echo "   Run: xcode-select --install"
    exit 1
fi

# 3. Check Digispark
echo "3. Checking Digispark..."
if system_profiler SPUSBDataType 2>/dev/null | grep -q "16c0"; then
    echo "   ✓ Digispark connected"
else
    echo "   ⚠ Digispark not found - plug it in"
fi

# 4. Check audio
echo "4. Checking audio..."
if system_profiler SPAudioDataType >/dev/null 2>&1; then
    echo "   ✓ Audio devices available"
else
    echo "   ✗ No audio devices"
fi

# 5. Check architecture
echo "5. System architecture: $(uname -m)"
if [ "$(uname -m)" = "arm64" ]; then
    echo "   ✓ Apple Silicon detected - will build native ARM64"
elif [ "$(uname -m)" = "x86_64" ]; then
    echo "   ✓ Intel Mac detected"
fi

echo ""
echo "=== Test complete ==="
```

Make executable and run:
```bash
chmod +x test_macos.sh
./test_macos.sh
```

---

## 🎨 Building for Distribution

### **Universal Binary (Intel + Apple Silicon):**

```bash
# Add targets
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin

# Build both
cargo build --release --target x86_64-apple-darwin
cargo build --release --target aarch64-apple-darwin

# Create universal binary
lipo -create \
    target/x86_64-apple-darwin/release/paddle_decoder \
    target/aarch64-apple-darwin/release/paddle_decoder \
    -output paddle_decoder_universal

# Verify
lipo -info paddle_decoder_universal
# Should show: x86_64 arm64
```

---

## 🔐 Code Signing (Optional)

For distribution outside App Store:

```bash
# Check if you have a Developer ID
security find-identity -v -p codesigning

# Sign the app
codesign --force --deep --sign "Developer ID Application: Your Name" \
    PaddleDecoder.app

# Verify
codesign -v PaddleDecoder.app
spctl -a -v PaddleDecoder.app
```

---

## 📦 Creating DMG Installer

Create a distributable DMG:

```bash
# Install create-dmg
brew install create-dmg

# Create DMG
create-dmg \
    --volname "Paddle Decoder" \
    --window-pos 200 120 \
    --window-size 600 400 \
    --icon-size 100 \
    --app-drop-link 450 185 \
    PaddleDecoder.dmg \
    PaddleDecoder.app
```

---

## 🚀 Ready to Go!

```bash
cd /path/to/paddle_decoder_cross_platform
cargo run --release
```

Or open the .app bundle you created!

**73!** 📻
