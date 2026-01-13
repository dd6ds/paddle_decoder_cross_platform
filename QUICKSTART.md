# 🚀 QUICK START GUIDE

**Get up and running on Linux, Windows, or macOS in 5 minutes!**

---

## 📋 Choose Your Platform

<table>
<tr>
<td>🐧 <strong>Linux</strong></td>
<td>🪟 <strong>Windows</strong></td>
<td>🍎 <strong>macOS</strong></td>
</tr>
<tr>
<td><a href="#-linux-quick-start">Jump to Linux →</a></td>
<td><a href="#-windows-quick-start">Jump to Windows →</a></td>
<td><a href="#-macos-quick-start">Jump to macOS →</a></td>
</tr>
</table>

---

## 🐧 Linux Quick Start

### **1. Install Dependencies**

```bash
# Ubuntu/Debian:
sudo apt install libasound2-dev pkg-config libgtk-3-dev

# Fedora:
sudo dnf install alsa-lib-devel gtk3-devel

# Arch:
sudo pacman -S alsa-lib gtk3
```

### **2. Install Rust (if not installed)**

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### **3. Build and Run**

```bash
cd paddle_decoder_cross_platform
./build_linux.sh
cargo run --release
```

**Done!** 🎉

📖 **Detailed guide:** [BUILD_LINUX.md](BUILD_LINUX.md)

---

## 🪟 Windows Quick Start

### **1. Install Rust**

Download and run: https://rustup.rs

### **2. Install Visual Studio Build Tools**

Download: https://visualstudio.microsoft.com/downloads/

Install "Desktop development with C++" workload.

### **3. Build and Run**

Open PowerShell:
```powershell
cd paddle_decoder_cross_platform
.\build_windows.ps1
cargo run --release
```

**Or just double-click `paddle_decoder.exe` in `target\release\`!**

**Done!** 🎉

📖 **Detailed guide:** [BUILD_WINDOWS.md](BUILD_WINDOWS.md)

---

## 🍎 macOS Quick Start

### **1. Install Xcode Command Line Tools**

```bash
xcode-select --install
```

### **2. Install Rust**

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### **3. Build and Run**

```bash
cd paddle_decoder_cross_platform
./build_macos.sh
cargo run --release
```

**Done!** 🎉

📖 **Detailed guide:** [BUILD_MACOS.md](BUILD_MACOS.md)

---

## 🔌 Hardware Setup (All Platforms)

### **1. Program the ATtiny85**

1. Install Arduino IDE
2. Add Digispark support:
   - File → Preferences → Additional Board Manager URLs
   - Add: `http://digistump.com/package_digistump_index.json`
3. Tools → Board Manager → Install "Digistump AVR Boards"
4. Tools → Manage Libraries → Install "DigiMIDI"
5. Open `paddle_decoder.ino`
6. Tools → Board → Digispark (Default - 16.5mhz)
7. Sketch → Upload (plug in when prompted)

### **2. Wire Your Paddle**

```
ATtiny85 Pin 2 (P2)  →  LEFT paddle (Dit)
ATtiny85 Pin 0 (P0)  →  RIGHT paddle (Dah)
ATtiny85 GND         →  Paddle common
```

### **3. Verify**

- LED blinks 5 times on startup ✓
- LED lights when you press paddle ✓

---

## 🎮 Using the Application

### **The GUI:**

```
┌────────────────────────────────────┐
│  🎹 MORSE CODE PADDLE DECODER     │
├────────────────────────────────────┤
│  WPM: [====20====]  ← Drag me!    │
│  Frequency: [=600Hz=]  ← Drag me! │
├────────────────────────────────────┤
│  LEFT (Dit)    RIGHT (Dah)        │
│     🔴            ⚪  ← Status     │
├────────────────────────────────────┤
│  Current: -.-  ← Your keying      │
├────────────────────────────────────┤
│  Decoded: HELLO  ← Your text      │
│  [Add Space] [Clear Text]         │
└────────────────────────────────────┘
```

### **Quick Test:**

**Try letter E (easiest):**
1. Press LEFT paddle
2. Release
3. Wait 300ms
4. Should show: **E** ✅

**Try letter T:**
1. Press RIGHT paddle
2. Release
3. Wait 300ms
4. Should show: **T** ✅

**Try letter D:**
1. Press RIGHT (dah)
2. Press LEFT (dit)
3. Press LEFT (dit)
4. Wait 300ms
5. Should show: **D** ✅

---

## 🔧 Troubleshooting

### **Problem: No MIDI device found**

**Check USB:**
- Linux: `lsusb | grep 16c0`
- Windows: Device Manager → USB devices
- macOS: `system_profiler SPUSBDataType | grep 16c0`

**Solution:** Plug in Digispark, verify LED blinks.

---

### **Problem: No sound**

**Test audio:**
- Linux: `speaker-test -t sine -f 600`
- Windows: Check Volume Mixer
- macOS: System Preferences → Sound

**Solution:** Check volume, unmute application.

---

### **Problem: Wrong characters**

**Example:** Type D (`-..`) but shows T

**Cause:** Timing too fast

**Solution:** 
- Pause between elements
- Wait 300ms after last element
- Try adjusting WPM slider

---

## 📚 Full Documentation

**Platform-specific guides:**
- [BUILD_LINUX.md](BUILD_LINUX.md) - Complete Linux guide
- [BUILD_WINDOWS.md](BUILD_WINDOWS.md) - Complete Windows guide  
- [BUILD_MACOS.md](BUILD_MACOS.md) - Complete macOS guide

**General:**
- [README.md](README.md) - Main documentation
- [TIMING_FIX.md](TIMING_FIX.md) - Timing adjustments

---

## 🎯 Next Steps

1. ✅ Build and run the application
2. ✅ Connect your Digispark
3. ✅ Test with E and T
4. ✅ Adjust WPM to your comfort
5. ✅ Start practicing!

---

## 💡 Tips

- **Start at 15 WPM** - Comfortable for learning
- **Try 600 Hz** - Most popular frequency
- **Pause 300ms** - Between letters for decode
- **Watch indicators** - Visual feedback helps!
- **Practice daily** - Consistency builds skill

---

## 📞 Need Help?

1. Check platform-specific guide (BUILD_*.md)
2. Review troubleshooting section above
3. Verify hardware connections
4. Test with diagnostic commands

---

## 🎉 You're Ready!

**Fire it up and start decoding!**

```bash
# Linux/macOS:
cargo run --release

# Windows:
cargo run --release
# Or double-click paddle_decoder.exe
```

**73!** 📻🎹✨
