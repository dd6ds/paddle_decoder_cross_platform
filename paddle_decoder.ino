// ATtiny85 Paddle Interface for Real-Time Morse Decoder
// Sends MIDI messages when paddle contacts are made/broken
// Compatible with Digispark ATtiny85 boards
//
// Connections:
// - P2 (Pin 2): LEFT paddle (Dit) - generates dots
// - P0 (Pin 0): RIGHT paddle (Dah) - generates dashes  
// - P1 (Pin 1): LED indicator
// - Ground: Common ground for both paddle contacts
//
// MIDI Output:
// - LEFT paddle (P2):  Note 1 On/Off
// - RIGHT paddle (P0): Note 2 On/Off
//
// Installation:
// 1. Install Digispark board support in Arduino IDE
// 2. Install DigiMIDI library from Library Manager
// 3. Select "Digispark (Default - 16.5mhz)" as board
// 4. Upload with bootloader (plug in USB when prompted)

#include <DigiMIDI.h>

// Pin definitions
#define paddleLeft   2    // P2 - LEFT paddle (Dit)
#define paddleRight  0    // P0 - RIGHT paddle (Dah)
#define LED          1    // P1 - Status LED

// State tracking
bool lastLeftState = false;
bool lastRightState = false;

void setup() {
  // Configure pins
  pinMode(paddleLeft, INPUT_PULLUP);
  pinMode(paddleRight, INPUT_PULLUP); 
  pinMode(LED, OUTPUT);
  
  // Startup indicator: 5 fast blinks
  for (int i = 0; i < 5; i++) {
    digitalWrite(LED, HIGH);
    DigiMIDI.delay(80);
    digitalWrite(LED, LOW);
    DigiMIDI.delay(80);
  }
  
  // Pause before starting
  DigiMIDI.delay(500);
}

void loop() {
  // Required for MIDI USB communication
  DigiMIDI.update();
  
  // Read paddle states
  // Note: INPUT_PULLUP means LOW = pressed, HIGH = not pressed
  bool leftPressed = !digitalRead(paddleLeft);   // Invert for true state
  bool rightPressed = !digitalRead(paddleRight); // Invert for true state
  
  // Handle LEFT paddle (Dit) state changes
  if (leftPressed != lastLeftState) {
    if (leftPressed) {
      // LEFT pressed - send Note 1 ON
      DigiMIDI.sendNoteOn(1, 100, 1);  // Note 1, velocity 100, channel 1
      digitalWrite(LED, HIGH);
    } else {
      // LEFT released - send Note 1 OFF
      DigiMIDI.sendNoteOn(1, 0, 1);    // Note 1, velocity 0 = Note Off
      digitalWrite(LED, LOW);
    }
    lastLeftState = leftPressed;
    DigiMIDI.delay(10);  // Small debounce delay
  }
  
  // Handle RIGHT paddle (Dah) state changes
  if (rightPressed != lastRightState) {
    if (rightPressed) {
      // RIGHT pressed - send Note 2 ON
      DigiMIDI.sendNoteOn(2, 100, 1);  // Note 2, velocity 100, channel 1
      digitalWrite(LED, HIGH);
    } else {
      // RIGHT released - send Note 2 OFF
      DigiMIDI.sendNoteOn(2, 0, 1);    // Note 2, velocity 0 = Note Off
      digitalWrite(LED, LOW);
    }
    lastRightState = rightPressed;
    DigiMIDI.delay(10);  // Small debounce delay
  }
  
  // Small delay for stability
  DigiMIDI.delay(1);
}
