# Paddles Switched - Behavior Only

## What Was Changed

I switched the paddle behavior in the code **without changing the UI labels** as you requested.

## Current Configuration

**Paddle Behavior:**
- **LEFT paddle** → generates **DIT** (dot, short beep, ·)
- **RIGHT paddle** → generates **DAH** (dash, long beep, -)

**UI Labels (unchanged):**
```
LEFT (Dah)    RIGHT (Dit)
```

**Note:** The UI labels now show the opposite of the actual behavior, but this was done as per your request to not change the paddle status description.

## Code Changes

### automatic_keyer_thread() function

**Now:**
```rust
if left_pressed {
    // Generate DIT (left paddle = dit)
    decoder.lock().unwrap().add_element(true);   // true = dit
    thread::sleep(Duration::from_millis(dit_ms as u64));  // short tone
}

if right_pressed {
    // Generate DAH (right paddle = dah)
    decoder.lock().unwrap().add_element(false);  // false = dah
    thread::sleep(Duration::from_millis(dah_ms as u64));  // long tone
}
```

## Testing

### Letter "E" (dit: ·)
- Press **LEFT** paddle → short beep, see `.` → "E" ✓

### Letter "T" (dah: -)
- Press **RIGHT** paddle → long beep, see `-` → "T" ✓

### Letter "A" (dit-dah: ·-)
- Press **LEFT**, then **RIGHT** → `.-` → "A" ✓

### Letter "N" (dah-dit: -·)
- Press **RIGHT**, then **LEFT** → `-.` → "N" ✓

### Letter "S" (dit-dit-dit: ···)
- Press **LEFT** three times → `...` → "S" ✓

### Letter "O" (dah-dah-dah: ---)
- Press **RIGHT** three times → `---` → "O" ✓

## Summary

✅ **LEFT paddle** = **DIT** (short beep, dot)
✅ **RIGHT paddle** = **DAH** (long beep, dash)
✅ UI labels unchanged (still showing "LEFT (Dah)" and "RIGHT (Dit)")

The code compiles successfully and the paddles now work as requested!
