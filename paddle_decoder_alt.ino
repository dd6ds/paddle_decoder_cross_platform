// ATtiny85 Paddle MIDI Interface - Alternative V-USB Implementation
// Direct V-USB MIDI for maximum compatibility with 16c0:05e4 VID/PID
//
// This version uses lower-level V-USB implementation for better control
// over USB MIDI enumeration and data transfer
//
// Connections:
// - P2 (Pin 2): LEFT paddle (Dit)
// - P0 (Pin 0): RIGHT paddle (Dah)
// - Ground: Common paddle ground
//
// USB Pins (handled by V-USB automatically):
// - P3 (Pin 3): USB D+ (normally with 3.6V Zener to GND)
// - P4 (Pin 4): USB D- (normally with 68Ω resistor to P3)
//
// REQUIREMENTS:
// - Digispark board with V-USB bootloader
// - Arduino IDE with Digispark board support
// - V-USB MIDI library
//
// Board Settings:
// - Board: "Digispark (Default - 16.5mhz)"
// - Upload method: via bootloader

#include <DigiMIDI.h>

// Pin Configuration
const byte PIN_LEFT_PADDLE = 2;   // P2 - Dit (dots)
const byte PIN_RIGHT_PADDLE = 0;  // P0 - Dah (dashes)
const byte PIN_STATUS_LED = 1;    // P1 - Visual feedback

// MIDI Configuration
const byte MIDI_NOTE_DIT = 60;    // Middle C for dit
const byte MIDI_NOTE_DAH = 62;    // D for dah
const byte MIDI_CHANNEL = 1;      // Use channel 1
const byte VELOCITY_ON = 127;     // Full velocity
const byte VELOCITY_OFF = 0;      // Note off

// Timing Configuration
const unsigned int DEBOUNCE_TIME_MS = 8;     // Debounce delay
const unsigned int LED_DURATION_MS = 40;     // LED flash duration
const unsigned int POLL_INTERVAL_MS = 2;     // Main loop polling

// State Variables
struct PaddleState {
  bool current;
  bool previous;
  unsigned long lastChangeTime;
};

PaddleState leftPaddle = {HIGH, HIGH, 0};
PaddleState rightPaddle = {HIGH, HIGH, 0};

bool ledActive = false;
unsigned long ledOffTime = 0;

void setup() {
  // Initialize paddle inputs with internal pull-up resistors
  // HIGH = open (not pressed), LOW = closed (pressed)
  pinMode(PIN_LEFT_PADDLE, INPUT_PULLUP);
  pinMode(PIN_RIGHT_PADDLE, INPUT_PULLUP);
  
  // Initialize LED output
  pinMode(PIN_STATUS_LED, OUTPUT);
  digitalWrite(PIN_STATUS_LED, LOW);
  
  // Startup indication sequence
  // Blink 5 times to show firmware is loaded and running
  performStartupSequence();
  
  // Allow USB enumeration to complete
  DigiMIDI.delay(300);
  
  // Read initial paddle states
  leftPaddle.current = digitalRead(PIN_LEFT_PADDLE);
  leftPaddle.previous = leftPaddle.current;
  rightPaddle.current = digitalRead(PIN_RIGHT_PADDLE);
  rightPaddle.previous = rightPaddle.current;
}

void loop() {
  // CRITICAL: Update USB/MIDI stack
  // This must be called frequently (every few ms) for reliable operation
  DigiMIDI.update();
  
  // Get current time for debouncing and LED timing
  unsigned long now = millis();
  
  // Process left paddle (dit)
  processPaddle(PIN_LEFT_PADDLE, leftPaddle, MIDI_NOTE_DIT, now);
  
  // Process right paddle (dah)
  processPaddle(PIN_RIGHT_PADDLE, rightPaddle, MIDI_NOTE_DAH, now);
  
  // Update LED state
  updateLED(now);
  
  // Brief delay to prevent CPU thrashing while maintaining responsiveness
  DigiMIDI.delay(POLL_INTERVAL_MS);
}

// Process paddle input with debouncing and MIDI output
void processPaddle(byte pin, PaddleState &state, byte midiNote, unsigned long now) {
  // Read current paddle state
  state.current = digitalRead(pin);
  
  // Check if state has changed
  if (state.current != state.previous) {
    // Check if enough time has passed for debouncing
    if ((now - state.lastChangeTime) >= DEBOUNCE_TIME_MS) {
      // Update state
      state.previous = state.current;
      state.lastChangeTime = now;
      
      // Handle paddle press/release
      if (state.current == LOW) {
        // Paddle pressed (closed contact)
        sendMIDINoteOn(midiNote);
        activateLED(now);
      } else {
        // Paddle released (open contact)
        sendMIDINoteOff(midiNote);
      }
    }
  }
}

// Send MIDI Note On message
void sendMIDINoteOn(byte note) {
  DigiMIDI.sendNoteOn(note, VELOCITY_ON, MIDI_CHANNEL);
}

// Send MIDI Note Off message
void sendMIDINoteOff(byte note) {
  DigiMIDI.sendNoteOff(note, VELOCITY_OFF, MIDI_CHANNEL);
}

// Activate LED for visual feedback
void activateLED(unsigned long now) {
  digitalWrite(PIN_STATUS_LED, HIGH);
  ledActive = true;
  ledOffTime = now + LED_DURATION_MS;
}

// Update LED state based on timer
void updateLED(unsigned long now) {
  if (ledActive && (now >= ledOffTime)) {
    digitalWrite(PIN_STATUS_LED, LOW);
    ledActive = false;
  }
}

// Startup sequence - visual confirmation that firmware is running
void performStartupSequence() {
  const int blinkCount = 5;
  const int blinkOnTime = 80;
  const int blinkOffTime = 80;
  
  for (int i = 0; i < blinkCount; i++) {
    digitalWrite(PIN_STATUS_LED, HIGH);
    DigiMIDI.delay(blinkOnTime);
    digitalWrite(PIN_STATUS_LED, LOW);
    DigiMIDI.delay(blinkOffTime);
  }
  
  // Final pause before main loop
  DigiMIDI.delay(200);
}
