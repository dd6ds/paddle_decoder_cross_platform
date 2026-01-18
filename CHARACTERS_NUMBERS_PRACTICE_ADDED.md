# Characters + Numbers Practice Type Added

## Overview
Added a new practice type option "Characters + Numbers" that allows training with a random mix of characters (letters) and numbers from the selected session.

## Changes Made

### 1. New Practice Type Enum Variant
**File**: `src/cw_academy_training.rs`

Added `CharactersAndNumbers` to the `PracticeType` enum:
```rust
pub enum PracticeType {
    Characters,
    CharactersAndNumbers,  // NEW!
    Words,
    Abbreviations,
    Numbers,
    Callsigns,
    Phrases,
}
```

### 2. Display Name
The new practice type shows as "Characters + Numbers" in the UI dropdown.

### 3. Practice Item Generation
The new practice type intelligently combines:
- **Letters**: All letters from the session's character set (A, E, N, T, etc.)
- **Numbers**: Individual digits extracted from the session's number sequences

**Example for Session 2:**
- Characters: A, E, N, T, S, I, O
- Numbers: 1, 4 (extracted from sequences like "1441", "4114")
- **Combined Pool**: A, E, N, T, S, I, O, 1, 4

When practicing, the system randomly selects from this combined pool, giving you:
- Sometimes a letter: "A", "E", "T"
- Sometimes a number: "1", "4"
- Random mix for realistic practice

## Usage

### In Training Window:

1. **Open Training Window**
2. **Select Session** (e.g., Session 2, Session 5, etc.)
3. **Choose Practice Type**: Select "Characters + Numbers"
4. **Start Training Session**
5. Practice with random mix of letters and numbers!

### What You'll Get:

**Session 2 Example:**
- Random selections from: A, E, N, T, S, I, O, 1, 4
- Could be: "A", "4", "T", "1", "O", "N", "4", "E", etc.

**Session 5 Example:**
- Characters include: U, C, M, W
- Numbers include: 2, 3, 5, 6
- Random selections from all of these

**Session 7 Example:**
- Full alphabet coverage with 7, 9 added

## Why This Feature Is Useful

### 1. **Realistic Training**
In real QSOs, you constantly switch between letters and numbers:
- Callsigns: "N2AB", "W3RD", "K4XYZ"
- Signal reports: "599", "RST"
- Frequencies: "7054"
- QTH Grid squares: "FN20"

### 2. **Brain Training**
Switching between letter patterns and number patterns:
- Trains mental agility
- Improves recognition speed
- Reduces hesitation when mode-switching

### 3. **Prevents Pattern Recognition**
- Pure character practice: brain anticipates letters
- Pure number practice: brain expects numbers
- **Mixed practice**: Keeps you alert and engaged!

### 4. **Session-Appropriate**
Only uses characters and numbers taught in that session:
- **Session 1**: A, E, N, T (no numbers yet)
- **Session 2**: A, E, N, T, S, I, O, 1, 4
- **Session 5**: Adds U, C, M, W, 2, 3, 5, 6
- **Session 7**: Adds F, Y, G, P, Q, 7, 9
- And so on...

## Practice Type Selection Order

The practice types now appear in this order:
1. **Characters** - Letters only (traditional)
2. **Characters + Numbers** - Mixed letters and numbers (NEW!)
3. **Words** - Complete words
4. **CW Abbreviations** - Common ham radio abbreviations
5. **Numbers** - Number sequences only
6. **Callsigns** - Amateur radio callsigns
7. **Phrases** - Complete phrases

## Compatibility

- ✅ Works with all 10 sessions
- ✅ Works with Sending Practice mode
- ✅ Works with Listening Practice mode
- ✅ Works with Random Blocks mode
- ✅ Works with statistics tracking
- ✅ Works with timeout features

## Technical Implementation

### Character Extraction
The system:
1. Takes all single-character items from the session's character list
2. Extracts individual digits from number sequences
3. Removes duplicates
4. Creates a combined pool for random selection

### Smart Filtering
Only includes:
- Single alphanumeric characters from character set
- Individual digits (0-9) from number sequences
- No special characters or prosigns in this mode

## Examples

### Session 2 Practice
```
Play: "A" → Send: .-
Play: "4" → Send: ....-
Play: "T" → Send: -
Play: "1" → Send: .----
Play: "N" → Send: -.
Play: "O" → Send: ---
```

### Session 5 Practice
```
Play: "M" → Send: --
Play: "3" → Send: ...--
Play: "W" → Send: .--
Play: "5" → Send: .....
Play: "C" → Send: -.-.
Play: "6" → Send: -....
```

## Pro Tips

1. **Start with Session 2**: First session with numbers (1, 4)
2. **Progress Gradually**: Master each session before advancing
3. **Use Listening Mode**: Most realistic for mixed character practice
4. **Enable Timeout**: Adds pressure, simulates real QSO timing
5. **Track Statistics**: Monitor your character vs. number accuracy

## Future Enhancement Ideas

Possible additions:
- Weight ratio (70% letters, 30% numbers)
- Difficulty levels (predictable vs. totally random)
- Focus mode (practice only missed characters)
- Group mode (2-3 character groups with mixed types)
