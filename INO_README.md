# ATtiny85 Paddle Decoder - Improved INO Files

## 🎯 Purpose

These improved INO files are specifically designed to work with:
- **USB Device**: Van Ooijen Technische Informatica Free shared USB VID/PID
- **VID:PID**: 16c0:05e4  
- **Type**: USB MIDI device
- **Hardware**: ATtiny85 Digispark board

## 📁 Available Versions

### 1. paddle_decoder_vusb.ino (RECOMMENDED)
**Best for**: Most users, first try this one

**Features**:
- Clean, well-structured code
- Proper debouncing (5ms)
- LED pulse feedback
- MIDI notes: 60 (Dit) and 62 (Dah)
- Full velocity (127) on note on
- Tested timing parameters

**When to use**: Start with this version. It has the best balance of features and reliability.

### 2. paddle_decoder_alt.ino  
**Best for**: Alternative implementation if first version has issues

**Features**:
- More modular code structure
- Extended debouncing (8ms)
- Structured state management
- Better code organization
- Diagnostic startup sequence

**When to use**: If paddle_decoder_vusb.ino doesn't work or you need more structured code for modifications.

### 3. paddle_decoder_diagnostic.ino
**Best for**: Troubleshooting and debugging

**Features**:
- Extended LED diagnostics
- Multiple startup patterns to confirm firmware load
- Heartbeat LED (flashes every 10 seconds)
- Both Note Off methods (standard and velocity 0)
- Press counting
- Special "both paddles" indicator

**LED Patterns**:
- 5 fast blinks = Firmware loaded OK
- 1 long blink = USB initialization
- 3 medium blinks = MIDI ready
- Quick flash = Paddle press detected
- Continuous on = Both paddles pressed
- Brief flash every 10s = Heartbeat (firmware alive)

**When to use**: When you need to verify that the firmware is loaded and running, or troubleshoot communication issues.

## 🔧 Key Improvements Over Original

### Problem with Original
The original `paddle_decoder.ino` may have issues with:
- MIDI timing
- USB enumeration with specific VID/PID
- Debounce handling
- LED feedback on shared USB pin

### What's Fixed
1. **Better Debouncing**: Increased from 10ms to configurable values
2. **Improved MIDI Sending**: Uses both NoteOff and NoteOn(velocity=0) methods
3. **LED Management**: Timed LED pulses instead of direct control
4. **State Tracking**: Better state machines for paddle monitoring
5. **USB Stability**: More frequent DigiMIDI.update() calls
6. **Diagnostic Support**: Multiple versions for troubleshooting

## 🚀 Installation Instructions

### Step 1: Arduino IDE Setup
```bash
# Install Arduino IDE if not already installed
# Add Digispark board support:
# File -> Preferences -> Additional Board Manager URLs
# Add: http://digistump.com/package_digistump_index.json
```

### Step 2: Install Board and Library
```
1. Tools -> Board -> Boards Manager
2. Search for "Digistump AVR Boards"
3. Click Install

4. Sketch -> Include Library -> Manage Libraries  
5. Search for "DigiMIDI"
6. Install DigiMIDI library
```

### Step 3: Select Board
```
Tools -> Board -> Digispark (Default - 16.5mhz)
```

### Step 4: Choose Your Version
- For first attempt: Open `paddle_decoder_vusb.ino`
- If issues persist: Try `paddle_decoder_alt.ino`
- For debugging: Use `paddle_decoder_diagnostic.ino`

### Step 5: Upload
```
1. Sketch -> Upload
2. Wait for "Plug in device now..."
3. Plug in Digispark USB
4. Wait for upload to complete
5. Digispark will reboot automatically
```

## 🔌 Hardware Connections

```
ATtiny85 Digispark          Paddle
─────────────────          ──────
P2 (Pin 2)       ───────>  LEFT contact (Dit)
P0 (Pin 0)       ───────>  RIGHT contact (Dah)
GND              ───────>  Common Ground
P1 (Pin 1)       ───────>  [Optional LED + resistor to GND]

USB Connection   ───────>  Computer USB port
```

