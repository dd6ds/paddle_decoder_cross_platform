# Training Window Feature Added

## Overview
Added a separate, dedicated window for CW training features in the paddle decoder application.

## Changes Made

### 1. AppState Structure
- Added `show_training_window: bool` field to track training window visibility
- Default value: `false`

### 2. Main Window Updates
- Added a prominent "🎓 Open Training Window" button in the main panel
- Button located after the "Timing" section
- Clicking the button opens the training window and enables training mode
- Removed all training UI from the main window to avoid duplication

### 3. New Training Window
- Created `render_training_window()` method containing all training functionality
- Window features:
  - Title: "🎓 CW Training"
  - Default size: 700x800 pixels
  - Resizable and closable
  - Separate from main paddle decoder interface

### 4. Training Features (in separate window)
All existing training features are now in the dedicated window:
- Enable/Disable Training toggle
- Session selection (Session 1-26)
- Practice type selection (Characters, Words, Q-Codes, Mixed)
- Random Blocks Mode with configurable settings:
  - Session range (From/To)
  - Block size options
- Training mode selection:
  - 📝 Sending Practice: Shows text to send with paddle
  - 🎧 Listening Practice: Play morse and decode
- Statistics tracking (Correct/Wrong/Accuracy)
- Timeout configuration
- Answer checking and feedback

## Benefits

1. **Cleaner Main Interface**: Main window focuses on core paddle decoder functionality
2. **Dedicated Training Space**: Training gets its own window with more room
3. **Better Organization**: Training features are grouped together
4. **Flexible Workflow**: Users can have both windows open or close the training window when not needed
5. **Easier Navigation**: No need to scroll through training settings in the main window

## Usage

1. Launch the paddle decoder application
2. Click the "🎓 Open Training Window" button in the main window
3. The training window will open with all training features
4. Configure training settings in the separate window
5. Close the training window when done (click X or close button)
6. Reopen anytime by clicking the button again

## Technical Details

- Uses egui::Window for the separate training window
- Shares the same AppState between windows
- All training logic remains unchanged, just reorganized into a separate UI
- Window state (open/closed) is tracked in AppState

## Next Steps (Optional Enhancements)

Future improvements could include:
- Save window position/size preferences
- Keyboard shortcut to toggle training window
- Multiple training windows for different exercises
- Detachable training panels
