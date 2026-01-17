# 🔄 Updating Rust Code for New MIDI Notes

## 📋 Overview

The new INO files use **MIDI note numbers 60 and 62** instead of **1 and 2**.

| Paddle | Old Note | New Note | MIDI Name |
|--------|----------|----------|-----------|
| LEFT (Dit) | 1 | 60 | Middle C |
| RIGHT (Dah) | 2 | 62 | D |

## 🛠️ Quick Update (Using sed)

```bash
cd /home/developer/rust/paddle_decoder_cross_platform

# Backup first!
cp src/main.rs src/main.rs.backup

# Update note references
sed -i 's/message == 1/message == 60/g' src/main.rs
sed -i 's/message == 2/message == 62/g' src/main.rs
sed -i 's/note == 1/note == 60/g' src/main.rs
sed -i 's/note == 2/note == 62/g' src/main.rs

# Test
cargo check
```

## 📝 Manual Update Examples

### Pattern 1: If statements
