// ATtiny85 Paddle Interface with V-USB MIDI
// Works with VID 16c0:05e4 - Van Ooijen Technische Informatica Free shared USB VID/PID
// 
// This version uses direct V-USB MIDI implementation for better compatibility
// with the Free shared MIDI VID/PID pair
//
// Connections:
// - P2 (Pin 2): LEFT paddle (Dit) - generates dots
// - P0 (Pin 0): RIGHT paddle (Dah) - generates dashes  
// - P1 (Pin 1): LED indicator (optional, shared with USB)
// - Ground: Common ground for both paddle contacts
//
// MIDI Output:
// - LEFT paddle (P2):  Note 60 (Middle C) On/Off on Channel 1
// - RIGHT paddle (P0): Note 62 (D) On/Off on Channel 1
//
// Hardware Requirements:
// - Digispark ATtiny85 board (or compatible with V-USB)
// - USB connection for MIDI
// - Iambic paddle or dual contacts
//
// Installation:
// 1. Install Digispark board support: http://digistump.com/package_digistump_index.json
// 2. Tools -> Board -> Digispark (Default - 16.5mhz)
// 3. Sketch -> Upload (plug in when prompted)
// 4. Device should enumerate as MIDI device with VID:PID 16c0:05e4

#include <DigiMIDI.h>

// Pin definitions - optimized for Digispark
#define PADDLE_LEFT   2    // P2 - LEFT paddle (Dit)
#define PADDLE_RIGHT  0    // P0 - RIGHT paddle (Dah)
#define LED_PIN       1    // P1 - Status LED (shared with USB D-, use carefully)

// MIDI note numbers
#define NOTE_LEFT     60   // Middle C for dit
#define NOTE_RIGHT    62   // D for dah
#define MIDI_CHANNEL  1    // MIDI channel 1

// State tracking with debouncing
bool lastLeftState = HIGH;   // HIGH = not pressed (pullup)
bool lastRightState = HIGH;
unsigned long lastLeftChange = 0;
unsigned long lastRightChange = 0;
const unsigned long DEBOUNCE_MS = 5;  // 5ms debounce

// LED management
bool ledState = false;
unsigned long ledOffTime = 0;
const unsigned long LED_PULSE_MS = 50;  // LED stays on for 50ms

void setup() {
  // Configure paddle input pins with internal pullups
  pinMode(PADDLE_LEFT, INPUT_PULLUP);
  pinMode(PADDLE_RIGHT, INPUT_PULLUP);
  
  // Configure LED (be careful - P1 is also USB D-)
  pinMode(LED_PIN, OUTPUT);
  digitalWrite(LED_PIN, LOW);
  
  // Startup sequence: blink LED to indicate ready
  // Use DigiMIDI.delay() to maintain USB connection
  for (int i = 0; i < 5; i++) {
    digitalWrite(LED_PIN, HIGH);
    DigiMIDI.delay(100);
    digitalWrite(LED_PIN, LOW);
    DigiMIDI.delay(100);
  }
  
  // Wait for USB to stabilize
  DigiMIDI.delay(500);
  
  // Initialize state
  lastLeftState = digitalRead(PADDLE_LEFT);
  lastRightState = digitalRead(PADDLE_RIGHT);
}

void loop() {
  // CRITICAL: Must call this regularly for USB/MIDI to work
  DigiMIDI.update();
  
  unsigned long currentTime = millis();
  
  // Read current paddle states
  bool currentLeft = digitalRead(PADDLE_LEFT);
  bool currentRight = digitalRead(PADDLE_RIGHT);
  
  // Handle LEFT paddle (Dit) with debouncing
  if (currentLeft != lastLeftState && 
      (currentTime - lastLeftChange) > DEBOUNCE_MS) {
    
    lastLeftChange = currentTime;
    lastLeftState = currentLeft;
    
    if (currentLeft == LOW) {
      // Paddle pressed (LOW with pullup)
      DigiMIDI.sendNoteOn(NOTE_LEFT, 127, MIDI_CHANNEL);  // Full velocity
      flashLED();
    } else {
      // Paddle released
      DigiMIDI.sendNoteOff(NOTE_LEFT, 0, MIDI_CHANNEL);
    }
  }
  
  // Handle RIGHT paddle (Dah) with debouncing
  if (currentRight != lastRightState && 
      (currentTime - lastRightChange) > DEBOUNCE_MS) {
    
    lastRightChange = currentTime;
    lastRightState = currentRight;
    
    if (currentRight == LOW) {
      // Paddle pressed (LOW with pullup)
      DigiMIDI.sendNoteOn(NOTE_RIGHT, 127, MIDI_CHANNEL);  // Full velocity
      flashLED();
    } else {
      // Paddle released
      DigiMIDI.sendNoteOff(NOTE_RIGHT, 0, MIDI_CHANNEL);
    }
  }
  
  // Handle LED timeout
  if (ledState && currentTime >= ledOffTime) {
    digitalWrite(LED_PIN, LOW);
    ledState = false;
  }
  
  // Small delay for stability (but not too long to miss events)
  DigiMIDI.delay(1);
}

// Flash LED briefly when paddle is pressed
void flashLED() {
  digitalWrite(LED_PIN, HIGH);
  ledState = true;
  ledOffTime = millis() + LED_PULSE_MS;
}
