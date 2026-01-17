# Interactive Listening Practice Mode - Documentation

## Overview
Added an interactive "Listen and Respond" training mode where the program plays morse code and you try to copy it using your paddle. The system provides immediate feedback and shows the correct answer after 2 failed attempts.

## Features Added

### 1. **Morse Code Player** (`src/morse_player.rs`)
- Converts text to morse code elements (dit, dah, letter space, word space)
- Plays morse code audio using configurable WPM and frequency
- Supports all letters (A-Z), numbers (0-9), and punctuation (. , ? /)
- Uses rodio audio library for sound generation

### 2. **Interactive Practice Workflow**

#### Step 1: Select Training Item
- Enable Training Mode
- Choose session (1-10)
- Choose practice type (Characters, Words, Abbreviations, etc.)

#### Step 2: Choose Practice Mode
Two modes available:
- **📝 Sending Practice**: See text, send it with paddle (original mode)
- **🎧 Listening Practice**: NEW! Hear morse, copy with paddle

#### Step 3: Practice Cycle (Listening Mode)
1. Click "▶ Play Morse Code" to hear the item
2. Send what you heard using your paddle
3. Click "Check Answer" to verify
4. Get immediate feedback:
   - **✓ RIGHT!** in GREEN if correct
   - **✗ FALSE** in RED if wrong

#### Step 4: Learning Assistance
- **First Wrong Answer**: 
  - Shows "✗ FALSE" in red
  - Automatically replays the morse code after 1 second
  - You can try again
  
- **Second Wrong Answer**:
  - Shows "✗ FALSE" in red
  - Displays the correct answer in yellow text
  - You can see what you should have sent

- **Correct Answer**:
  - Shows "✓ RIGHT!" in green
  - Click "Next Item" to continue

## How to Use

### Basic Usage

1. **Start the Application**
```bash
./paddle_decoder_linux_amd64
```

2. **Enable Training Mode**
   - Scroll to "🎓 CW Academy Training Mode" section
   - Click "Enable Training" button

3. **Select Session and Type**
   - Choose your session (1-10) from dropdown
   - Choose practice type from dropdown

4. **Switch to Listening Mode**
   - Click "🎧 Listening Practice" button
   - Mode indicator shows as selected

5. **Practice**
   - Click "▶ Play Morse Code" button
   - Listen carefully to the morse code
   - Send what you heard using your paddle
   - Your decoded text appears in the "Decoded Text" section
   - Click "Check Answer" when ready

6. **Get Feedback**
   - See if you're RIGHT (green) or FALSE (red)
   - If wrong, code replays automatically
   - After 2 mistakes, see the correct answer
   - Click "Next Item" to continue practicing

### Tips for Best Results

1. **Start Slow**
   - Begin with Session 1 (A E N T)
   - Use Characters practice type first
   - Adjust WPM to comfortable speed (5-20 WPM)

2. **Focus on Sound Patterns**
   - Don't count dits and dahs
   - Listen for the whole character as a unique sound
   - Close your eyes while listening

3. **Practice Regularly**
   - Use "Next Item" frequently for variety
   - Mix Characters, Words, and Abbreviations
   - Progress through sessions as you improve

4. **Use Both Modes**
   - Practice Sending Mode to learn hand coordination
   - Practice Listening Mode to train your ear
   - Alternate between modes for complete training

## Technical Details

### Audio Playback
- **Sample Rate**: 48000 Hz
- **Waveform**: Sine wave
- **Volume**: 30% amplitude (0.3)
- **Dit Length**: 1200 / WPM (Paris standard)
- **Dah Length**: 3 × Dit length
- **Element Space**: 1 × Dit length
- **Letter Space**: 3 × Dit length (total)
- **Word Space**: 7 × Dit length (total)

### Timing Calculation
Based on PARIS standard word timing:
- **Dit duration (ms)** = 1200 / WPM
- Example at 20 WPM: 1200 / 20 = 60ms per dit

### Character Support
- **Letters**: A-Z (26 characters)
- **Numbers**: 0-9 (10 characters)
- **Punctuation**: . , ? /
- **Prosigns**: Partial support (can be enhanced)

## UI Elements

### Listening Mode Interface

```
🎧 Listen and Decode

[▶ Play Morse Code]  <- Click to hear morse

Now send what you heard using your paddle:

✓ RIGHT!             <- Green feedback when correct
  or
✗ FALSE              <- Red feedback when wrong

The correct answer was:  <- Shows after 2 mistakes
[ANSWER IN YELLOW]

[Check Answer]       <- Click to verify your input
[Next Item]          <- Click for new practice item
```

### Mode Selection
```
[📝 Sending Practice]    <- Original mode
[🎧 Listening Practice]  <- New interactive mode
```

## Benefits

1. **Immediate Feedback**: Know right away if you're correct
2. **Learning Support**: See answer after struggling
3. **Automatic Replay**: Hear code again when wrong
4. **Structured Learning**: Follow CW Academy curriculum
5. **Visual Feedback**: Clear color-coded results
6. **No Frustration**: Answer revealed after 2 attempts
7. **Real Practice**: Use actual paddle hardware
8. **Progressive Difficulty**: Start easy, advance gradually

## Example Practice Session

### Beginner (Session 1 - Characters)
```
1. Select Session 1
2. Select "Characters"  
3. Enable Listening Mode
4. Click Play -> Hear ".- " (dit-dah)
5. Send: DIT-DAH
6. Check Answer -> ✓ RIGHT! (It was 'A')
7. Next Item -> Hear ".." (dit-dit)
8. Send: DIT-DIT
9. Check Answer -> ✓ RIGHT! (It was 'I')
10. Continue practicing...
```

### Intermediate (Session 5 - Words)
```
1. Select Session 5
2. Select "Words"
3. Enable Listening Mode
4. Click Play -> Hear word in morse
5. Try to send it back
6. First attempt wrong -> Hear it again
7. Second attempt wrong -> See correct answer
8. Learn from mistake
9. Next Item to practice more
```

## Troubleshooting

### Audio Not Playing
- Check system volume
- Verify audio output device
- Try adjusting frequency slider (300-1000 Hz)

### Can't Hear the Difference
- Reduce WPM speed (try 10 WPM)
- Start with Session 1 Characters
- Practice individual characters first
- Use headphones for better clarity

### Paddle Not Responding
- Check MIDI connection
- Verify paddle is powered
- Check "Paddle Status" indicators

### Wrong Answer But Seems Right
- Check for extra spaces
- Verify all characters were sent
- Compare carefully with displayed answer

## Future Enhancements (Optional)

- [ ] Add prosign support (<AR>, <BK>, <SK>)
- [ ] Score tracking and statistics
- [ ] Timed practice sessions
- [ ] Difficulty progression hints
- [ ] Success rate display
- [ ] Session progress tracking
- [ ] Export practice log
- [ ] Audio recording of sessions
- [ ] Adjustable feedback delay
- [ ] Hint system (show first letter)

## Credits

Based on:
- **CW Academy Beginner CW Curriculum** (Rev 4.2.8.1)
- **By**: Ken Rainey, AC5EZ
- **Available at**: https://cwops.org/

---

**73 and good luck with your CW training!** 📻
