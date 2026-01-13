# 🐧 Building on Linux

Complete guide for building Paddle Decoder on Linux.

---

## 📦 Prerequisites

### **1. Install Rust**

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

Verify installation:
```bash
rustc --version
cargo --version
```

### **2. Install System Dependencies**

#### **Ubuntu / Debian / Mint:**
```bash
sudo apt update
sudo apt install -y \
    libasound2-dev \
    pkg-config \
    libgtk-3-dev \
    libx11-dev \
    build-essential
```

#### **Fedora / RHEL / CentOS:**
```bash
sudo dnf install -y \
    alsa-lib-devel \
    gtk3-devel \
    libX11-devel \
    pkg-config \
    gcc \
    gcc-c++
```

#### **Arch / Manjaro:**
```bash
sudo pacman -S --needed \
    alsa-lib \
    gtk3 \
    libx11 \
    base-devel
```

#### **openSUSE:**
```bash
sudo zypper install -y \
    alsa-devel \
    gtk3-devel \
    libX11-devel \
    pkg-config \
    gcc \
    gcc-c++
```

---

## 🔨 Building

### **1. Clone or download the project**

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

### **Method 3: Install system-wide (optional)**
```bash
cargo install --path .
# Then run from anywhere:
paddle_decoder
```

---

## 🔧 Platform-Specific Issues

### **Issue: "Failed to initialize any backend"**

**Cause:** X11 display not available

**Solutions:**

1. **If on local machine:**
   ```bash
   export DISPLAY=:0
   cargo run --release
   ```

2. **If via SSH:**
   ```bash
   # Reconnect with X11 forwarding:
   ssh -X user@host
   cargo run --release
   ```

3. **Check X11 is running:**
   ```bash
   ps aux | grep X
   echo $DISPLAY
   ```

---

### **Issue: "No MIDI device found"**

**Check USB device:**
```bash
lsusb | grep "16c0:05e4"
```

**Check MIDI ports:**
```bash
# Install alsa-utils if needed:
sudo apt install alsa-utils  # Ubuntu/Debian
sudo dnf install alsa-utils  # Fedora

# List MIDI devices:
amidi -l
aconnect -l
```

**Fix permissions:**
```bash
# Add user to dialout and audio groups:
sudo usermod -a -G dialout $USER
sudo usermod -a -G audio $USER

# Log out and log back in for changes to take effect
```

---

### **Issue: No sound**

**Test audio:**
```bash
speaker-test -t sine -f 600 -c 2
# Press Ctrl+C to stop
```

**Check ALSA mixer:**
```bash
alsamixer
# Use F6 to select sound card
# Use arrow keys to adjust volume
# Press M to unmute if needed
```

**Check PulseAudio:**
```bash
# If using PulseAudio:
pavucontrol
```

**Check audio group:**
```bash
groups | grep audio
# If not in audio group:
sudo usermod -a -G audio $USER
# Log out and log back in
```

---

## 🎯 Distribution-Specific Notes

### **Ubuntu / Debian:**
- ALSA and PulseAudio usually work out of the box
- GTK3 required for GUI

### **Fedora / RHEL:**
- SELinux may block MIDI access
- Check: `sudo ausearch -m avc -ts recent`
- May need to allow MIDI in SELinux policy

### **Arch Linux:**
- Minimal by default, may need additional packages
- Consider installing `pipewire` for better audio

### **Wayland Users:**
- Application should work with XWayland
- If issues, try forcing X11:
  ```bash
  GDK_BACKEND=x11 cargo run --release
  ```

---

## 📋 Quick Test Script

Create `test_linux.sh`:

```bash
#!/bin/bash
echo "=== Linux System Test ==="
echo ""

# 1. Check Rust
echo "1. Checking Rust..."
if command -v rustc >/dev/null 2>&1; then
    echo "   ✓ Rust installed: $(rustc --version)"
else
    echo "   ✗ Rust not found - install from https://rustup.rs"
    exit 1
fi

# 2. Check X11
echo "2. Checking X11..."
if [ -n "$DISPLAY" ]; then
    echo "   ✓ DISPLAY set: $DISPLAY"
else
    echo "   ⚠ DISPLAY not set - may need: export DISPLAY=:0"
fi

# 3. Check Digispark
echo "3. Checking Digispark..."
if lsusb | grep -q "16c0:05e4"; then
    echo "   ✓ Digispark connected"
else
    echo "   ⚠ Digispark not found - plug it in"
fi

# 4. Check MIDI
echo "4. Checking MIDI..."
if command -v amidi >/dev/null 2>&1; then
    if amidi -l | grep -q "MIDI\|MidiStomp"; then
        echo "   ✓ MIDI device found"
    else
        echo "   ⚠ No MIDI device"
    fi
else
    echo "   ⚠ amidi not installed"
fi

# 5. Check audio
echo "5. Checking audio..."
if groups | grep -q audio; then
    echo "   ✓ User in audio group"
else
    echo "   ⚠ User not in audio group"
fi

echo ""
echo "=== Test complete ==="
```

Make executable and run:
```bash
chmod +x test_linux.sh
./test_linux.sh
```

---

## 🚀 Ready to Go!

```bash
cd /home/developer/rust/paddle_decoder_cross_platform
cargo run --release
```

**73!** 📻
