# 🪟 Building on Windows

Complete guide for building Paddle Decoder on Windows.

---

## 📦 Prerequisites

### **1. Install Rust**

Download and run the installer from: https://rustup.rs

Or use PowerShell:
```powershell
# Download rustup-init.exe and run it
# Follow the on-screen instructions
```

After installation, open a new PowerShell/Command Prompt and verify:
```powershell
rustc --version
cargo --version
```

### **2. Install Visual Studio Build Tools**

Rust on Windows requires the Microsoft C++ build tools.

**Option A: Visual Studio 2022 (Recommended)**
- Download from: https://visualstudio.microsoft.com/downloads/
- Install "Desktop development with C++" workload
- Includes all required components

**Option B: Build Tools Only (Minimal)**
- Download "Build Tools for Visual Studio 2022"
- Install "C++ build tools" workload
- Smaller download, command-line only

### **3. No additional dependencies needed!**

Windows includes everything else needed:
- ✅ Audio support (Windows Audio Session API)
- ✅ GUI support (built into Windows)
- ✅ MIDI support (Windows MIDI API)

---

## 🔨 Building

### **1. Open PowerShell or Command Prompt**

Navigate to project directory:
```powershell
cd C:\path\to\paddle_decoder_cross_platform
```

### **2. Build the application**

```powershell
cargo build --release
```

First build takes 3-5 minutes. Subsequent builds are instant.

### **3. The executable will be at:**
```powershell
target\release\paddle_decoder.exe
```

---

## 🚀 Running

### **Method 1: Using cargo**
```powershell
cargo run --release
```

### **Method 2: Run the .exe directly**
```powershell
.\target\release\paddle_decoder.exe
```

### **Method 3: Double-click**
- Navigate to `target\release\` in File Explorer
- Double-click `paddle_decoder.exe`

---

## 🔧 Platform-Specific Issues

### **Issue: "No MIDI device found"**

**Check Device Manager:**
1. Open Device Manager (Win+X → Device Manager)
2. Look under "Sound, video and game controllers"
3. Should see "USB Audio Device" or similar when Digispark is plugged in

**Install Digispark Drivers (Windows 10/11):**

Modern Windows usually auto-installs, but if needed:
1. Download Digispark driver installer
2. Run as Administrator
3. Plug in Digispark
4. Wait for driver installation

**Check MIDI device:**
```powershell
# In PowerShell:
Get-PnpDevice | Where-Object {$_.FriendlyName -like "*MIDI*"}
```

---

### **Issue: "VCRUNTIME140.dll not found"**

**Cause:** Missing Visual C++ Redistributable

**Solution:**
Download and install:
- Microsoft Visual C++ 2015-2022 Redistributable (x64)
- From: https://aka.ms/vs/17/release/vc_redist.x64.exe

---

### **Issue: No sound**

**Check Windows Sound Settings:**
1. Right-click speaker icon in taskbar
2. Open Sound Settings
3. Verify output device is selected
4. Test with other applications

**Check Volume Mixer:**
1. Right-click speaker icon
2. Open Volume Mixer
3. Verify application isn't muted

---

### **Issue: Windows Defender blocks the application**

**If SmartScreen blocks:**
1. Click "More info"
2. Click "Run anyway"

**If Defender quarantines:**
1. Open Windows Security
2. Virus & threat protection
3. Protection history
4. Restore the file
5. Add to exclusions if needed

---

## 🎯 Windows-Specific Features

### **High DPI Support**
Application automatically scales on high-DPI displays.

### **Dark Mode**
Follows Windows dark/light theme automatically.

### **Multiple Audio Devices**
Automatically uses Windows default audio output device.

---

## 📋 Quick Test Script

Create `test_windows.ps1`:

```powershell
# Windows System Test
Write-Host "=== Windows System Test ===" -ForegroundColor Cyan
Write-Host ""

# 1. Check Rust
Write-Host "1. Checking Rust..." -ForegroundColor Yellow
if (Get-Command rustc -ErrorAction SilentlyContinue) {
    $version = rustc --version
    Write-Host "   ✓ Rust installed: $version" -ForegroundColor Green
} else {
    Write-Host "   ✗ Rust not found" -ForegroundColor Red
    Write-Host "   Install from: https://rustup.rs" -ForegroundColor Yellow
    exit 1
}

# 2. Check Build Tools
Write-Host "2. Checking Visual C++ Build Tools..." -ForegroundColor Yellow
if (Test-Path "C:\Program Files (x86)\Microsoft Visual Studio\") {
    Write-Host "   ✓ Visual Studio found" -ForegroundColor Green
} else {
    Write-Host "   ⚠ Visual Studio not detected" -ForegroundColor Yellow
    Write-Host "   May need C++ build tools" -ForegroundColor Yellow
}

# 3. Check Digispark
Write-Host "3. Checking USB devices..." -ForegroundColor Yellow
$devices = Get-PnpDevice | Where-Object {$_.InstanceId -like "*VID_16C0*"}
if ($devices) {
    Write-Host "   ✓ Digispark device found" -ForegroundColor Green
} else {
    Write-Host "   ⚠ Digispark not found - plug it in" -ForegroundColor Yellow
}

# 4. Check Audio
Write-Host "4. Checking audio devices..." -ForegroundColor Yellow
$audio = Get-CimInstance Win32_SoundDevice
if ($audio) {
    Write-Host "   ✓ Audio devices found" -ForegroundColor Green
} else {
    Write-Host "   ✗ No audio devices" -ForegroundColor Red
}

Write-Host ""
Write-Host "=== Test complete ===" -ForegroundColor Cyan
```

Run in PowerShell:
```powershell
.\test_windows.ps1
```

---

## 🎮 Creating a Desktop Shortcut

### **Method 1: Manual**
1. Navigate to `target\release\`
2. Right-click `paddle_decoder.exe`
3. Send to → Desktop (create shortcut)

### **Method 2: PowerShell Script**

Create `create_shortcut.ps1`:
```powershell
$WshShell = New-Object -comObject WScript.Shell
$Shortcut = $WshShell.CreateShortcut("$Home\Desktop\Paddle Decoder.lnk")
$Shortcut.TargetPath = "$PWD\target\release\paddle_decoder.exe"
$Shortcut.WorkingDirectory = "$PWD\target\release"
$Shortcut.Save()
Write-Host "Shortcut created on Desktop!"
```

Run:
```powershell
.\create_shortcut.ps1
```

---

## 📦 Creating a Portable Package

Build a portable version:

```powershell
# Build release
cargo build --release

# Create package folder
mkdir paddle_decoder_portable
cd paddle_decoder_portable

# Copy executable
copy ..\target\release\paddle_decoder.exe .

# Create README
echo "Paddle Decoder - Portable Windows Version" > README.txt
echo "Double-click paddle_decoder.exe to run" >> README.txt
echo "Make sure Digispark is plugged in!" >> README.txt

# Zip it
Compress-Archive -Path * -DestinationPath ..\paddle_decoder_windows.zip
```

Now you have `paddle_decoder_windows.zip` ready to share!

---

## 🚀 Ready to Go!

```powershell
cd C:\path\to\paddle_decoder_cross_platform
cargo run --release
```

Or just double-click `paddle_decoder.exe` in `target\release\`!

**73!** 📻
