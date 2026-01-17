// Add morse code player module
use std::collections::HashMap;
use rodio::{Sink, Source};
use std::time::Duration;
use std::thread;

pub struct MorsePlayer {
    morse_map: HashMap<char, &'static str>,
    frequency: f32,
    wpm: u32,
}

impl MorsePlayer {
    pub fn new(frequency: f32, wpm: u32) -> Self {
        let mut morse_map = HashMap::new();
        
        // Letters
        morse_map.insert('A', ".-");
        morse_map.insert('B', "-...");
        morse_map.insert('C', "-.-.");
        morse_map.insert('D', "-..");
        morse_map.insert('E', ".");
        morse_map.insert('F', "..-.");
        morse_map.insert('G', "--.");
        morse_map.insert('H', "....");
        morse_map.insert('I', "..");
        morse_map.insert('J', ".---");
        morse_map.insert('K', "-.-");
        morse_map.insert('L', ".-..");
        morse_map.insert('M', "--");
        morse_map.insert('N', "-.");
        morse_map.insert('O', "---");
        morse_map.insert('P', ".--.");
        morse_map.insert('Q', "--.-");
        morse_map.insert('R', ".-.");
        morse_map.insert('S', "...");
        morse_map.insert('T', "-");
        morse_map.insert('U', "..-");
        morse_map.insert('V', "...-");
        morse_map.insert('W', ".--");
        morse_map.insert('X', "-..-");
        morse_map.insert('Y', "-.--");
        morse_map.insert('Z', "--..");
        
        // Numbers
        morse_map.insert('0', "-----");
        morse_map.insert('1', ".----");
        morse_map.insert('2', "..---");
        morse_map.insert('3', "...--");
        morse_map.insert('4', "....-");
        morse_map.insert('5', ".....");
        morse_map.insert('6', "-....");
        morse_map.insert('7', "--...");
        morse_map.insert('8', "---..");
        morse_map.insert('9', "----.");
        
        // Punctuation
        morse_map.insert('.', ".-.-.-");
        morse_map.insert(',', "--..--");
        morse_map.insert('?', "..--..");
        morse_map.insert('/', "-..-.");
        
        MorsePlayer { 
            morse_map,
            frequency,
            wpm,
        }
    }
    
    pub fn text_to_morse(&self, text: &str) -> Vec<MorseElement> {
        let mut elements = Vec::new();
        
        for ch in text.chars() {
            if ch == ' ' {
                elements.push(MorseElement::WordSpace);
                continue;
            }
            
            if let Some(morse) = self.morse_map.get(&ch.to_ascii_uppercase()) {
                for symbol in morse.chars() {
                    match symbol {
                        '.' => elements.push(MorseElement::Dit),
                        '-' => elements.push(MorseElement::Dah),
                        _ => {}
                    }
                }
                elements.push(MorseElement::LetterSpace);
            }
        }
        
        elements
    }
    
    pub fn play_morse(&self, sink: &Sink, text: &str) {
        let dit_ms = 1200 / self.wpm.max(1);
        let elements = self.text_to_morse(text);
        
        for element in elements {
            match element {
                MorseElement::Dit => {
                    sink.stop();
                    let tone = ToneGen::new(self.frequency, dit_ms);
                    sink.append(tone);
                    sink.play();
                    thread::sleep(Duration::from_millis(dit_ms as u64));
                    sink.stop();
                    thread::sleep(Duration::from_millis(dit_ms as u64));
                }
                MorseElement::Dah => {
                    sink.stop();
                    let tone = ToneGen::new(self.frequency, dit_ms * 3);
                    sink.append(tone);
                    sink.play();
                    thread::sleep(Duration::from_millis((dit_ms * 3) as u64));
                    sink.stop();
                    thread::sleep(Duration::from_millis(dit_ms as u64));
                }
                MorseElement::LetterSpace => {
                    thread::sleep(Duration::from_millis((dit_ms * 2) as u64));
                }
                MorseElement::WordSpace => {
                    thread::sleep(Duration::from_millis((dit_ms * 6) as u64));
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum MorseElement {
    Dit,
    Dah,
    LetterSpace,
    WordSpace,
}

// Simple tone generator for morse playback
struct ToneGen {
    frequency: f32,
    duration_samples: usize,
    sample_rate: u32,
    phase: f32,
    samples_played: usize,
}

impl ToneGen {
    fn new(frequency: f32, duration_ms: u32) -> Self {
        let sample_rate = 48000;
        let duration_samples = (sample_rate as u64 * duration_ms as u64 / 1000) as usize;
        
        ToneGen {
            frequency,
            duration_samples,
            sample_rate,
            phase: 0.0,
            samples_played: 0,
        }
    }
}

impl Iterator for ToneGen {
    type Item = f32;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.samples_played >= self.duration_samples {
            return None;
        }
        
        let sample = (self.phase * 2.0 * std::f32::consts::PI).sin() * 0.3;
        self.phase += self.frequency / self.sample_rate as f32;
        if self.phase >= 1.0 {
            self.phase -= 1.0;
        }
        self.samples_played += 1;
        Some(sample)
    }
}

impl Source for ToneGen {
    fn current_frame_len(&self) -> Option<usize> { 
        Some(self.duration_samples - self.samples_played) 
    }
    fn channels(&self) -> u16 { 1 }
    fn sample_rate(&self) -> u32 { self.sample_rate }
    fn total_duration(&self) -> Option<Duration> { 
        Some(Duration::from_millis(
            (self.duration_samples * 1000 / self.sample_rate as usize) as u64
        ))
    }
}
