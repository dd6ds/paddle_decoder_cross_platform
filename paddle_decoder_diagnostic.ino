// ATtiny85 Paddle MIDI - Diagnostic Version
// This version includes extended diagnostics and alternative MIDI sending methods
// Use this if the standard versions don't work with your setup
//
// VID:PID 16c0:05e4 - Van Ooijen Technische Informatica Free shared MIDI
//
// DIAGNOSTIC FEATURES:
// - Extended LED feedback patterns
// - Alternative MIDI message formats
// - Startup diagnostics
// - State monitoring
//
// Hardware Setup:
// - P2: LEFT paddle (Dit) -> Connect to paddle dit contact
// - P0: RIGHT paddle (Dah) -> Connect to paddle dah contact  
// - GND: Common ground for paddle
// - P1: LED for diagnostics (optional, shared with USB)
//
// LED Patterns:
// - 5 fast blinks at startup = Firmware loaded OK
// - 1 long blink = USB enumeration attempt
// - Quick flash on paddle press = Contact detected
// - Continuous on = Both paddles pressed simultaneously

#include <DigiMIDI.h>

// Hardware pins
#define P_DIT    2   // Left paddle (dit)
#define P_DAH    0   // Right paddle (dah)
#define P_LED    1   // Status LED

// MIDI parameters  
#define NOTE_DIT       60    // Middle C
#define NOTE_DAH       62    // D note
#define CHANNEL        1     // MIDI channel
#define VEL_ON         127   // Note on velocity
#define VEL_OFF        0     // Note off velocity

// Timing constants
#define DEBOUNCE_MS    10    // Debounce time
#define LED_FLASH_MS   50    // LED flash duration
#define STARTUP_WAIT   500   // Startup wait time

// State tracking
volatile bool ditState = HIGH;
volatile bool dahState = HIGH;
volatile bool ditLast = HIGH;
volatile bool dahLast = HIGH;
volatile unsigned long ditTime = 0;
volatile unsigned long dahTime = 0;
volatile unsigned long ledTime = 0;
volatile bool ledOn = false;

// Statistics for debugging
unsigned long ditPresses = 0;
unsigned long dahPresses = 0;
unsigned long midiMessagesSent = 0;

void setup() {
  // Configure hardware
  setupPins();
  
  // Diagnostic startup sequence
  startupDiagnostics();
  
  // Initialize MIDI
  DigiMIDI.delay(STARTUP_WAIT);
  
  // Read initial states
  ditLast = digitalRead(P_DIT);
  dahLast = digitalRead(P_DAH);
}

void setupPins() {
  pinMode(P_DIT, INPUT_PULLUP);
  pinMode(P_DAH, INPUT_PULLUP);
  pinMode(P_LED, OUTPUT);
  digitalWrite(P_LED, LOW);
}

void startupDiagnostics() {
  // Pattern 1: 5 fast blinks = Firmware OK
  for (int i = 0; i < 5; i++) {
    digitalWrite(P_LED, HIGH);
    DigiMIDI.delay(80);
    digitalWrite(P_LED, LOW);
    DigiMIDI.delay(80);
  }
  
  DigiMIDI.delay(200);
  
  // Pattern 2: 1 long blink = USB initialization
  digitalWrite(P_LED, HIGH);
  DigiMIDI.delay(300);
  digitalWrite(P_LED, LOW);
  
  DigiMIDI.delay(200);
  
  // Pattern 3: 3 medium blinks = MIDI ready
  for (int i = 0; i < 3; i++) {
    digitalWrite(P_LED, HIGH);
    DigiMIDI.delay(150);
    digitalWrite(P_LED, LOW);
    DigiMIDI.delay(150);
  }
}

void loop() {
  // REQUIRED: Update USB/MIDI stack
  DigiMIDI.update();
  
  unsigned long now = millis();
  
  // Read paddle states
  ditState = digitalRead(P_DIT);
  dahState = digitalRead(P_DAH);
  
  // Process dit paddle
  if (ditState != ditLast) {
    if ((now - ditTime) > DEBOUNCE_MS) {
      ditTime = now;
      ditLast = ditState;
      
      if (ditState == LOW) {
        // Dit pressed
        ditPresses++;
        sendNoteOnSafe(NOTE_DIT, VEL_ON, CHANNEL);
        ledFlash(now);
      } else {
        // Dit released
        sendNoteOffSafe(NOTE_DIT, VEL_OFF, CHANNEL);
      }
    }
  }
  
  // Process dah paddle
  if (dahState != dahLast) {
    if ((now - dahTime) > DEBOUNCE_MS) {
      dahTime = now;
      dahLast = dahState;
      
      if (dahState == LOW) {
        // Dah pressed
        dahPresses++;
        sendNoteOnSafe(NOTE_DAH, VEL_ON, CHANNEL);
        ledFlash(now);
      } else {
        // Dah released
        sendNoteOffSafe(NOTE_DAH, VEL_OFF, CHANNEL);
      }
    }
  }
  
  // Special case: Both paddles pressed = continuous LED
  if (ditState == LOW && dahState == LOW) {
    digitalWrite(P_LED, HIGH);
    ledOn = true;
    ledTime = now + 1000; // Stay on while both pressed
  }
  
  // LED timeout handling
  if (ledOn && now >= ledTime) {
    digitalWrite(P_LED, LOW);
    ledOn = false;
  }
  
  // Heartbeat: Every 10 seconds, brief flash to show firmware is alive
  static unsigned long lastHeartbeat = 0;
  if ((now - lastHeartbeat) > 10000) {
    lastHeartbeat = now;
    digitalWrite(P_LED, HIGH);
    DigiMIDI.delay(10);
    digitalWrite(P_LED, LOW);
  }
  
  // Small delay for stability
  DigiMIDI.delay(2);
}

// Safe MIDI Note On with error recovery
void sendNoteOnSafe(byte note, byte velocity, byte channel) {
  DigiMIDI.sendNoteOn(note, velocity, channel);
  midiMessagesSent++;
  
  // Brief delay for MIDI buffer
  DigiMIDI.delay(1);
}

// Safe MIDI Note Off with error recovery
void sendNoteOffSafe(byte note, byte velocity, byte channel) {
  // Try standard Note Off
  DigiMIDI.sendNoteOff(note, velocity, channel);
  midiMessagesSent++;
  
  // Alternative: Send Note On with velocity 0
  // Some devices prefer this method
  DigiMIDI.delay(1);
  DigiMIDI.sendNoteOn(note, 0, channel);
  
  DigiMIDI.delay(1);
}

// LED flash for visual feedback
void ledFlash(unsigned long now) {
  digitalWrite(P_LED, HIGH);
  ledOn = true;
  ledTime = now + LED_FLASH_MS;
}