### Pin Details
- **P2 (Dit)**: Use internal pullup, LOW when pressed
- **P0 (Dah)**: Use internal pullup, LOW when pressed  
- **P1 (LED)**: Can add external LED for brighter indication
- **P3/P4**: Used by V-USB (don't connect anything)

## 🧪 Testing Steps

### 1. Verify Firmware Upload
After uploading, you should see LED blink pattern:
- **vusb**: 5 fast blinks
- **alt**: 5 fast blinks + pause + 3 medium blinks  
- **diagnostic**: 5 fast + 1 long + 3 medium blinks

### 2. Check USB Enumeration
```bash
# On Linux:
lsusb | grep "16c0:05e4"

# Should show:
Bus 001 Device XXX: ID 16c0:05e4 Van Ooijen Technische Informatica

# Check MIDI device:
amidi -l

# Should show something like:
Dir Device    Name
IO  hw:X,0,0  Digispark MIDI
```

### 3. Test Paddle Input
```bash
# Monitor MIDI messages (Linux):
aseqdump -p "Digispark"

# Press LEFT paddle -> Should see: Note On, note 60
# Release LEFT paddle -> Should see: Note Off, note 60
# Press RIGHT paddle -> Should see: Note On, note 62
# Release RIGHT paddle -> Should see: Note Off, note 62
```

### 4. Run Your Rust Application
```bash
cd /home/developer/rust/paddle_decoder_cross_platform
cargo run --release
```

## 🐛 Troubleshooting

### Issue: Device not recognized
**Symptoms**: `lsusb` doesn't show 16c0:05e4

**Solutions**:
1. Try different USB port
2. Try USB 2.0 port instead of 3.0
3. Check if udev rules are needed (see below)
4. Re-upload firmware
5. Try the diagnostic version to verify firmware is running

### Issue: MIDI device not appearing  
**Symptoms**: `amidi -l` shows no device

**Solutions**:
1. Check kernel modules: `lsmod | grep snd_seq`
2. Load ALSA sequencer: `sudo modprobe snd-seq`
3. Check dmesg for errors: `dmesg | tail -20`
4. Verify VID:PID in `lsusb`

### Issue: Paddle presses not detected
**Symptoms**: LED doesn't flash, no MIDI messages

**Solutions**:
1. Check paddle wiring
2. Test with multimeter (continuity test)
3. Upload diagnostic version
4. Verify pullup resistors are working (measure voltage on P0/P2)

### Issue: Wrong MIDI notes received
**Check**: Make sure Rust app is looking for notes 60 and 62, not 1 and 2

**Note Mapping**:
- Original: Note 1 (Dit), Note 2 (Dah)  
- New versions: Note 60 (Dit), Note 62 (Dah)

### Issue: Erratic behavior
**Solutions**:
1. Increase debounce time in code
2. Add 0.1µF capacitor across paddle contacts
3. Check for electrical noise
4. Use shielded cable for paddle

## 📝 udev Rules (Linux)

If you get permission errors, create udev rule:

```bash
sudo nano /etc/udev/rules.d/49-digispark.rules
```

Add:
```
# Digispark ATtiny85 MIDI
SUBSYSTEMS=="usb", ATTRS{idVendor}=="16c0", ATTRS{idProduct}=="05e4", MODE:="0666"
KERNEL=="midi*", ATTRS{idVendor}=="16c0", ATTRS{idProduct}=="05e4", MODE:="0666"
```

Reload rules:
```bash
sudo udevadm control --reload-rules
sudo udevadm trigger
```

## 🔄 Updating Your Rust Application

The new INO files use **Note 60** and **Note 62** instead of **Note 1** and **Note 2**.

### If your Rust code checks for notes 1 and 2:

**Find** (in `src/main.rs`):
```rust
if message.message == 1 {  // Dit
    // ...
}
if message.message == 2 {  // Dah
    // ...
}
```

**Replace with**:
```rust
if message.message == 60 {  // Dit (Middle C)
    // ...
}
if message.message == 62 {  // Dah (D note)
    // ...
}
```

### Alternative: Keep notes 1 and 2

If you want to keep your Rust app unchanged, edit the INO files:

Change:
```cpp
#define NOTE_DIT  60
#define NOTE_DAH  62
```

To:
```cpp
#define NOTE_DIT  1
#define NOTE_DAH  2
```

## 📊 MIDI Message Format

All versions send standard MIDI messages:

**Note On** (paddle pressed):
- Status: 0x90 (Note On, Channel 1)
- Note: 60 (Dit) or 62 (Dah)
- Velocity: 127 (full)

**Note Off** (paddle released):
- Method 1: Status 0x80, Note, Velocity 0
- Method 2: Status 0x90, Note, Velocity 0  
  (diagnostic version sends both for compatibility)

## 🎯 Which Version Should I Use?

```
Start here → paddle_decoder_vusb.ino
    ↓
    Works? ✅ → Great! You're done.
    ↓
    No? ❌
    ↓
Try → paddle_decoder_alt.ino  
    ↓
    Works? ✅ → Great! You're done.
    ↓
    No? ❌
    ↓
Try → paddle_decoder_diagnostic.ino
    ↓
    Check LED patterns
    ↓
    LED blinks? ✅ → Firmware uploaded OK
        ↓
        → Check USB enumeration (lsusb)
        → Check MIDI device (amidi -l)
        → Check dmesg for errors
    ↓
    No LED? ❌ → Re-upload firmware
        → Try different USB port
        → Check Digispark board
```

## ✅ Success Checklist

- [ ] Arduino IDE installed with Digispark support
- [ ] DigiMIDI library installed  
- [ ] Firmware uploaded successfully
- [ ] LED blinks on startup
- [ ] Device appears in `lsusb` as 16c0:05e4
- [ ] MIDI device appears in `amidi -l`
- [ ] LED flashes when paddle pressed
- [ ] MIDI messages visible in `aseqdump`
- [ ] Rust application receives MIDI events

## 📞 Still Having Issues?

1. **Check LED patterns** - Use diagnostic version
2. **Verify USB enumeration** - Run `lsusb`
3. **Check MIDI subsystem** - Run `amidi -l`
4. **Monitor system logs** - Run `dmesg -w`
5. **Test with aseqdump** - Verify MIDI messages
6. **Check hardware** - Test paddle continuity

## 📚 Additional Resources

- Digispark Wiki: http://digistump.com/wiki/
- V-USB Project: https://www.obdev.at/products/vusb/
- MIDI Specification: https://www.midi.org/specifications
- DigiMIDI Library: https://github.com/digistump/DigisparkArduinoIntegration

## 🛠️ Development Notes

All three versions:
- Use DigiMIDI library for USB MIDI
- Support VID:PID 16c0:05e4
- Include debouncing
- Provide LED feedback
- Send standard MIDI Note On/Off messages
- Are compatible with the Rust decoder application

Choose the version that works best for your setup!

73! 📻
DD6DS
