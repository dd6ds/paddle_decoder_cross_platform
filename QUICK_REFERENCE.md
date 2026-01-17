# 🚀 Quick Reference - Paddle Decoder INO Files

## 📦 Files Created

| File | Purpose | Best For |
|------|---------|----------|
| `paddle_decoder_vusb.ino` | Main version | **TRY THIS FIRST** |
| `paddle_decoder_alt.ino` | Alternative impl | If first doesn't work |
| `paddle_decoder_diagnostic.ino` | Debug version | Troubleshooting |

## ⚡ Quick Upload Steps

```bash
# 1. Open Arduino IDE
# 2. File -> Open -> paddle_decoder_vusb.ino
# 3. Tools -> Board -> Digispark (Default - 16.5mhz)
# 4. Sketch -> Upload
# 5. Plug in Digispark when prompted
# 6. Wait for "Micronucleus done. Thank you!"
```

## 🔌 Pin Connections

```
P2 (Pin 2)  →  LEFT paddle (Dit)
P0 (Pin 0)  →  RIGHT paddle (Dah)
GND         →  Common ground
P1 (Pin 1)  →  LED (optional)
```

## 🎵 MIDI Notes

| Version | Dit Note | Dah Note | Channel |
|---------|----------|----------|---------|
| **All new versions** | 60 (C) | 62 (D) | 1 |
| Original | 1 | 2 | 1 |

⚠️ **Important**: Update your Rust code if it expects notes 1 and 2!

## 🔍 Quick Test Commands

```bash
# Check USB device
lsusb | grep "16c0:05e4"

# Check MIDI device
amidi -l

# Monitor MIDI messages
aseqdump -p "Digispark"

# Run your app
cd /home/developer/rust/paddle_decoder_cross_platform
cargo run --release
```

## 🚦 LED Indicators

### paddle_decoder_vusb.ino
- 5 fast blinks = Ready
- Quick flash = Paddle press

### paddle_decoder_diagnostic.ino
- 5 fast blinks = Firmware OK
- 1 long blink = USB init
- 3 medium blinks = MIDI ready
- Flash every 10s = Heartbeat
- Continuous = Both paddles pressed

## 🐛 Quick Troubleshooting

| Problem | Solution |
|---------|----------|
| No USB device | Try different USB port, check udev rules |
| No MIDI device | Load module: `sudo modprobe snd-seq` |
| No LED blink | Re-upload firmware |
| Wrong notes | Update Rust code or change INO defines |
| Bouncing | Increase debounce time in code |

## 🔧 udev Rule (if needed)

```bash
# Create file:
sudo nano /etc/udev/rules.d/49-digispark.rules

# Add:
SUBSYSTEMS=="usb", ATTRS{idVendor}=="16c0", ATTRS{idProduct}=="05e4", MODE:="0666"

# Reload:
sudo udevadm control --reload-rules
sudo udevadm trigger
```

## 📝 Modify for Notes 1 & 2 (keep Rust code unchanged)

In any INO file, change:
```cpp
#define NOTE_DIT  60    // Change to: 1
#define NOTE_DAH  62    // Change to: 2
```

## 📊 Success Verification

✅ Arduino upload says "Micronucleus done"  
✅ LED blinks on startup  
✅ `lsusb` shows 16c0:05e4  
✅ `amidi -l` shows MIDI device  
✅ LED flashes when paddle pressed  
✅ `aseqdump` shows Note On/Off  
✅ Rust app receives events  

## 🎯 Decision Tree

```
Want to test quickly?
  └─> Use paddle_decoder_vusb.ino

Having problems?
  └─> Try paddle_decoder_alt.ino

Need to debug?
  └─> Use paddle_decoder_diagnostic.ino

Want original behavior (notes 1,2)?
  └─> Edit NOTE_DIT and NOTE_DAH in any file
```

## 📚 Full Documentation

See `INO_README.md` for complete details, troubleshooting, and explanations.

## 🔗 Key Differences from Original

✅ Better debouncing  
✅ Improved MIDI timing  
✅ LED pulse feedback (not continuous)  
✅ Multiple versions for different needs  
✅ Better USB enumeration  
✅ Diagnostic support  
✅ Proper state management  
✅ Compatible with VID:PID 16c0:05e4  

---

**73!** 📻 DD6DS

*For ham radio CW training*
